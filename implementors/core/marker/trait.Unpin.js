(function() {var implementors = {};
implementors["corundum"] = [{"text":"impl Unpin for BuddyAlloc","synthetic":true,"types":[]},{"text":"impl&lt;A&gt; Unpin for BuddyAlg&lt;A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T, A&gt; Unpin for Zones&lt;T, A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: Unpin,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Unpin for Heap","synthetic":true,"types":[]},{"text":"impl&lt;'a, T, A&gt; Unpin for RootCell&lt;'a, T, A&gt;","synthetic":true,"types":[]},{"text":"impl&lt;T:&nbsp;?Sized, A&gt; Unpin for Ptr&lt;T, A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: Unpin,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T:&nbsp;?Sized, A&gt; Unpin for PCell&lt;T, A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: Unpin,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T:&nbsp;?Sized, A&gt; Unpin for PRefCell&lt;T, A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: Unpin,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;'b, T:&nbsp;?Sized, A&gt; Unpin for Ref&lt;'b, T, A&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'b, T:&nbsp;?Sized, A&gt; Unpin for RefMut&lt;'b, T, A&gt;","synthetic":true,"types":[]},{"text":"impl&lt;T, A&gt; Unpin for VCell&lt;T, A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: Unpin,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T:&nbsp;?Sized, A&gt; Unpin for PrcBox&lt;T, A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: Unpin,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T:&nbsp;?Sized, A&gt; Unpin for Weak&lt;T, A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: Unpin,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T:&nbsp;?Sized, A&gt; Unpin for VWeak&lt;T, A&gt;","synthetic":true,"types":[]},{"text":"impl&lt;T, A&gt; Unpin for PMutex&lt;T, A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: Unpin,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;'a, T, A&gt; Unpin for MutexGuard&lt;'a, T, A&gt;","synthetic":true,"types":[]},{"text":"impl&lt;T:&nbsp;?Sized, A&gt; Unpin for ParcInner&lt;T, A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: Unpin,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T:&nbsp;?Sized, A&gt; Unpin for Weak&lt;T, A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: Unpin,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T:&nbsp;?Sized, A&gt; Unpin for VWeak&lt;T, A&gt;","synthetic":true,"types":[]},{"text":"impl&lt;T, A&gt; Unpin for FatPtr&lt;T, A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: Unpin,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T:&nbsp;?Sized&gt; Unpin for NonNull&lt;T&gt;","synthetic":true,"types":[]},{"text":"impl&lt;T:&nbsp;?Sized, A&gt; Unpin for LogNonNull&lt;T, A&gt;","synthetic":true,"types":[]},{"text":"impl Unpin for Chaperon","synthetic":true,"types":[]},{"text":"impl&lt;A&gt; Unpin for Journal&lt;A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Unpin for LogEnum","synthetic":true,"types":[]},{"text":"impl&lt;A&gt; Unpin for Notifier&lt;A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;A&gt; Unpin for Log&lt;A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;A&gt; Unpin for String&lt;A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T, A&gt; Unpin for Vec&lt;T, A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: Unpin,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Unpin for IntoIteratorHelper&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;'a, T&gt; Unpin for IterHelper&lt;'a, T&gt;","synthetic":true,"types":[]},{"text":"impl&lt;A&gt; Unpin for Measure&lt;A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Unpin for AssertTxInSafe&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T:&nbsp;PSafe + ?Sized, A:&nbsp;MemPool&gt; Unpin for Pbox&lt;T, A&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;PSafe + ?Sized, A:&nbsp;MemPool&gt; Unpin for Prc&lt;T, A&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;PSafe + ?Sized, A:&nbsp;MemPool&gt; Unpin for Parc&lt;T, A&gt;","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()