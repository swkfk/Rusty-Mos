(function() {var type_impls = {
"rusty_mos":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-LinkList%3CT%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/rusty_mos/utils/linked_list.rs.html#20\">source</a><a href=\"#impl-Clone-for-LinkList%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T: Clone + Copy&gt; Clone for <a class=\"struct\" href=\"rusty_mos/utils/linked_list/struct.LinkList.html\" title=\"struct rusty_mos::utils::linked_list::LinkList\">LinkList</a>&lt;T&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/rusty_mos/utils/linked_list.rs.html#20\">source</a><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">clone</a>(&amp;self) -&gt; <a class=\"struct\" href=\"rusty_mos/utils/linked_list/struct.LinkList.html\" title=\"struct rusty_mos::utils::linked_list::LinkList\">LinkList</a>&lt;T&gt;</h4></section></summary><div class='docblock'>Returns a copy of the value. <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"since rightside\" title=\"Stable since Rust version 1.0.0\">1.0.0</span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">clone_from</a>(&amp;mut self, source: &amp;Self)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a>Read more</a></div></details></div></details>","Clone","rusty_mos::memory::pmap::PageList"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Default-for-LinkList%3CT%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/rusty_mos/utils/linked_list.rs.html#45-50\">source</a><a href=\"#impl-Default-for-LinkList%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T: Copy&gt; Default for <a class=\"struct\" href=\"rusty_mos/utils/linked_list/struct.LinkList.html\" title=\"struct rusty_mos::utils::linked_list::LinkList\">LinkList</a>&lt;T&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.default\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/rusty_mos/utils/linked_list.rs.html#47-49\">source</a><a href=\"#method.default\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">default</a>() -&gt; Self</h4></section></summary><div class=\"docblock\"><p>Constructor for the default.</p>\n</div></details></div></details>","Default","rusty_mos::memory::pmap::PageList"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-LinkList%3CT%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/rusty_mos/utils/linked_list.rs.html#52-113\">source</a><a href=\"#impl-LinkList%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T: Copy&gt; <a class=\"struct\" href=\"rusty_mos/utils/linked_list/struct.LinkList.html\" title=\"struct rusty_mos::utils::linked_list::LinkList\">LinkList</a>&lt;T&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.new\" class=\"method\"><a class=\"src rightside\" href=\"src/rusty_mos/utils/linked_list.rs.html#54-58\">source</a><h4 class=\"code-header\">pub const fn <a href=\"rusty_mos/utils/linked_list/struct.LinkList.html#tymethod.new\" class=\"fn\">new</a>() -&gt; <a class=\"struct\" href=\"rusty_mos/utils/linked_list/struct.LinkList.html\" title=\"struct rusty_mos::utils::linked_list::LinkList\">LinkList</a>&lt;T&gt;</h4></section></summary><div class=\"docblock\"><p>Create an empty link list with its head null.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.empty\" class=\"method\"><a class=\"src rightside\" href=\"src/rusty_mos/utils/linked_list.rs.html#61-63\">source</a><h4 class=\"code-header\">pub fn <a href=\"rusty_mos/utils/linked_list/struct.LinkList.html#tymethod.empty\" class=\"fn\">empty</a>(&amp;self) -&gt; bool</h4></section></summary><div class=\"docblock\"><p>Judge whether this list is empty.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.insert_head\" class=\"method\"><a class=\"src rightside\" href=\"src/rusty_mos/utils/linked_list.rs.html#69-80\">source</a><h4 class=\"code-header\">pub fn <a href=\"rusty_mos/utils/linked_list/struct.LinkList.html#tymethod.insert_head\" class=\"fn\">insert_head</a>(&amp;mut self, item: *mut <a class=\"struct\" href=\"rusty_mos/utils/linked_list/struct.LinkNode.html\" title=\"struct rusty_mos::utils::linked_list::LinkNode\">LinkNode</a>&lt;T&gt;)</h4></section></summary><div class=\"docblock\"><p>Insert a node to the head of the list</p>\n<h5 id=\"safety\"><a class=\"doc-anchor\" href=\"#safety\">§</a>Safety</h5>\n<p>The parameter <code>item</code> <em>SHALL</em> be mutably-visitable!</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.pop_head\" class=\"method\"><a class=\"src rightside\" href=\"src/rusty_mos/utils/linked_list.rs.html#88-97\">source</a><h4 class=\"code-header\">pub fn <a href=\"rusty_mos/utils/linked_list/struct.LinkList.html#tymethod.pop_head\" class=\"fn\">pop_head</a>(&amp;mut self) -&gt; Option&lt;*mut <a class=\"struct\" href=\"rusty_mos/utils/linked_list/struct.LinkNode.html\" title=\"struct rusty_mos::utils::linked_list::LinkNode\">LinkNode</a>&lt;T&gt;&gt;</h4></section></summary><div class=\"docblock\"><p>Get the first node of this list and removce it</p>\n<p>The return value will be <code>None</code> is the list is empty.</p>\n<h5 id=\"safety-1\"><a class=\"doc-anchor\" href=\"#safety-1\">§</a>Safety</h5>\n<p>All things in the list <em>SHALL</em> be valid!</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.remove\" class=\"method\"><a class=\"src rightside\" href=\"src/rusty_mos/utils/linked_list.rs.html#104-112\">source</a><h4 class=\"code-header\">pub fn <a href=\"rusty_mos/utils/linked_list/struct.LinkList.html#tymethod.remove\" class=\"fn\">remove</a>(item: *mut <a class=\"struct\" href=\"rusty_mos/utils/linked_list/struct.LinkNode.html\" title=\"struct rusty_mos::utils::linked_list::LinkNode\">LinkNode</a>&lt;T&gt;)</h4></section></summary><div class=\"docblock\"><p>Remove a specified node from the list contains this node.</p>\n<h5 id=\"safety-2\"><a class=\"doc-anchor\" href=\"#safety-2\">§</a>Safety</h5>\n<p>The parameter <code>item</code> <em>SHALL</em> be mutably-visitable and <em>SHALL</em> be in an\nvalid link list!</p>\n</div></details></div></details>",0,"rusty_mos::memory::pmap::PageList"],["<section id=\"impl-Copy-for-LinkList%3CT%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/rusty_mos/utils/linked_list.rs.html#20\">source</a><a href=\"#impl-Copy-for-LinkList%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T: Copy + Copy&gt; Copy for <a class=\"struct\" href=\"rusty_mos/utils/linked_list/struct.LinkList.html\" title=\"struct rusty_mos::utils::linked_list::LinkList\">LinkList</a>&lt;T&gt;</h3></section>","Copy","rusty_mos::memory::pmap::PageList"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()