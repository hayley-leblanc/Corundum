use crate::alloc::MemPool;
use crate::ll::*;
use crate::utils::*;
use std::marker::PhantomData;
use std::mem;

#[cfg(feature = "verbose")]
use term_painter::Color::*;

#[cfg(feature = "verbose")]
use term_painter::ToStyle;

#[repr(transparent)]
#[derive(Clone, Debug)]
/// Buddy memory block
/// Each memory block has some meta-data information in form of `Buddy` data
/// structure. It has a pointer to the next buddy block, if there is any.
struct Buddy {
    /// Next pointer offset
    /// We assume that usize::MAX is NULL
    next: u64,
}

impl Default for Buddy {
    fn default() -> Self {
        Self { next: u64::MAX }
    }
}

#[inline]
fn is_none(p: u64) -> bool {
    p == u64::MAX
}

#[inline]
fn off_to_option(p: u64) -> Option<u64> {
    if is_none(p) {
        None
    } else {
        Some(p)
    }
}

#[inline]
fn option_to_pptr(p: Option<u64>) -> u64 {
    if let Some(p) = p {
        p
    } else {
        u64::MAX
    }
}

#[repr(C)]
/// Buddy Allocation Algorithm
///
/// It contains 61 free-lists of available buddy blocks to keep at most `2^64`
/// bytes including meta-data information. A free-list `k` keeps all available
/// memory blocks of size `2^k` bytes. Assuming that `Buddy` has a size of 
/// 8 bytes, the shape of lists can be like this:
///
/// ```text
///    [8]: [8] -> [8]
///   [16]: [8|8] -> [8|8]
///   [32]: [8|24] -> [8|24] -> [8|24]
///   [64]: [8|56]
///   ...
/// ```
///
/// The first 8 bytes of each free block is meta-data. Once they are selected
/// for occupation, this 8 byte is going to be used, too. So, the smallest block
/// size is 8 bytes.
pub struct BuddyAlg<A: MemPool> {
    /// Lists of free blocks
    buddies: [u64; 64],

    /// The index of the last buddy list
    last_idx: usize,

    /// Total available space in bytes
    available: usize,

    /// The device size in bytes
    size: usize,

    /// An axillary ring list for allocation and recovery
    aux: Ring<(u64, u64), 128>,

    /// Low-level 64-bit logs for allocation and recovery
    log64: Ring<(u64, u64), 8>,

    /// Low-level `DropOnFailure` logs for recovery
    drop_log: Ring<(u64,usize), 8>,

    /// Indicates that it is draining `aux`
    aux_valid: bool,

    /// Log of available space
    available_log: usize,

    #[cfg(feature = "capture_footprint")]
    /// The footprint of memory usage in bytes
    foot_print: usize,

    #[cfg(feature = "pthread")]
    /// A mutex for atomic operations
    mutex: (libc::pthread_mutex_t, libc::pthread_mutexattr_t),

    #[cfg(not(feature = "pthread"))]
    /// A mutex for atomic operations
    mutex: u8,

    // Marker
    phantom: PhantomData<A>,
}

#[inline]
const fn num_bits<T>() -> u32 {
    (mem::size_of::<T>() << 3) as u32
}

#[inline]
pub fn get_idx(x: usize) -> usize {
    if x == 0 {
        usize::MAX
    } else {
        let x = usize::max(x, mem::size_of::<Buddy>());
        (num_bits::<usize>() - (x - 1).leading_zeros()) as usize
    }
}

impl<A: MemPool> BuddyAlg<A> {
    /// Pool Initialization with a given device size
    pub fn init(&mut self, size: usize) {
        let mut idx = get_idx(size);
        if 1 << idx > size {
            idx -= 1;
        }
        self.buddies = [u64::MAX; 64];
        self.size = 1 << idx;
        self.available = self.size;
        self.buddies[idx] = 0;
        self.last_idx = idx;
        self.log64.clear();
        self.drop_log.clear();
        self.aux.clear();

        #[cfg(feature = "pthread")] unsafe {
        crate::sync::init_lock(&mut self.mutex.0, &mut self.mutex.1);
        }

        #[cfg(not(feature = "pthread"))] {
        self.mutex = 0; }
    }

    #[inline]
    #[track_caller]
    fn buddy<'a>(off: u64) -> &'a mut Buddy {
        debug_assert!(off < u64::MAX - A::start(), "off({}) out of range", off);
        debug_assert!(off + A::start() < A::end(), "off({}) out of range", off);
        union U<'a> {
            off: u64,
            raw: &'a mut Buddy,
        }
        unsafe {
            U {
                off: A::start() + off,
            }.raw
        }
    }

    #[inline]
    fn byte<'a>(off: u64) -> &'a mut u8 {
        union U<'a> {
            off: u64,
            raw: &'a mut u8,
        }
        unsafe {
            U {
                off: A::start() + off,
            }.raw
        }
    }

    #[inline]
    fn lock(&mut self) {
        unsafe { 
            #[cfg(feature = "pthread")]
            libc::pthread_mutex_lock(&mut self.mutex.0); 

            #[cfg(not(feature = "pthread"))] {
                let tid = thread::current().id().get().unwrap();
                while std::intrinsics::atomic_cxchg_acqrel(&mut self.mutex, 0, tid).0 != tid {}
            }
        }
    }

    #[inline]
    fn unlock(&mut self) {
        unsafe { 
            #[cfg(feature = "pthread")]
            libc::pthread_mutex_unlock(&mut self.mutex.0); 

            #[cfg(not(feature = "pthread"))]
            std::intrinsics::atomic_store_rel(&mut self.mutex, 0);
        }
    }

    #[inline]
    /// Adds a new low-level 64-bit log entry
    pub unsafe fn log(&mut self, off: u64, data: u64) {
        self.log64.push((off, data));
    }

    #[inline]
    /// Adds a new low-level `DropOnFailure` log entry
    pub unsafe fn drop_on_failure(&mut self, off: u64, len: usize) {
        self.drop_log.push((off, len));
    }

    #[inline]
    /// Adds a new entry to the auxiliary list of changes
    pub unsafe fn aux_push(&mut self, off: u64, data: u64) {
        self.aux.push((off, data));
    }

    #[inline]
    /// Drain the auxiliary list of changes
    /// 
    /// The functions [`alloc_impl`] and [`dealloc_impl`] fills up the auxiliary
    /// buffer with the required changes to the free lists. Then, they call this
    /// function to materialize the changes. The changes are not valid until
    /// `drain_aux()` is called. The recovery procedure performs changes if they
    /// are valid. Otherwise, it discards them.
    /// 
    /// [`alloc_impl`]: #method.alloc_impl
    /// [`dealloc_impl`]: #method.dealloc_impl
    pub fn drain_aux(&mut self) {
        self.aux.sync_all();
        self.log64.sync_all();
        self.aux_valid = true;
        self.aux.foreach(|(off, next)| {
            let n = Self::buddy(off);
            n.next = next;
        });
        self.aux.clear();
        self.log64.foreach(|(off, data)| {
            let n = Self::buddy(off);
            n.next = data;
        });
        self.log64.clear();
        self.available = self.available_log;
        sfence();
        self.aux_valid = false;
    }

    #[inline]
    /// Discards the changes in the auxiliary buffer
    pub fn discard(&mut self) {
        self.aux.clear();
        self.log64.clear();
    }

    #[inline]
    fn get_off(b: &u64) -> u64 {
        let off = b as *const _ as u64;
        off - A::start()
    }

    #[inline]
    unsafe fn find_free_memory(&mut self, idx: usize, split: bool) -> Option<u64> {
        if idx > self.last_idx {
            // TODO: Check if there are enough free adjacent blocks to add up
            //       to the requested size.
            None
        } else {
            let res;
            if let Some(b) = off_to_option(self.buddies[idx]) {
                // Remove the available block and return it
                let buddy = Self::buddy(b);
                self.aux_push(Self::get_off(&self.buddies[idx]), buddy.next);
                res = b;
            } else {
                res = self.find_free_memory(idx + 1, true)?;
            }
            if idx > 0 && split {
                let next = res + (1 << (idx - 1));
                let mut curr = self.buddies[idx - 1];
                let mut prev: Option<u64> = None;

                while let Some(b) = off_to_option(curr) {
                    if b > next {
                        break;
                    }
                    prev = Some(b);
                    curr = Self::buddy(b).next;
                }

                if let Some(p) = prev {
                    self.aux_push(next, Self::buddy(p).next);
                    self.aux_push(p, next);
                } else {
                    self.aux_push(next, self.buddies[idx - 1]);
                    self.aux_push(Self::get_off(&self.buddies[idx - 1]), next);
                }
            }
            Some(res)
        }
    }

    #[inline]
    /// Generates required changes to the metadata for allocating a new memory
    /// block with the size `len`, and materialize them by calling
    /// [`drain_aux`](#methods.drain_aux) according to the `perform` argument.
    /// If successful, it returns the offset of the available free block.
    /// Otherwise, `u64::MAX` is returned.
    pub unsafe fn alloc_impl(&mut self, len: usize, perform: bool) -> u64 {
        self.lock();

        let idx = get_idx(len);
        let len = 1 << idx;

        if len > self.available {
            eprintln!(
                "No space left (requested = {}, avilable= {})",
                len,
                self.available()
            );
            self.unlock();
            u64::MAX
        } else {
            match self.find_free_memory(idx, false) {
                Some(off) => {
                    #[cfg(feature = "verbose")]
                    debug_alloc(off, len, self.used(), self.used() + (1 << idx));

                    self.available_log = self.available - len;

                    if perform {
                        self.drain_aux();
                        self.drop_log.clear();
                        self.unlock();
                    } else {
                        self.aux.sync_all();
                    }

                    #[cfg(feature = "capture_footprint")]
                    {
                        let usage = self.size - self.available_log;
                        if usage > self.foot_print {
                            self.foot_print = usage;
                        }
                    }

                    off
                }
                None => {
                    eprintln!(
                        "No slot with size {} left (avilable= {})",
                        len,
                        self.available()
                    );
                    self.unlock();
                    u64::MAX
                }
            }
        }
    }

    #[inline]
    /// Generates required changes to the metadata for reclaiming the memory
    /// block at offset `off` with the size of `len`, and materialize them by
    /// calling [`drain_aux`](#methods.drain_aux) according to the `perform`
    /// argument.
    pub unsafe fn dealloc_impl(&mut self, off: u64, len: usize, perform: bool) {
        self.lock();

        let idx = get_idx(len);
        let len = 1 << idx;

        #[cfg(feature = "verbose")]
        debug_dealloc(off, len, self.used(), self.used() - len);

        self.available_log = self.available;
        self.free_impl(off, len);

        if perform {
            self.drain_aux();
            self.unlock();
        } else {
            self.aux.sync_all();
        }
    }

    #[inline]
    /// Materializes the changes in the auxiliary list and clears the drop log
    /// records
    pub unsafe fn perform(&mut self) {
        self.drain_aux();
        self.drop_log.clear();
        self.unlock();
    }

    #[inline]
    unsafe fn free_impl(&mut self, off: u64, len: usize) {
        let idx = get_idx(len);
        let end = off + (1 << idx);
        let mut curr = self.buddies[idx];
        let mut prev: Option<u64> = None;
        if idx < self.last_idx {
            while let Some(b) = off_to_option(curr) {
                let e = Self::buddy(b);
                let on_left = off & (1 << idx) == 0;
                if (b == end && on_left) || (b + len as u64 == off && !on_left) {
                    let off = u64::min(off, b);
                    if let Some(p) = prev {
                        self.aux_push(p, e.next);
                    } else {
                        self.aux_push(Self::get_off(&self.buddies[idx]), e.next);
                    }
                    self.available_log -= len;
                    self.free_impl(off, len << 1);
                    return;
                }
                if b > off {
                    break;
                }
                prev = Some(b);
                curr = e.next;
                // if curr == b {
                //     eprintln!("Double free for @{} ({})", off, len);
                //     self.aux.clear();
                //     return;
                // }
                debug_assert_ne!(curr, b, "Cyclic link in free_impl");
            }
        }
        if let Some(p) = prev {
            self.aux_push(off, Self::buddy(p).next);
            self.aux_push(p, off);
        } else {
            self.aux_push(off, self.buddies[idx]);
            self.aux_push(Self::get_off(&self.buddies[idx]), off);
        }
        self.available_log += len;
    }

    #[inline]
    /// Determines if the given address range is allocated
    pub fn is_allocated(&mut self, off: u64, _len: usize) -> bool {
        self.lock();

        if !self.aux.empty() {
            self.unlock();
            return true;
        }

        let end = off + _len as u64 - 1;
        let idx = get_idx(_len);
        for idx in idx..self.last_idx + 1 {
            let len = 1 << idx;
            let mut curr = self.buddies[idx];

            #[cfg(feature = "cyclic_link_check")]
            let mut links = vec![];

            while let Some(b) = off_to_option(curr) {
                #[cfg(feature = "cyclic_link_check")]
                {
                    if links.contains(&b) {
                        self.unlock();
                        panic!("A cyclic link detected in list {}", idx);
                    } else {
                        links.push(b);
                    }
                }

                let r = b + len;
                if (off >= b && off < r) || (end >= b && end < r) || (off <= b && end >= r) {
                    self.unlock();
                    return false;
                }
                if b > off {
                    break;
                }
                curr = Self::buddy(b).next;
                debug_assert_ne!(curr, b, "Cyclic link in is_allocated");
            }
        }
        self.unlock();
        true
    }

    #[inline]
    /// Starts the recovery procedure. If the crash happened while draining the
    /// auxiliary buffer, it continues draining it and making the remaining
    /// changes. It is rational because the [`DropOnFailure`] log was taken
    /// before draining the auxiliary buffer. When the draining is finished,
    /// the higher-level log reclaims the allocation in the higher level
    /// recovery procedure.
    /// 
    /// [`DropOnFailure`]: ../alloc/trait.MemPool.html#method.drop_on_failure
    pub fn recover(&mut self) {
        #[cfg(feature = "pthread")] unsafe {
        crate::sync::init_lock(&mut self.mutex.0, &mut self.mutex.1);
        }

        #[cfg(not(feature = "pthread"))] {
        self.mutex = 0; }

        if self.aux_valid {
            #[cfg(debug_assertions)]
            eprintln!("Crashed while the allocator was operating");

            // continue draining
            self.drain_aux();

            #[cfg(debug_assertions)]
            self.check(module_path!());
        } else {
            self.aux.clear();
            self.log64.clear();
        }

        // drop unnecessary allocations
        unsafe {
            let self_mut = self as *mut Self;
            self.drop_log.foreach_atomic(|(off, len)| {
                (*self_mut).dealloc_impl(off, len, false);
            }, || {
                (*self_mut).drain_aux();
                (*self_mut).unlock();
            });
        }
        self.drop_log.clear();
    }

    #[inline]
    /// Returns the pool size
    pub fn size(&self) -> usize {
        self.size
    }

    #[inline]
    /// Returns the total available space in the pool
    pub fn available(&self) -> usize {
        self.available
    }

    #[inline]
    /// Returns the total number of bytes used from the pool
    pub fn used(&self) -> usize {
        self.size - self.available
    }

    #[cfg(feature = "capture_footprint")]
    /// Returns the total number of bytes written to the pool. It may exceed the
    /// pool size as it does not subtract the reclaimed space after being used.
    pub fn footprint(&self) -> usize {
        self.foot_print
    }

    fn check(&self, f: &str) {
        for idx in 3..self.last_idx + 1 {
            let mut curr = self.buddies[idx];
            while let Some(b) = off_to_option(curr) {
                let e = Self::buddy(b);
                curr = e.next;
                assert_ne!(curr, b, "Cyclic link in checking {}", f);
            }
        }
    }

    /// Prints the free lists
    pub fn print(&self) {
        println!();
        for idx in 3..self.last_idx + 1 {
            print!("{:>8} [{:>2}] ", 1 << idx, idx);
            let mut curr = self.buddies[idx];
            while let Some(b) = off_to_option(curr) {
                print!("({}..{})", b, b + (1 << idx) - 1);
                let e = Self::buddy(b);
                curr = e.next;
            }
            println!();
        }
    }
}

#[cfg(test)]
mod test {
    use crate::default::*;
    use crate::boxed::Pbox;
    type P = BuddyAlloc;

    #[test]
    fn buddy_alg_test() {
        let _pool = P::open_no_root("buddy.pool", O_CFNE).unwrap();
        crate::utils::allow_crash(true);
        P::transaction(|j| {
            let _b = Pbox::new(1, j);
        })
        .unwrap();
        println!("{}", P::used());
    }
}

#[macro_export]
/// This macro creates a new pool module and aliases for persistent types. It
/// generates type [`BuddyAlloc`] which a persistent allocator type. It is
/// recommended to alias the [`BuddyAlloc`] type for tidiness.
/// 
/// The aliased types are 
/// 
/// * `Pbox<T>` = [`crndm::boxed::Pbox`]`<T, `[`BuddyAlloc`]`>`
/// * `Prc<T>` = [`crndm::prc::Prc`]`<T, `[`BuddyAlloc`]`>`
/// * `Parc<T>` = [`crndm::sync::Parc`]`<T, `[`BuddyAlloc`]`>`
/// * `PMutex<T>` = [`crndm::sync::Mutex`]`<T, `[`BuddyAlloc`]`>`
/// * `PCell<T>` = [`crndm::cell::LogCell`]`<T, `[`BuddyAlloc`]`>`
/// * `PRefCell<T>` = [`crndm::cell::LogRefCell`]`<T, `[`BuddyAlloc`]`>`
/// * `VCell<T>` = [`crndm::cell::VCell`]`<T, `[`BuddyAlloc`]`>`
/// * `PVec<T>` = [`crndm::vec::Vec`]`<T, `[`BuddyAlloc`]`>`
/// * `PString` = [`crndm::str::String`]`<`[`BuddyAlloc`]`>`
///
/// # Examples
/// 
/// To associate a single pool to the program, it is enough to define a pool
/// type using this macro.
/// 
/// ```
/// # fn main() {
/// crndm::pool!(my_alloc);
/// use my_alloc::*;
/// 
/// type P = BuddyAlloc;
/// 
/// let _ = P::open_no_root("p.pool", O_CF).unwrap();
/// 
/// P::transaction(|j| {
///     let temp = Pbox::new(10, j);
/// }).unwrap();
/// # }
/// ```
/// 
/// If multiple pools are needed, multiple pool modules can be defined and used.
/// 
/// ```
/// use crndm::alloc::*;
/// 
/// crndm::pool!(pool1);
/// crndm::pool!(pool2);
/// 
/// type P1 = pool1::BuddyAlloc;
/// type P2 = pool2::BuddyAlloc;
/// 
/// let _ = P1::open_no_root("p1.pool", O_CF).unwrap();
/// let _ = P2::open_no_root("p2.pool", O_CF).unwrap();
/// 
/// P1::transaction(|j1| {
///     let temp = pool1::Pbox::new(10, j1);
///     P2::transaction(|j2| {
///         let temp = pool2::Pbox::new(20, j2);
///     }).unwrap();
/// }).unwrap();
/// ```
/// 
/// [`BuddyAlloc`]: ./alloc/default/struct.BuddyAlloc.html
/// [`crndm::boxed::Pbox`]: ./boxed/struct.Pbox.html
/// [`crndm::prc::Prc`]: ./prc/struct.Prc.html
/// [`crndm::sync::Parc`]: ./sync/struct.Parc.html
/// [`crndm::sync::Mutex`]: ./sync/struct.Mutex.html
/// [`crndm::cell::LogCell`]: ./cell/struct.LogCell.html
/// [`crndm::cell::LogRefCell`]: ./cell/struct.LogRefCell.html
/// [`crndm::cell::VCell`]: ./cell/struct.VCell.html
/// [`crndm::vec::Vec`]: ./vec/struct.Vec.html
/// [`crndm::str::String`]: ./str/struct.String.html
macro_rules! pool {
    ($name:ident) => {
        /// The default allocator module
        pub mod $name {
            use memmap::*;
            use std::collections::hash_map::DefaultHasher;
            use std::collections::HashMap;
            use std::fs::OpenOptions;
            use std::hash::{Hash, Hasher};
            use std::mem;
            use std::ops::Range;
            use std::path::{Path, PathBuf};
            use std::sync::atomic::{AtomicBool, Ordering};
            use std::sync::{Arc, Mutex};
            use std::thread::current;
            use std::thread::ThreadId;
            use $crate::ll::*;
            use $crate::result::Result;
            pub use $crate::*;
            pub use $crate::alloc::*;
            pub use $crate::cell::{RootCell, RootObj};
            pub use $crate::clone::PClone;
            pub use $crate::convert::PFrom;
            pub use $crate::str::ToString;
            pub use $crate::stm::transaction;

            static mut BUDDY_START: u64 = 0;
            static mut BUDDY_VALID_START: u64 = 0;
            static mut BUDDY_END: u64 = 0;

            #[repr(C)]
            struct BuddyAllocInner {
                magic_number: u64,
                flags: u64,
                gen: u32,
                root_obj: u64,
                root_type_id: u64,
                logs: u64,
                size: usize,
                alg: &'static mut BuddyAlg<BuddyAlloc>,
            }

            struct VData {
                filename: String,
                mutex: Mutex<()>,
                journals: HashMap<ThreadId, (&'static Journal, i32)>,
                mmap: MmapMut,
            }

            union U<T> {
                raw: *mut u8,
                rf: *mut T,
            }

            impl<T> U<T> {
                pub fn read<'a>(raw: *mut u8) -> &'a mut T {
                    unsafe { &mut *U { raw }.rf }
                }
            }

            impl VData {
                fn new(mmap: MmapMut, filename: &str) -> Self {
                    Self {
                        filename: filename.to_string(),
                        mutex: Mutex::new(()),
                        journals: HashMap::new(),
                        mmap,
                    }
                }
            }

            impl BuddyAllocInner {
                fn init(&mut self, size: usize) {
                    let id = std::any::type_name::<Self>();
                    let mut s = DefaultHasher::new();
                    id.hash(&mut s);
                    self.flags = 0;
                    self.gen = 1;
                    self.root_obj = u64::MAX;
                    self.root_type_id = 0;
                    self.logs = u64::MAX;
                    self.size = size;

                    type T = BuddyAlg<BuddyAlloc>;
                    let base = self as *mut Self as u64;
                    let off = base + mem::size_of::<Self>() as u64;
                    self.alg = U::<T>::read(off as *mut u8);
                    self.alg.init(size);
                    self.magic_number = u64::MAX;
                    unsafe {
                        self.alg.alloc_impl(
                            mem::size_of::<Self>() + mem::size_of::<T>(),
                            true,
                        );
                    }
                    self.magic_number = s.finish();
                }

                fn as_bytes(&self) -> &[u8] {
                    let ptr: *const Self = self;
                    let ptr = ptr as *const u8;
                    unsafe { std::slice::from_raw_parts(ptr, std::mem::size_of::<Self>()) }
                }

                fn has_root(&self) -> bool {
                    self.flags & FLAG_HAS_ROOT == FLAG_HAS_ROOT
                }
            }

            /// A memory allocator with buddy allocation mechanism
            ///
            /// To define a new buddy allocator type as a memory pool, you may
            /// use [`pool!()`] macro. 
            /// 
            /// [`pool!()`]: ../../macro.pool.html
            pub struct BuddyAlloc {}

            static_inner_object!(BUDDY_INNER, BuddyAllocInner);
            static mut VDATA: Option<VData> = None;
            static mut OPEN: AtomicBool = AtomicBool::new(false);
            static mut MAX_GEN: u32 = 0;

            impl BuddyAlloc {
                fn inside_transaction() -> bool {
                    unsafe {
                        if let Some(vdata) = &VDATA {
                            !vdata.journals.is_empty()
                        } else {
                            false
                        }
                    }
                }

                /// Opens a memory pool file and returns an instance of
                /// [`BuddyAlloc`](#) if success. The pool remains open as long
                /// as the instance lives.
                #[track_caller]
                pub fn open_impl(filename: &str) -> Result<Self> {
                    let metadata = std::fs::metadata(filename);
                    if let Err(e) = &metadata {
                        Err(format!("{}", e))
                    } else {
                        let metadata = metadata.unwrap();
                        assert!(metadata.is_file());
                        if metadata.len() < 8 {
                            Err("Invalid pool file".to_string())
                        } else {
                            let path = PathBuf::from(filename);
                            let file = OpenOptions::new()
                                .read(true)
                                .write(true)
                                .create(true)
                                .open(&path)
                                .unwrap();

                            let mut mmap =
                                unsafe { memmap::MmapOptions::new().map_mut(&file).unwrap() };

                            let raw_offset = mmap.get_mut(0).unwrap();

                            let id = std::any::type_name::<BuddyAllocInner>();
                            let mut s = DefaultHasher::new();
                            id.hash(&mut s);
                            let id = s.finish();

                            let inner = U::<BuddyAllocInner>::read(raw_offset);
                            assert_eq!(
                                inner.magic_number, id,
                                "Invalid magic number for the pool image file"
                            );

                            let base = raw_offset as *mut _ as u64;
                            let off = base + mem::size_of::<BuddyAllocInner>() as u64;
                            inner.alg = U::<BuddyAlg<Self>>::read(off as *mut u8);

                            unsafe {
                                inner.gen = u32::max(MAX_GEN, inner.gen) + 1;
                                MAX_GEN = inner.gen;
                                BUDDY_START = base;
                                BUDDY_VALID_START = base
                                    + mem::size_of::<BuddyAllocInner>() as u64
                                    + mem::size_of::<BuddyAlg<Self>>() as u64;
                                BUDDY_END = BUDDY_START + inner.size as u64 + 1;
                                BUDDY_INNER = Some(inner);
                                VDATA = Some(VData::new(mmap, filename));
                            }

                            Ok(Self {})
                        }
                    }
                }
            }

            unsafe impl MemPool for BuddyAlloc {
                #[cfg(any(feature = "concurrent_pools", test))]
                #[allow(unused_unsafe)]
                #[track_caller]
                fn open_no_root(path: &str, flags: u32) -> Result<Self> {
                    unsafe {
                        while OPEN.compare_and_swap(false, true, Ordering::AcqRel) {}
                        if !Self::inside_transaction() {
                            if let Ok(_) = Self::apply_flags(path, flags) {
                                let res = Self::open_impl(path);
                                if res.is_ok() {
                                    Self::recover();
                                }
                                res
                            } else {
                                OPEN.store(false, Ordering::Release);
                                Err("Could not open file".to_string())
                            }
                        } else {
                            OPEN.store(false, Ordering::Release);
                            Err("Could not open a pool inside a transaction of its own kind"
                                .to_string())
                        }
                    }
                }

                #[cfg(not(any(feature = "concurrent_pools", test)))]
                #[allow(unused_unsafe)]
                #[track_caller]
                fn open_no_root(path: &str, flags: u32) -> Result<Self> {
                    unsafe {
                        if !OPEN.compare_and_swap(false, true, Ordering::AcqRel) {
                            if !Self::inside_transaction() {
                                if let Ok(_) = Self::apply_flags(path, flags) {
                                    let res = mem::ManuallyDrop::new(Self::open_impl(path));
                                    if res.is_ok() {
                                        Self::recover();
                                    }
                                    mem::ManuallyDrop::into_inner(res)
                                } else {
                                    OPEN.store(false, Ordering::Release);
                                    Err("Could not open file".to_string())
                                }
                            } else {
                                OPEN.store(false, Ordering::Release);
                                Err("Could not open a pool inside a transaction of its own kind"
                                    .to_string())
                            }
                        } else {
                            static_inner!(VDATA, vdata, {
                                Err(format!(
                                    "The pool was already opened (`{}')",
                                    vdata.filename
                                ))
                            })
                        }
                    }
                }

                #[allow(unused_unsafe)]
                unsafe fn close() -> Result<()> {
                    if OPEN.load(Ordering::Acquire) {
                        static_inner!(BUDDY_INNER, inner, {
                            while let Ok(logs) =Self::deref_mut::<Journal>(inner.logs) {

                                logs.commit();
                                logs.clear();

                                #[cfg(feature = "pin_journals")]
                                Self::drop_journal(logs);
                            }
                            OPEN.store(false, Ordering::Release);
                            Ok(())
                        })
                    } else {
                        Err("Pool was already closed".to_string())
                    }
                }

                /// Formats the image file
                unsafe fn format(filename: &str) -> Result<()> {
                    if Path::new(filename).exists() {
                        let file = OpenOptions::new()
                            .read(true)
                            .write(true)
                            .create(true)
                            .open(filename);
                        if let Err(e) = &file {
                            Err(format!("{}", e))
                        } else {
                            let file = file.unwrap();
                            let mut len = file.metadata().unwrap().len() as usize;
                            if len < 8 {
                                len = 10 * 1024 * 1024;
                                file.set_len(len as u64).unwrap();
                            }

                            let mut mmap = memmap::MmapOptions::new().map_mut(&file).unwrap();
                            let begin = mmap.get_mut(0).unwrap();
                            std::ptr::write_bytes(begin, 0xff, 8);
                            BUDDY_START = begin as *const _ as u64;
                            BUDDY_END = u64::MAX;

                            let inner = U::<BuddyAllocInner>::read(begin);
                            inner.init(len);
                            mmap.flush().unwrap();
                            Ok(())
                        }
                    } else {
                        Err("Image file does not exist".to_string())
                    }
                }

                #[inline]
                fn gen() -> u32 {
                    static_inner!(BUDDY_INNER, inner, { inner.gen })
                }

                fn size() -> usize {
                    static_inner!(BUDDY_INNER, inner, { inner.size })
                }

                #[inline]
                fn available() -> usize {
                    static_inner!(BUDDY_INNER, inner, { inner.alg.available() })
                }

                fn used() -> usize {
                    static_inner!(BUDDY_INNER, inner, { inner.alg.used() })
                }

                #[inline]
                fn rng() -> Range<u64> {
                    unsafe { BUDDY_VALID_START..BUDDY_END }
                }

                #[inline]
                fn start() -> u64 {
                    unsafe { BUDDY_START }
                }

                #[inline]
                fn end() -> u64 {
                    unsafe { BUDDY_END }
                }

                #[allow(unused_unsafe)]
                #[track_caller]
                unsafe fn pre_alloc(size: usize) -> (*mut u8, u64, usize) {
                    static_inner!(BUDDY_INNER, inner, {
                        let a = inner.alg.alloc_impl(size, false);
                        if a != u64::MAX {
                            (Self::get_mut_unchecked(a), a, size)
                        } else {
                            (std::ptr::null_mut(), u64::MAX, 0)
                        }
                    })
                }

                #[allow(unused_unsafe)]
                #[track_caller]
                unsafe fn pre_dealloc(ptr: *mut u8, size: usize) {
                    static_inner!(BUDDY_INNER, inner, {
                        let off = Self::off(ptr).expect("invalid pointer");
                        if cfg!(feature = "access_violation_check") {
                            if inner.alg.is_allocated(off, size) {
                                inner.alg.dealloc_impl(off, size, false);
                            } else {
                                panic!("offset @{} ({}) was not allocated", off, size);
                            }
                        } else {
                            inner.alg.dealloc_impl(off, size, false);
                        }
                    })
                }

                #[allow(unused_unsafe)]
                #[track_caller]
                unsafe fn pre_realloc(ptr: *mut *mut u8, size: usize, new_size: usize) -> bool {
                    static_inner!(BUDDY_INNER, inner, {
                        let off = Self::off(*ptr).expect("invalid pointer");
                        if get_idx(size) == get_idx(new_size) {
                            // New size is already available
                            true
                        } else {
                            inner.alg.dealloc_impl(off, size, false);
                            let a = inner.alg.alloc_impl(new_size, false);
                            if a == u64::MAX {
                                // No space left
                                Self::discard();
                                false
                            } else {
                                // Successful reallocation
                                let new_ptr = Self::deref_mut(a).unwrap();
                                std::ptr::copy_nonoverlapping(
                                    *ptr,
                                    new_ptr,
                                    std::cmp::min(size, new_size),
                                );
                                *ptr = new_ptr;
                                true
                            }
                        }
                    })
                }

                #[inline]
                #[allow(unused_unsafe)]
                #[track_caller]
                unsafe fn log64(obj: *const u64, val: u64) {
                    static_inner!(BUDDY_INNER, inner, {
                        let off = (obj as u64);
                        debug_assert!(off > BUDDY_START && off < BUDDY_END);
                        let off = off - BUDDY_START;
                        inner.alg.log(off, val);
                    })
                }

                #[inline]
                #[allow(unused_unsafe)]
                #[track_caller]
                unsafe fn drop_on_failure(off: u64, len: usize) {
                    static_inner!(BUDDY_INNER, inner, {
                        inner.alg.drop_on_failure(off, len);
                    })
                }

                #[inline]
                #[allow(unused_unsafe)]
                #[track_caller]
                unsafe fn perform() {
                    static_inner!(BUDDY_INNER, inner, {
                        inner.alg.perform();
                    })
                }

                #[inline]
                #[allow(unused_unsafe)]
                #[track_caller]
                unsafe fn discard() {
                    static_inner!(BUDDY_INNER, inner, {
                        inner.alg.discard();
                    })
                }

                #[inline]
                #[allow(unused_unsafe)]
                fn allocated(off: u64, len: usize) -> bool {
                    static_inner!(BUDDY_INNER, inner, {
                        if off >= Self::end() {
                            false
                        } else if Self::contains(off + Self::start()) {
                            if cfg!(feature = "access_violation_check") {
                                inner.alg.is_allocated(off, len)
                            } else {
                                true
                            }
                        } else {
                            false
                        }
                    })
                }

                #[allow(unused_unsafe)]
                unsafe fn new_journal(tid: ThreadId) {
                    let journal = static_inner!(BUDDY_INNER, inner, {
                        let (journal, _, _) = Self::atomic_new(Journal::new());
                        journal.enter_into(&inner.logs);
                        Self::perform();
                        journal
                    });
                    Self::journals().insert(tid, (journal, 0));
                }

                #[allow(unused_unsafe)]
                unsafe fn drop_journal(journal: &mut Journal) {
                    static_inner!(BUDDY_INNER, inner, {
                        let off = Self::off(journal).unwrap();
                        let tid = current().id();
                        if inner.logs == off {
                            inner.logs = journal.next_off();
                        }

                        #[cfg(feature = "pin_journals")]
                        journal.drop_pages();

                        Self::free_nolog(journal);
                        Self::perform();
                        Self::journals().remove(&tid);
                    })
                }

                #[allow(unused_unsafe)]
                unsafe fn journals(
                ) -> &'static mut HashMap<ThreadId, (&'static Journal, i32)> {
                    static_inner!(VDATA, vdata, { &mut vdata.journals })
                }

                #[inline]
                #[allow(unused_unsafe)]
                unsafe fn guarded<T, F>(f: F) -> T
                where
                    F: FnOnce() -> T,
                {
                    static_inner!(VDATA, vdata, {
                        let _lock = vdata.mutex.lock();
                        f()
                    })
                }

                #[allow(unused_unsafe)]
                unsafe fn recover() {
                    static_inner!(BUDDY_INNER, inner, {
                        inner.alg.recover();
                        while let Ok(logs) = Self::deref_mut::<Journal>(inner.logs) {

                            #[cfg(feature = "verbose")]
                            println!("{:?}", logs);

                            logs.recover();
                            logs.clear();

                            #[cfg(feature = "pin_journals")]
                            Self::drop_journal(logs);
                        }
                    })
                }

                #[allow(unused_unsafe)]
                #[track_caller]
                fn open<'a, U: 'a + PSafe + RootObj<Self>>(
                    path: &str,
                    flags: u32,
                ) -> Result<RootCell<'a, U, Self>> {
                    let slf = Self::open_no_root(path, flags)?;
                    static_inner!(BUDDY_INNER, inner, {
                        // Replace it with std::any::TypeId::of::<U>() when it
                        // is available in the future
                        let id = std::any::type_name::<U>();
                        let mut s = DefaultHasher::new();
                        id.hash(&mut s);
                        let id = s.finish();
                        if !inner.has_root() {
                            if mem::size_of::<U>() == 0 {
                                Err("root type cannot be a ZST".to_string())
                            } else {
                                let root_off = Self::transaction(move |j| {
                                    let ptr = Self::new(U::init(j), j);
                                    Self::off_unchecked(ptr)
                                })
                                .unwrap();
                                let ptr = Self::get_unchecked(root_off);
                                inner.flags |= FLAG_HAS_ROOT;
                                inner.root_obj = root_off;
                                inner.root_type_id = id;
                                msync_obj(inner);
                                Ok((RootCell::new(ptr, Arc::new(slf))))
                            }
                        } else {
                            if inner.root_type_id == id {
                                Ok((RootCell::new(
                                    Self::deref::<U>(inner.root_obj)?,
                                    Arc::new(slf),
                                )))
                            } else {
                                Err("Incompatible root type".to_string())
                            }
                        }
                    })
                }

                #[cfg(feature = "capture_footprint")]
                fn footprint() -> usize {
                    static_inner!(BUDDY_INNER, inner, { inner.alg.footprint() })
                }

                fn print_info() {
                    println!("      Total: {} bytes", Self::size());
                    println!("       Used: {} bytes", Self::used());
                    println!("  Available: {} bytes", Self::available());
                    print!("Free Blocks:");
                    static_inner!(BUDDY_INNER, inner, { inner.alg.print() })
                }
            }

            impl Drop for BuddyAlloc {
                fn drop(&mut self) {
                    unsafe {
                        Self::close().unwrap();
                    }
                }
            }

            /// Compact form of [`Pbox`](../boxed/struct.Pbox.html)
            /// `<T,`[`BuddyAlloc`](./struct.BuddyAlloc.html)`>`.
            pub type Pbox<T> = $crate::boxed::Pbox<T, BuddyAlloc>;

            /// Compact form of [`Prc`](../prc/struct.Prc.html)
            /// `<T,`[`BuddyAlloc`](./struct.BuddyAlloc.html)`>`.
            pub type Prc<T> = $crate::prc::Prc<T, BuddyAlloc>;

            /// Compact form of [`Parc`](../sync/struct.Parc.html)
            /// `<T,`[`BuddyAlloc`](./struct.BuddyAlloc.html)`>`.
            pub type Parc<T> = $crate::sync::Parc<T, BuddyAlloc>;

            /// Compact form of [`Mutex`](../sync/struct.Mutex.html)
            /// `<T,`[`BuddyAlloc`](./struct.BuddyAlloc.html)`>`.
            pub type PMutex<T> = $crate::sync::Mutex<T, BuddyAlloc>;

            /// Compact form of [`LogCell`](../cell/struct.LogCell.html)
            /// `<T,`[`BuddyAlloc`](./struct.BuddyAlloc.html)`>`.
            pub type PCell<T> = $crate::cell::LogCell<T, BuddyAlloc>;

            /// Compact form of [`LogNonNull`](../ptr/struct.LogNonNull.html)
            /// `<T,`[`BuddyAlloc`](./struct.BuddyAlloc.html)`>`.
            pub type PNonNull<T> = $crate::ptr::LogNonNull<T, BuddyAlloc>;

            /// Compact form of [`LogRefCell`](../cell/struct.LogRefCell.html)
            /// `<T,`[`BuddyAlloc`](./struct.BuddyAlloc.html)`>`.
            pub type PRefCell<T> = $crate::cell::LogRefCell<T, BuddyAlloc>;

            /// Compact form of [`Ref`](../cell/struct.Ref.html)
            /// `<'b, T, `[`BuddyAlloc`](./struct.BuddyAlloc.html)`>`.
            pub type PRef<'b, T> = $crate::cell::Ref<'b, T, BuddyAlloc>;

            /// Compact form of [`RefMut`](../cell/struct.Mut.html)
            /// `<'b, T, `[`BuddyAlloc`](./struct.BuddyAlloc.html)`>`.
            pub type PRefMut<'b, T> = $crate::cell::RefMut<'b, T, BuddyAlloc>;

            /// Compact form of `[VCell](../cell/struct.VCell.html)
            /// `<T,`[`BuddyAlloc`](./struct.BuddyAlloc.html)`>`.
            pub type VCell<T> = $crate::cell::VCell<T, BuddyAlloc>;

            /// Compact form of [`Vec`](../vec/struct.Vec.html)
            /// `<T,`[`BuddyAlloc`](./struct.BuddyAlloc.html)`>`.
            pub type PVec<T> = $crate::vec::Vec<T, BuddyAlloc>;

            /// Compact form of [`String`](../str/struct.String.html)
            /// `<`[`BuddyAlloc`](./struct.BuddyAlloc.html)`>`.
            pub type PString = $crate::str::String<BuddyAlloc>;

            /// Compact form of [`Journal`](../stm/struct.Journal.html)
            /// `<`[`BuddyAlloc`](./struct.BuddyAlloc.html)`>`.
            pub type Journal = $crate::stm::Journal<BuddyAlloc>;
        }
    };
}

// This is an example of defining a new buddy allocator type
// `BuddyAlloc` is the default allocator with Buddy Allocation
crate::pool!(default);

#[cfg(feature = "verbose")]
pub fn debug_alloc(addr: u64, len: usize, pre: usize, post: usize) {
    println!(
        "{}",
        Green.paint(format!(
            "                     PRE: {:<6}  ({:>4}..{:<4}) = {:<4}  POST = {:<6}",
            pre,
            addr,
            addr + len as u64 - 1,
            len,
            post
        ))
    );
}

#[cfg(feature = "verbose")]
pub fn debug_dealloc(addr: u64, len: usize, pre: usize, post: usize) {
    println!(
        "{}",
        Red.paint(format!(
            "          DEALLOC    PRE: {:<6}  ({:>4}..{:<4}) = {:<4}  POST = {:<6}",
            pre,
            addr,
            addr + len as u64 - 1,
            len,
            post
        ))
    );
}
