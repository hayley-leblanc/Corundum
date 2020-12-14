//! Corundum Markers
//! 
use std::task::Poll;
use std::task::Context;
use std::pin::Pin;
use std::ops::{Deref, DerefMut};
use std::future::Future;
use std::panic::{RefUnwindSafe, UnwindSafe};
use std::cell::UnsafeCell;
use std::fmt;

/// It marks the implementing type to be free of pointers to the volatile heap,
/// and persistence safe.
///
/// Also, every type that allows interior mutability is not safe in persistence
/// terms, because there might be no log of the value. Atomic types are
/// persistence safe, even though they provide interior mutability.
pub unsafe auto trait PSafe {}

impl<T: ?Sized> !PSafe for *const T {}
impl<T: ?Sized> !PSafe for *mut T {}
impl<T> !PSafe for &T {}
impl<T> !PSafe for &mut T {}
impl !PSafe for std::fs::File {}

/// `UnsafeCell` is marked as PSafe because it exposes interior mutability
/// without taking a log, which is unsafe from persistence perspective.
impl<T: ?Sized> !PSafe for UnsafeCell<T> {}

/// It marks the implementing type to be safe crossing transaction boundaries
///
/// Types that implement this trait may go in/out of a transaction. This
/// guarantees no cross-pool referencing.
pub unsafe auto trait TxOutSafe {}

impl<T: ?Sized> !TxOutSafe for *const T {}
impl<T: ?Sized> !TxOutSafe for *mut T {}
impl<T: ?Sized> !TxOutSafe for &mut T {}
impl<T: ?Sized> !TxOutSafe for UnsafeCell<T> {}

unsafe impl TxOutSafe for String {}
unsafe impl<T> TxOutSafe for std::thread::JoinHandle<T> {}
unsafe impl<T> TxOutSafe for Vec<std::thread::JoinHandle<T>> {}

/// It is equal to UnwindSafe, but is used to ensure doubly that mutable
/// references cannot go inside a transaction.
///
/// # Safety
///
/// The user can safely specify a type as `UnwindSafe`, but `TxInSafe` is
/// unsafe to implement. This warns the programmer that the non-existence
/// of orphans is not guaranteed anymore.
pub unsafe auto trait TxInSafe {}

/// The implementing type can be asserted [`TxInSafe`] albeit being `!TxInSafe`
/// by [`AssertTxInSafe`](./struct.AssertTxInSafe.html).
/// 
/// [`TxInSafe`]: ./trait.TxInSafe.html
pub unsafe auto trait LooseTxInUnsafe {}

/// A simple wrapper around a type to assert that it is safe to go in a
/// transaction.
///
/// When using [`transaction`] it may be the case that some of the closed over
/// variables are not [`TxInSafe`] safe. For example if `&mut T` is captured the
/// compiler will generate a warning indicating that it is not [`TxInSafe`]. It
/// may not be the case, however, that this is actually a problem due to the
/// specific usage of [`transaction`] if transaction inward safety is
/// specifically taken into account. This wrapper struct is useful for a quick
/// and lightweight annotation that a variable is indeed [`TxInSafe`] at the
/// programmer's responsibilities. The `Journal` object cannot be wrapped by 
/// `AssertTxInSafe` to make sure no inter-pool pointer can be made.
///
/// # Examples
/// 
/// You may wrap individual captures, as shown below. This ensures that if a new
/// capture is added which is not [`TxInSafe`], you will get a compilation error
/// at that time, which will allow you to consider whether that new capture in
/// fact represent a bug or not. 
///
/// ```
/// use crndm::alloc::*;
/// use crndm::AssertTxInSafe;
///
/// let mut variable = 4;
/// let other_capture = 3;
///
/// let result = {
///     let mut wrapper = AssertTxInSafe(&mut variable);
///     Heap::transaction(move |_| {
///         **wrapper += other_capture;
///     })
/// };
/// // ...
/// ```
/// 
/// [`transaction`]: ./stm/fn.transaction.html
/// [`TxInSafe`]: ./trait.TxInSafe.html
pub struct AssertTxInSafe<T: LooseTxInUnsafe>(pub T);

impl<T: ?Sized> !TxInSafe for *mut T {}
impl<T: ?Sized> !TxInSafe for &mut T {}
impl<T: ?Sized> !TxInSafe for UnsafeCell<T> {}
impl<T: LooseTxInUnsafe> UnwindSafe for AssertTxInSafe<T> {}
impl<T: LooseTxInUnsafe> RefUnwindSafe for AssertTxInSafe<T> {}
unsafe impl<T: LooseTxInUnsafe> TxInSafe for AssertTxInSafe<T> {}

impl<T: LooseTxInUnsafe> Deref for AssertTxInSafe<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T: LooseTxInUnsafe> DerefMut for AssertTxInSafe<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<R, F: FnOnce() -> R> FnOnce<()> for AssertTxInSafe<F> 
where F: LooseTxInUnsafe{
    type Output = R;

    extern "rust-call" fn call_once(self, _args: ()) -> R {
        (self.0)()
    }
}

impl<T: fmt::Debug + LooseTxInUnsafe> fmt::Debug for AssertTxInSafe<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("AssertUnwindSafe").field(&self.0).finish()
    }
}

impl<F: Future + LooseTxInUnsafe> Future for AssertTxInSafe<F> {
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let pinned_field = unsafe { Pin::map_unchecked_mut(self, |x| &mut x.0) };
        F::poll(pinned_field, cx)
    }
}

/// Safe to be stored in volatile memory useful in `VCell` type to prevent
/// storing persistent pointers in [`VCell`](./cell/struct.VCell.html)
pub unsafe auto trait VSafe {}
