(function() {var implementors = {
"lock_api":[["impl !Send for <a class=\"struct\" href=\"lock_api/struct.GuardNoSend.html\" title=\"struct lock_api::GuardNoSend\">GuardNoSend</a>",1,["lock_api::GuardNoSend"]],["impl Send for <a class=\"struct\" href=\"lock_api/struct.GuardSend.html\" title=\"struct lock_api::GuardSend\">GuardSend</a>",1,["lock_api::GuardSend"]],["impl&lt;'a, R, G, T&gt; !Send for <a class=\"struct\" href=\"lock_api/struct.MappedReentrantMutexGuard.html\" title=\"struct lock_api::MappedReentrantMutexGuard\">MappedReentrantMutexGuard</a>&lt;'a, R, G, T&gt;",1,["lock_api::remutex::MappedReentrantMutexGuard"]],["impl&lt;'a, R, G, T&gt; !Send for <a class=\"struct\" href=\"lock_api/struct.ReentrantMutexGuard.html\" title=\"struct lock_api::ReentrantMutexGuard\">ReentrantMutexGuard</a>&lt;'a, R, G, T&gt;",1,["lock_api::remutex::ReentrantMutexGuard"]],["impl&lt;'a, R, T&gt; Send for <a class=\"struct\" href=\"lock_api/struct.MutexGuard.html\" title=\"struct lock_api::MutexGuard\">MutexGuard</a>&lt;'a, R, T&gt;<div class=\"where\">where\n    R: Sync,\n    T: Send + ?Sized,\n    &lt;R as <a class=\"trait\" href=\"lock_api/trait.RawMutex.html\" title=\"trait lock_api::RawMutex\">RawMutex</a>&gt;::<a class=\"associatedtype\" href=\"lock_api/trait.RawMutex.html#associatedtype.GuardMarker\" title=\"type lock_api::RawMutex::GuardMarker\">GuardMarker</a>: Send,</div>",1,["lock_api::mutex::MutexGuard"]],["impl&lt;'a, R, T&gt; Send for <a class=\"struct\" href=\"lock_api/struct.RwLockReadGuard.html\" title=\"struct lock_api::RwLockReadGuard\">RwLockReadGuard</a>&lt;'a, R, T&gt;<div class=\"where\">where\n    R: Sync,\n    T: Send + Sync + ?Sized,\n    &lt;R as <a class=\"trait\" href=\"lock_api/trait.RawRwLock.html\" title=\"trait lock_api::RawRwLock\">RawRwLock</a>&gt;::<a class=\"associatedtype\" href=\"lock_api/trait.RawRwLock.html#associatedtype.GuardMarker\" title=\"type lock_api::RawRwLock::GuardMarker\">GuardMarker</a>: Send,</div>",1,["lock_api::rwlock::RwLockReadGuard"]],["impl&lt;'a, R, T&gt; Send for <a class=\"struct\" href=\"lock_api/struct.RwLockUpgradableReadGuard.html\" title=\"struct lock_api::RwLockUpgradableReadGuard\">RwLockUpgradableReadGuard</a>&lt;'a, R, T&gt;<div class=\"where\">where\n    R: Sync,\n    T: Send + Sync + ?Sized,\n    &lt;R as <a class=\"trait\" href=\"lock_api/trait.RawRwLock.html\" title=\"trait lock_api::RawRwLock\">RawRwLock</a>&gt;::<a class=\"associatedtype\" href=\"lock_api/trait.RawRwLock.html#associatedtype.GuardMarker\" title=\"type lock_api::RawRwLock::GuardMarker\">GuardMarker</a>: Send,</div>",1,["lock_api::rwlock::RwLockUpgradableReadGuard"]],["impl&lt;'a, R, T&gt; Send for <a class=\"struct\" href=\"lock_api/struct.RwLockWriteGuard.html\" title=\"struct lock_api::RwLockWriteGuard\">RwLockWriteGuard</a>&lt;'a, R, T&gt;<div class=\"where\">where\n    R: Sync,\n    T: Send + Sync + ?Sized,\n    &lt;R as <a class=\"trait\" href=\"lock_api/trait.RawRwLock.html\" title=\"trait lock_api::RawRwLock\">RawRwLock</a>&gt;::<a class=\"associatedtype\" href=\"lock_api/trait.RawRwLock.html#associatedtype.GuardMarker\" title=\"type lock_api::RawRwLock::GuardMarker\">GuardMarker</a>: Send,</div>",1,["lock_api::rwlock::RwLockWriteGuard"]],["impl&lt;'a, R: <a class=\"trait\" href=\"lock_api/trait.RawMutex.html\" title=\"trait lock_api::RawMutex\">RawMutex</a> + 'a, T: ?Sized + Send + 'a&gt; Send for <a class=\"struct\" href=\"lock_api/struct.MappedMutexGuard.html\" title=\"struct lock_api::MappedMutexGuard\">MappedMutexGuard</a>&lt;'a, R, T&gt;<div class=\"where\">where\n    R::<a class=\"associatedtype\" href=\"lock_api/trait.RawMutex.html#associatedtype.GuardMarker\" title=\"type lock_api::RawMutex::GuardMarker\">GuardMarker</a>: Send,</div>"],["impl&lt;'a, R: <a class=\"trait\" href=\"lock_api/trait.RawRwLock.html\" title=\"trait lock_api::RawRwLock\">RawRwLock</a> + 'a, T: ?Sized + Send + 'a&gt; Send for <a class=\"struct\" href=\"lock_api/struct.MappedRwLockWriteGuard.html\" title=\"struct lock_api::MappedRwLockWriteGuard\">MappedRwLockWriteGuard</a>&lt;'a, R, T&gt;<div class=\"where\">where\n    R::<a class=\"associatedtype\" href=\"lock_api/trait.RawRwLock.html#associatedtype.GuardMarker\" title=\"type lock_api::RawRwLock::GuardMarker\">GuardMarker</a>: Send,</div>"],["impl&lt;'a, R: <a class=\"trait\" href=\"lock_api/trait.RawRwLock.html\" title=\"trait lock_api::RawRwLock\">RawRwLock</a> + 'a, T: ?Sized + Sync + 'a&gt; Send for <a class=\"struct\" href=\"lock_api/struct.MappedRwLockReadGuard.html\" title=\"struct lock_api::MappedRwLockReadGuard\">MappedRwLockReadGuard</a>&lt;'a, R, T&gt;<div class=\"where\">where\n    R::<a class=\"associatedtype\" href=\"lock_api/trait.RawRwLock.html#associatedtype.GuardMarker\" title=\"type lock_api::RawRwLock::GuardMarker\">GuardMarker</a>: Send,</div>"],["impl&lt;R: <a class=\"trait\" href=\"lock_api/trait.RawMutex.html\" title=\"trait lock_api::RawMutex\">RawMutex</a> + Send, G: <a class=\"trait\" href=\"lock_api/trait.GetThreadId.html\" title=\"trait lock_api::GetThreadId\">GetThreadId</a> + Send&gt; Send for <a class=\"struct\" href=\"lock_api/struct.RawReentrantMutex.html\" title=\"struct lock_api::RawReentrantMutex\">RawReentrantMutex</a>&lt;R, G&gt;"],["impl&lt;R: <a class=\"trait\" href=\"lock_api/trait.RawMutex.html\" title=\"trait lock_api::RawMutex\">RawMutex</a> + Send, G: <a class=\"trait\" href=\"lock_api/trait.GetThreadId.html\" title=\"trait lock_api::GetThreadId\">GetThreadId</a> + Send, T: ?Sized + Send&gt; Send for <a class=\"struct\" href=\"lock_api/struct.ReentrantMutex.html\" title=\"struct lock_api::ReentrantMutex\">ReentrantMutex</a>&lt;R, G, T&gt;"],["impl&lt;R: <a class=\"trait\" href=\"lock_api/trait.RawMutex.html\" title=\"trait lock_api::RawMutex\">RawMutex</a> + Send, T: ?Sized + Send&gt; Send for <a class=\"struct\" href=\"lock_api/struct.Mutex.html\" title=\"struct lock_api::Mutex\">Mutex</a>&lt;R, T&gt;"],["impl&lt;R: <a class=\"trait\" href=\"lock_api/trait.RawRwLock.html\" title=\"trait lock_api::RawRwLock\">RawRwLock</a> + Send, T: ?Sized + Send&gt; Send for <a class=\"struct\" href=\"lock_api/struct.RwLock.html\" title=\"struct lock_api::RwLock\">RwLock</a>&lt;R, T&gt;"]],
"rusty_mos":[["impl !Send for <a class=\"struct\" href=\"rusty_mos/kdef/env/struct.EnvData.html\" title=\"struct rusty_mos::kdef::env::EnvData\">EnvData</a>",1,["rusty_mos::kdef::env::EnvData"]],["impl Send for <a class=\"enum\" href=\"rusty_mos/kdef/env/enum.EnvStatus.html\" title=\"enum rusty_mos::kdef::env::EnvStatus\">EnvStatus</a>",1,["rusty_mos::kdef::env::EnvStatus"]],["impl Send for <a class=\"enum\" href=\"rusty_mos/kdef/error/enum.KError.html\" title=\"enum rusty_mos::kdef::error::KError\">KError</a>",1,["rusty_mos::kdef::error::KError"]],["impl Send for <a class=\"enum\" href=\"rusty_mos/kdef/error/enum.UError.html\" title=\"enum rusty_mos::kdef::error::UError\">UError</a>",1,["rusty_mos::kdef::error::UError"]],["impl Send for <a class=\"enum\" href=\"rusty_mos/kdef/syscall/enum.SyscallNo.html\" title=\"enum rusty_mos::kdef::syscall::SyscallNo\">SyscallNo</a>",1,["rusty_mos::kdef::syscall::SyscallNo"]],["impl Send for <a class=\"struct\" href=\"rusty_mos/kdef/elf/struct.Elf32Ehdr.html\" title=\"struct rusty_mos::kdef::elf::Elf32Ehdr\">Elf32Ehdr</a>",1,["rusty_mos::kdef::elf::Elf32Ehdr"]],["impl Send for <a class=\"struct\" href=\"rusty_mos/kdef/elf/struct.Elf32Phdr.html\" title=\"struct rusty_mos::kdef::elf::Elf32Phdr\">Elf32Phdr</a>",1,["rusty_mos::kdef::elf::Elf32Phdr"]],["impl Send for <a class=\"struct\" href=\"rusty_mos/kdef/env/struct.IpcData.html\" title=\"struct rusty_mos::kdef::env::IpcData\">IpcData</a>",1,["rusty_mos::kdef::env::IpcData"]],["impl Send for <a class=\"struct\" href=\"rusty_mos/kern/trap/struct.TrapFrame.html\" title=\"struct rusty_mos::kern::trap::TrapFrame\">TrapFrame</a>",1,["rusty_mos::kern::trap::TrapFrame"]],["impl Send for <a class=\"struct\" href=\"rusty_mos/memory/pmap/struct.PageData.html\" title=\"struct rusty_mos::memory::pmap::PageData\">PageData</a>",1,["rusty_mos::memory::pmap::PageData"]],["impl&lt;T&gt; !Send for <a class=\"struct\" href=\"rusty_mos/utils/linked_list/struct.LinkList.html\" title=\"struct rusty_mos::utils::linked_list::LinkList\">LinkList</a>&lt;T&gt;",1,["rusty_mos::utils::linked_list::LinkList"]],["impl&lt;T&gt; !Send for <a class=\"struct\" href=\"rusty_mos/utils/linked_list/struct.LinkNode.html\" title=\"struct rusty_mos::utils::linked_list::LinkNode\">LinkNode</a>&lt;T&gt;",1,["rusty_mos::utils::linked_list::LinkNode"]],["impl&lt;T&gt; !Send for <a class=\"struct\" href=\"rusty_mos/utils/linked_list/struct.TailLinkList.html\" title=\"struct rusty_mos::utils::linked_list::TailLinkList\">TailLinkList</a>&lt;T&gt;",1,["rusty_mos::utils::linked_list::TailLinkList"]],["impl&lt;T&gt; Send for <a class=\"struct\" href=\"rusty_mos/kern/env/struct.EnvsWrapper.html\" title=\"struct rusty_mos::kern::env::EnvsWrapper\">EnvsWrapper</a>&lt;T&gt;<div class=\"where\">where\n    T: Send,</div>",1,["rusty_mos::kern::env::EnvsWrapper"]],["impl&lt;const CCOUNT: usize&gt; Send for <a class=\"struct\" href=\"rusty_mos/memory/buddy_allocator/struct.BuddyAllocator.html\" title=\"struct rusty_mos::memory::buddy_allocator::BuddyAllocator\">BuddyAllocator</a>&lt;CCOUNT&gt;",1,["rusty_mos::memory::buddy_allocator::BuddyAllocator"]],["impl&lt;const COUNT: usize&gt; Send for <a class=\"struct\" href=\"rusty_mos/utils/bitmap/struct.Bitmap.html\" title=\"struct rusty_mos::utils::bitmap::Bitmap\">Bitmap</a>&lt;COUNT&gt;",1,["rusty_mos::utils::bitmap::Bitmap"]]],
"scopeguard":[["impl Send for <a class=\"enum\" href=\"scopeguard/enum.Always.html\" title=\"enum scopeguard::Always\">Always</a>",1,["scopeguard::Always"]],["impl&lt;T, F, S&gt; Send for <a class=\"struct\" href=\"scopeguard/struct.ScopeGuard.html\" title=\"struct scopeguard::ScopeGuard\">ScopeGuard</a>&lt;T, F, S&gt;<div class=\"where\">where\n    T: Send,\n    F: Send,</div>",1,["scopeguard::ScopeGuard"]]],
"spin":[["impl Send for <a class=\"struct\" href=\"spin/barrier/struct.BarrierWaitResult.html\" title=\"struct spin::barrier::BarrierWaitResult\">BarrierWaitResult</a>",1,["spin::barrier::BarrierWaitResult"]],["impl Send for <a class=\"struct\" href=\"spin/relax/struct.Loop.html\" title=\"struct spin::relax::Loop\">Loop</a>",1,["spin::relax::Loop"]],["impl Send for <a class=\"struct\" href=\"spin/relax/struct.Spin.html\" title=\"struct spin::relax::Spin\">Spin</a>",1,["spin::relax::Spin"]],["impl&lt;'a, T&gt; Send for <a class=\"struct\" href=\"spin/mutex/struct.MutexGuard.html\" title=\"struct spin::mutex::MutexGuard\">MutexGuard</a>&lt;'a, T&gt;<div class=\"where\">where\n    T: Send + ?Sized,</div>",1,["spin::mutex::MutexGuard"]],["impl&lt;R&gt; Send for <a class=\"struct\" href=\"spin/barrier/struct.Barrier.html\" title=\"struct spin::barrier::Barrier\">Barrier</a>&lt;R&gt;",1,["spin::barrier::Barrier"]],["impl&lt;T, F, R&gt; Send for <a class=\"struct\" href=\"spin/lazy/struct.Lazy.html\" title=\"struct spin::lazy::Lazy\">Lazy</a>&lt;T, F, R&gt;<div class=\"where\">where\n    T: Send,\n    F: Send,</div>",1,["spin::lazy::Lazy"]],["impl&lt;T: ?Sized + Send + Sync, R&gt; Send for <a class=\"struct\" href=\"spin/rwlock/struct.RwLockUpgradableGuard.html\" title=\"struct spin::rwlock::RwLockUpgradableGuard\">RwLockUpgradableGuard</a>&lt;'_, T, R&gt;"],["impl&lt;T: ?Sized + Send + Sync, R&gt; Send for <a class=\"struct\" href=\"spin/rwlock/struct.RwLockWriteGuard.html\" title=\"struct spin::rwlock::RwLockWriteGuard\">RwLockWriteGuard</a>&lt;'_, T, R&gt;"],["impl&lt;T: ?Sized + Send&gt; Send for <a class=\"struct\" href=\"spin/mutex/spin/struct.SpinMutexGuard.html\" title=\"struct spin::mutex::spin::SpinMutexGuard\">SpinMutexGuard</a>&lt;'_, T&gt;"],["impl&lt;T: ?Sized + Send, R&gt; Send for <a class=\"struct\" href=\"spin/mutex/spin/struct.SpinMutex.html\" title=\"struct spin::mutex::spin::SpinMutex\">SpinMutex</a>&lt;T, R&gt;"],["impl&lt;T: ?Sized + Send, R&gt; Send for <a class=\"struct\" href=\"spin/mutex/struct.Mutex.html\" title=\"struct spin::mutex::Mutex\">Mutex</a>&lt;T, R&gt;"],["impl&lt;T: ?Sized + Send, R&gt; Send for <a class=\"struct\" href=\"spin/rwlock/struct.RwLock.html\" title=\"struct spin::rwlock::RwLock\">RwLock</a>&lt;T, R&gt;"],["impl&lt;T: ?Sized + Sync&gt; Send for <a class=\"struct\" href=\"spin/rwlock/struct.RwLockReadGuard.html\" title=\"struct spin::rwlock::RwLockReadGuard\">RwLockReadGuard</a>&lt;'_, T&gt;"],["impl&lt;T: Send, R&gt; Send for <a class=\"struct\" href=\"spin/once/struct.Once.html\" title=\"struct spin::once::Once\">Once</a>&lt;T, R&gt;"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()