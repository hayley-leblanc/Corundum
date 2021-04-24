(function() {var implementors = {};
implementors["corundum"] = [{"text":"impl&lt;T:&nbsp;<a class=\"trait\" href=\"corundum/trait.PSafe.html\" title=\"trait corundum::PSafe\">PSafe</a> + <a class=\"trait\" href=\"corundum/stm/trait.Logger.html\" title=\"trait corundum::stm::Logger\">Logger</a>&lt;A&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a>, A:&nbsp;<a class=\"trait\" href=\"corundum/alloc/trait.MemPool.html\" title=\"trait corundum::alloc::MemPool\">MemPool</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"corundum/cell/struct.PCell.html\" title=\"struct corundum::cell::PCell\">PCell</a>&lt;T, A&gt;","synthetic":false,"types":["corundum::cell::cell::PCell"]},{"text":"impl&lt;T:&nbsp;<a class=\"trait\" href=\"corundum/trait.PSafe.html\" title=\"trait corundum::PSafe\">PSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>, A:&nbsp;<a class=\"trait\" href=\"corundum/alloc/trait.MemPool.html\" title=\"trait corundum::alloc::MemPool\">MemPool</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"corundum/cell/struct.PRefCell.html\" title=\"struct corundum::cell::PRefCell\">PRefCell</a>&lt;T, A&gt;","synthetic":false,"types":["corundum::cell::refcell::PRefCell"]},{"text":"impl&lt;T:&nbsp;<a class=\"trait\" href=\"corundum/trait.PSafe.html\" title=\"trait corundum::PSafe\">PSafe</a>, A:&nbsp;<a class=\"trait\" href=\"corundum/alloc/trait.MemPool.html\" title=\"trait corundum::alloc::MemPool\">MemPool</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"corundum/cell/struct.RootCell.html\" title=\"struct corundum::cell::RootCell\">RootCell</a>&lt;'_, T, A&gt;","synthetic":false,"types":["corundum::cell::rootcell::RootCell"]},{"text":"impl&lt;T:&nbsp;<a class=\"trait\" href=\"corundum/trait.PSafe.html\" title=\"trait corundum::PSafe\">PSafe</a> + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>, A:&nbsp;<a class=\"trait\" href=\"corundum/alloc/trait.MemPool.html\" title=\"trait corundum::alloc::MemPool\">MemPool</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"corundum/prc/struct.VWeak.html\" title=\"struct corundum::prc::VWeak\">VWeak</a>&lt;T, A&gt;","synthetic":false,"types":["corundum::prc::VWeak"]},{"text":"impl&lt;T:&nbsp;<a class=\"trait\" href=\"corundum/trait.PSafe.html\" title=\"trait corundum::PSafe\">PSafe</a> + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>, A:&nbsp;<a class=\"trait\" href=\"corundum/alloc/trait.MemPool.html\" title=\"trait corundum::alloc::MemPool\">MemPool</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"corundum/sync/struct.VWeak.html\" title=\"struct corundum::sync::VWeak\">VWeak</a>&lt;T, A&gt;","synthetic":false,"types":["corundum::sync::parc::VWeak"]},{"text":"impl&lt;A:&nbsp;<a class=\"trait\" href=\"corundum/alloc/trait.MemPool.html\" title=\"trait corundum::alloc::MemPool\">MemPool</a>, T:&nbsp;<a class=\"trait\" href=\"corundum/trait.PSafe.html\" title=\"trait corundum::PSafe\">PSafe</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"corundum/ptr/struct.Slice.html\" title=\"struct corundum::ptr::Slice\">Slice</a>&lt;T, A&gt;","synthetic":false,"types":["corundum::ptr::slice::Slice"]},{"text":"impl&lt;A:&nbsp;<a class=\"trait\" href=\"corundum/alloc/trait.MemPool.html\" title=\"trait corundum::alloc::MemPool\">MemPool</a>, T:&nbsp;<a class=\"trait\" href=\"corundum/trait.PSafe.html\" title=\"trait corundum::PSafe\">PSafe</a> + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"corundum/ptr/struct.Ptr.html\" title=\"struct corundum::ptr::Ptr\">Ptr</a>&lt;T, A&gt;","synthetic":false,"types":["corundum::ptr::ptr::Ptr"]},{"text":"impl&lt;T:&nbsp;<a class=\"trait\" href=\"corundum/trait.PSafe.html\" title=\"trait corundum::PSafe\">PSafe</a> + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"corundum/ptr/struct.NonNull.html\" title=\"struct corundum::ptr::NonNull\">NonNull</a>&lt;T&gt;","synthetic":false,"types":["corundum::ptr::non_null::NonNull"]},{"text":"impl&lt;T:&nbsp;<a class=\"trait\" href=\"corundum/trait.PSafe.html\" title=\"trait corundum::PSafe\">PSafe</a> + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>, A:&nbsp;<a class=\"trait\" href=\"corundum/alloc/trait.MemPool.html\" title=\"trait corundum::alloc::MemPool\">MemPool</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"corundum/ptr/struct.LogNonNull.html\" title=\"struct corundum::ptr::LogNonNull\">LogNonNull</a>&lt;T, A&gt;","synthetic":false,"types":["corundum::ptr::non_null::LogNonNull"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"enum\" href=\"corundum/stm/enum.LogEnum.html\" title=\"enum corundum::stm::LogEnum\">LogEnum</a>","synthetic":false,"types":["corundum::stm::log::LogEnum"]},{"text":"impl&lt;A:&nbsp;<a class=\"trait\" href=\"corundum/alloc/trait.MemPool.html\" title=\"trait corundum::alloc::MemPool\">MemPool</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"enum\" href=\"corundum/stm/enum.Notifier.html\" title=\"enum corundum::stm::Notifier\">Notifier</a>&lt;A&gt;","synthetic":false,"types":["corundum::stm::log::Notifier"]},{"text":"impl&lt;A:&nbsp;<a class=\"trait\" href=\"corundum/alloc/trait.MemPool.html\" title=\"trait corundum::alloc::MemPool\">MemPool</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"corundum/stm/struct.Log.html\" title=\"struct corundum::stm::Log\">Log</a>&lt;A&gt;","synthetic":false,"types":["corundum::stm::log::Log"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()