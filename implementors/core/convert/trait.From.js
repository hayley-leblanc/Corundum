(function() {var implementors = {};
implementors["corundum"] = [{"text":"impl&lt;T:&nbsp;<a class=\"trait\" href=\"corundum/trait.PSafe.html\" title=\"trait corundum::PSafe\">PSafe</a>, A:&nbsp;<a class=\"trait\" href=\"corundum/alloc/trait.MemPool.html\" title=\"trait corundum::alloc::MemPool\">MemPool</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;T&gt; for <a class=\"struct\" href=\"corundum/cell/struct.PRefCell.html\" title=\"struct corundum::cell::PRefCell\">PRefCell</a>&lt;T, A&gt;","synthetic":false,"types":["corundum::cell::refcell::PRefCell"]},{"text":"impl&lt;A:&nbsp;<a class=\"trait\" href=\"corundum/alloc/trait.MemPool.html\" title=\"trait corundum::alloc::MemPool\">MemPool</a>, T:&nbsp;<a class=\"trait\" href=\"corundum/trait.PSafe.html\" title=\"trait corundum::PSafe\">PSafe</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">&amp;'_ [T]</a>&gt; for <a class=\"struct\" href=\"corundum/ptr/struct.Slice.html\" title=\"struct corundum::ptr::Slice\">Slice</a>&lt;T, A&gt;","synthetic":false,"types":["corundum::ptr::slice::Slice"]},{"text":"impl&lt;A:&nbsp;<a class=\"trait\" href=\"corundum/alloc/trait.MemPool.html\" title=\"trait corundum::alloc::MemPool\">MemPool</a>, T:&nbsp;<a class=\"trait\" href=\"corundum/trait.PSafe.html\" title=\"trait corundum::PSafe\">PSafe</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">&amp;'_ mut [T]</a>&gt; for <a class=\"struct\" href=\"corundum/ptr/struct.Slice.html\" title=\"struct corundum::ptr::Slice\">Slice</a>&lt;T, A&gt;","synthetic":false,"types":["corundum::ptr::slice::Slice"]},{"text":"impl&lt;A:&nbsp;<a class=\"trait\" href=\"corundum/alloc/trait.MemPool.html\" title=\"trait corundum::alloc::MemPool\">MemPool</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"corundum/str/struct.String.html\" title=\"struct corundum::str::String\">String</a>&lt;A&gt;&gt; for <a class=\"struct\" href=\"corundum/vec/struct.Vec.html\" title=\"struct corundum::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>, A&gt;","synthetic":false,"types":["corundum::vec::Vec"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()