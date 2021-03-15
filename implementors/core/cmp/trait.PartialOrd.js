(function() {var implementors = {};
implementors["corundum"] = [{"text":"impl&lt;T:&nbsp;<a class=\"trait\" href=\"corundum/trait.PSafe.html\" title=\"trait corundum::PSafe\">PSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html\" title=\"trait core::cmp::PartialOrd\">PartialOrd</a> + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>, A:&nbsp;<a class=\"trait\" href=\"corundum/alloc/trait.MemPool.html\" title=\"trait corundum::alloc::MemPool\">MemPool</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html\" title=\"trait core::cmp::PartialOrd\">PartialOrd</a>&lt;<a class=\"struct\" href=\"corundum/boxed/struct.Pbox.html\" title=\"struct corundum::boxed::Pbox\">Pbox</a>&lt;T, A&gt;&gt; for <a class=\"struct\" href=\"corundum/boxed/struct.Pbox.html\" title=\"struct corundum::boxed::Pbox\">Pbox</a>&lt;T, A&gt;","synthetic":false,"types":["corundum::boxed::Pbox"]},{"text":"impl&lt;T:&nbsp;<a class=\"trait\" href=\"corundum/trait.PSafe.html\" title=\"trait corundum::PSafe\">PSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html\" title=\"trait core::cmp::PartialOrd\">PartialOrd</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a>, A:&nbsp;<a class=\"trait\" href=\"corundum/alloc/trait.MemPool.html\" title=\"trait corundum::alloc::MemPool\">MemPool</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html\" title=\"trait core::cmp::PartialOrd\">PartialOrd</a>&lt;<a class=\"struct\" href=\"corundum/cell/struct.PCell.html\" title=\"struct corundum::cell::PCell\">PCell</a>&lt;T, A&gt;&gt; for <a class=\"struct\" href=\"corundum/cell/struct.PCell.html\" title=\"struct corundum::cell::PCell\">PCell</a>&lt;T, A&gt;","synthetic":false,"types":["corundum::cell::cell::PCell"]},{"text":"impl&lt;T:&nbsp;<a class=\"trait\" href=\"corundum/trait.PSafe.html\" title=\"trait corundum::PSafe\">PSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html\" title=\"trait core::cmp::PartialOrd\">PartialOrd</a> + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>, A:&nbsp;<a class=\"trait\" href=\"corundum/alloc/trait.MemPool.html\" title=\"trait corundum::alloc::MemPool\">MemPool</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html\" title=\"trait core::cmp::PartialOrd\">PartialOrd</a>&lt;<a class=\"struct\" href=\"corundum/cell/struct.PRefCell.html\" title=\"struct corundum::cell::PRefCell\">PRefCell</a>&lt;T, A&gt;&gt; for <a class=\"struct\" href=\"corundum/cell/struct.PRefCell.html\" title=\"struct corundum::cell::PRefCell\">PRefCell</a>&lt;T, A&gt;","synthetic":false,"types":["corundum::cell::refcell::PRefCell"]},{"text":"impl&lt;T:&nbsp;<a class=\"trait\" href=\"corundum/trait.PSafe.html\" title=\"trait corundum::PSafe\">PSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html\" title=\"trait core::cmp::PartialOrd\">PartialOrd</a>, A:&nbsp;<a class=\"trait\" href=\"corundum/alloc/trait.MemPool.html\" title=\"trait corundum::alloc::MemPool\">MemPool</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html\" title=\"trait core::cmp::PartialOrd\">PartialOrd</a>&lt;<a class=\"struct\" href=\"corundum/cell/struct.RootCell.html\" title=\"struct corundum::cell::RootCell\">RootCell</a>&lt;'_, T, A&gt;&gt; for <a class=\"struct\" href=\"corundum/cell/struct.RootCell.html\" title=\"struct corundum::cell::RootCell\">RootCell</a>&lt;'_, T, A&gt;","synthetic":false,"types":["corundum::cell::rootcell::RootCell"]},{"text":"impl&lt;T:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> + <a class=\"trait\" href=\"corundum/trait.VSafe.html\" title=\"trait corundum::VSafe\">VSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html\" title=\"trait core::cmp::PartialOrd\">PartialOrd</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a>, A:&nbsp;<a class=\"trait\" href=\"corundum/alloc/trait.MemPool.html\" title=\"trait corundum::alloc::MemPool\">MemPool</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html\" title=\"trait core::cmp::PartialOrd\">PartialOrd</a>&lt;<a class=\"struct\" href=\"corundum/cell/struct.VCell.html\" title=\"struct corundum::cell::VCell\">VCell</a>&lt;T, A&gt;&gt; for <a class=\"struct\" href=\"corundum/cell/struct.VCell.html\" title=\"struct corundum::cell::VCell\">VCell</a>&lt;T, A&gt;","synthetic":false,"types":["corundum::cell::vcell::VCell"]},{"text":"impl&lt;T:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> + <a class=\"trait\" href=\"corundum/trait.VSafe.html\" title=\"trait corundum::VSafe\">VSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html\" title=\"trait core::cmp::PartialOrd\">PartialOrd</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a>, A:&nbsp;<a class=\"trait\" href=\"corundum/alloc/trait.MemPool.html\" title=\"trait corundum::alloc::MemPool\">MemPool</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html\" title=\"trait core::cmp::PartialOrd\">PartialOrd</a>&lt;T&gt; for <a class=\"struct\" href=\"corundum/cell/struct.VCell.html\" title=\"struct corundum::cell::VCell\">VCell</a>&lt;T, A&gt;","synthetic":false,"types":["corundum::cell::vcell::VCell"]},{"text":"impl&lt;T:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> + <a class=\"trait\" href=\"corundum/trait.VSafe.html\" title=\"trait corundum::VSafe\">VSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html\" title=\"trait core::cmp::PartialOrd\">PartialOrd</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a>, A:&nbsp;<a class=\"trait\" href=\"corundum/alloc/trait.MemPool.html\" title=\"trait corundum::alloc::MemPool\">MemPool</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html\" title=\"trait core::cmp::PartialOrd\">PartialOrd</a>&lt;<a class=\"struct\" href=\"corundum/cell/struct.TCell.html\" title=\"struct corundum::cell::TCell\">TCell</a>&lt;T, A&gt;&gt; for <a class=\"struct\" href=\"corundum/cell/struct.TCell.html\" title=\"struct corundum::cell::TCell\">TCell</a>&lt;T, A&gt;","synthetic":false,"types":["corundum::cell::tcell::TCell"]},{"text":"impl&lt;T:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> + <a class=\"trait\" href=\"corundum/trait.VSafe.html\" title=\"trait corundum::VSafe\">VSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html\" title=\"trait core::cmp::PartialOrd\">PartialOrd</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a>, A:&nbsp;<a class=\"trait\" href=\"corundum/alloc/trait.MemPool.html\" title=\"trait corundum::alloc::MemPool\">MemPool</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html\" title=\"trait core::cmp::PartialOrd\">PartialOrd</a>&lt;T&gt; for <a class=\"struct\" href=\"corundum/cell/struct.TCell.html\" title=\"struct corundum::cell::TCell\">TCell</a>&lt;T, A&gt;","synthetic":false,"types":["corundum::cell::tcell::TCell"]},{"text":"impl&lt;T:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html\" title=\"trait core::cmp::PartialOrd\">PartialOrd</a> + <a class=\"trait\" href=\"corundum/trait.PSafe.html\" title=\"trait corundum::PSafe\">PSafe</a> + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>, A:&nbsp;<a class=\"trait\" href=\"corundum/alloc/trait.MemPool.html\" title=\"trait corundum::alloc::MemPool\">MemPool</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html\" title=\"trait core::cmp::PartialOrd\">PartialOrd</a>&lt;<a class=\"struct\" href=\"corundum/prc/struct.Prc.html\" title=\"struct corundum::prc::Prc\">Prc</a>&lt;T, A&gt;&gt; for <a class=\"struct\" href=\"corundum/prc/struct.Prc.html\" title=\"struct corundum::prc::Prc\">Prc</a>&lt;T, A&gt;","synthetic":false,"types":["corundum::prc::Prc"]},{"text":"impl&lt;T:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html\" title=\"trait core::cmp::PartialOrd\">PartialOrd</a> + <a class=\"trait\" href=\"corundum/trait.PSafe.html\" title=\"trait corundum::PSafe\">PSafe</a> + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>, A:&nbsp;<a class=\"trait\" href=\"corundum/alloc/trait.MemPool.html\" title=\"trait corundum::alloc::MemPool\">MemPool</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html\" title=\"trait core::cmp::PartialOrd\">PartialOrd</a>&lt;<a class=\"struct\" href=\"corundum/sync/struct.Parc.html\" title=\"struct corundum::sync::Parc\">Parc</a>&lt;T, A&gt;&gt; for <a class=\"struct\" href=\"corundum/sync/struct.Parc.html\" title=\"struct corundum::sync::Parc\">Parc</a>&lt;T, A&gt;","synthetic":false,"types":["corundum::sync::parc::Parc"]},{"text":"impl&lt;A:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html\" title=\"trait core::cmp::PartialOrd\">PartialOrd</a> + <a class=\"trait\" href=\"corundum/alloc/trait.MemPool.html\" title=\"trait corundum::alloc::MemPool\">MemPool</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html\" title=\"trait core::cmp::PartialOrd\">PartialOrd</a>&lt;<a class=\"struct\" href=\"corundum/str/struct.String.html\" title=\"struct corundum::str::String\">String</a>&lt;A&gt;&gt; for <a class=\"struct\" href=\"corundum/str/struct.String.html\" title=\"struct corundum::str::String\">String</a>&lt;A&gt;","synthetic":false,"types":["corundum::str::String"]},{"text":"impl&lt;A:&nbsp;<a class=\"trait\" href=\"corundum/alloc/trait.MemPool.html\" title=\"trait corundum::alloc::MemPool\">MemPool</a>, T:&nbsp;<a class=\"trait\" href=\"corundum/trait.PSafe.html\" title=\"trait corundum::PSafe\">PSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html\" title=\"trait core::cmp::PartialOrd\">PartialOrd</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html\" title=\"trait core::cmp::PartialOrd\">PartialOrd</a>&lt;<a class=\"struct\" href=\"corundum/vec/struct.Vec.html\" title=\"struct corundum::vec::Vec\">Vec</a>&lt;T, A&gt;&gt; for <a class=\"struct\" href=\"corundum/vec/struct.Vec.html\" title=\"struct corundum::vec::Vec\">Vec</a>&lt;T, A&gt;","synthetic":false,"types":["corundum::vec::Vec"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()