var searchIndex = new Map(JSON.parse('[\
["rusty_mos",{"t":"JQQQQQQQQQQCCQQCCCQQQQQQCCQCCCCCSSSSSSSSHHHHSPPPPPPPPPPPPPPPPPPPPPPGNNNNNNNFSIINNHNNNHHHHHHHHHHHHHHHHHHHHHHNNNNJFNNNNNNNOOOONHHNNHHHHHHONOOHNNNNCCCSSSSSSSSSSSSSSSSSSPPPPPGPPPPPPPPPPPPGPNNNNNNNNNNNNNNNQQHCCCHHHHCFHHNNNNNNNNCCCCCCFFNNNNNNNNNNONNNNNNNNONNNNNNJJJJJSFIIIIHNNNNNNONONHHOHHHHHHHHOONNNNSSSSSSSSSSSSSSSSSSSSSSSSJFFJNNNNNNNNNNOONNNNNNNNHNNOOONNNNNNNNNOHHHHHCCCSIFIIFIISSSSOOOOOOOOOONNNNNNNNNNNNHOOONNNNOONNOOOOOONNNNNNNNOJJJJJJJFGFPFSSPPOOOHHNNNNNNNNNNNNNNNNNNNNNNNOHHHHHHHOHHNNNNNNOONNNNOHHHNOOOOOONNNONNNNNNNNNNNNOOJHCCCCCCFFFONNNNNNNNNNNNNNNNNNONNNNNNNONNONONNNNNNNNNNFNONNNNNNNNNNONNNOHHFFNNNNNNNNNNNNONNNNONNNNNONONNNNNNNNNFNNNNNNNNNN","n":["BUDDY_ALLOCATOR","ENVX","GEN_MASK","KADDR","PADDR","PDX","PPN","PTE_ADDR","PTX","ROUND","ROUNDDOWN","arch_mipsel","consts","debug","debugln","kernel_tests","library","memory","pa2page","page2kva","page2pa","page2ppn","print","println","process","utils","va2pa","machine","panic","syscall","syscall_impl","trap","KSEG1","MALTA_FPGA_BASE","MALTA_PCIIO_BASE","MALTA_SERIAL_BASE","MALTA_SERIAL_DATA","MALTA_SERIAL_DATA_READY","MALTA_SERIAL_LSR","MALTA_SERIAL_THR_EMPTY","halt","print_charc","scan_charc","panic","MAX_SYS_NO","SysBindPool","SysCgetc","SysCreatePool","SysEnvDestroy","SysExofork","SysGetenvid","SysIpcRecv","SysIpcTrySend","SysLock","SysMemAlloc","SysMemMap","SysMemUnmap","SysPanic","SysPrintCons","SysPutchar","SysReadDev","SysSetEnvStatus","SysSetTlbModEntry","SysSetTrapframe","SysUnlock","SysWriteDev","SysYield","SyscallNo","borrow","borrow_mut","from","into","try_from","try_into","type_id","CLikeStr","SYSCALL_TABLE","SyscallFn","SyscallRawPtr","borrow","borrow_mut","do_syscall","fmt","from","into","sys_bind_shared_pool","sys_cgetc","sys_create_shared_pool","sys_env_destroy","sys_exofork","sys_getenvid","sys_ipc_recv","sys_ipc_try_send","sys_lock","sys_mem_alloc","sys_mem_map","sys_mem_unmap","sys_panic","sys_print_cons","sys_putchar","sys_read_dev","sys_set_env_status","sys_set_tlb_mod_entry","sys_set_trapframe","sys_unlock","sys_write_dev","sys_yield","to_string","try_from","try_into","type_id","EXCEPTION_HANDLERS","TrapFrame","borrow","borrow_mut","clone","clone_into","clone_to_uninit","clone_to_uninit","const_construct","cp0_badvaddr","cp0_cause","cp0_epc","cp0_status","default","do_reserved","do_skip","fmt","from","handle_int","handle_mod","handle_reserved","handle_skip","handle_sys","handle_tlb","hi","into","lo","regs","set_exc_base","to_owned","try_from","try_into","type_id","cp0reg","error","types","STATUS_BEV","STATUS_CU0","STATUS_CU1","STATUS_CU2","STATUS_CU3","STATUS_ERL","STATUS_EXL","STATUS_IE","STATUS_IM0","STATUS_IM1","STATUS_IM2","STATUS_IM3","STATUS_IM4","STATUS_IM5","STATUS_IM6","STATUS_IM7","STATUS_R0","STATUS_UM","BadEnv","BadPath","FileExists","Invalid","IpcNotRecv","KError","LockByOthers","MaxOpen","NoDisk","NoFreeEnv","NoLock","NoMem","NoSys","NotExec","NotFound","PoolDoubleBind","PoolNotBind","PoolNotFound","UError","Unspecified","borrow","borrow","borrow_mut","borrow_mut","fmt","from","from","into","into","try_from","try_from","try_into","try_into","type_id","type_id","TEST_CALL","TEST_FN","slash_print","test_array_link_list","test_buddy_alloc","test_memory_pool","unit_test","test","test","test","print","Stdout","_print","_write_str","borrow","borrow_mut","from","into","try_from","try_into","type_id","write_str","buddy_allocator","marcos","pmap","regions","shared_pool","tlbex","BuddyAllocator","BuddyInner","alloc","alloc","borrow","borrow","borrow_mut","borrow_mut","dealloc","dealloc","default","default","free_list","from","from","init","init","into","into","new","new","page_start","try_from","try_from","try_into","try_into","type_id","type_id","CUR_PGDIR","KERN_HEAP","NPAGE","PAGES","PAGE_FREE_LIST","PAGE_SIZE","PageData","PageList","PageNode","Pde","Pte","alloc","borrow","borrow_mut","clone","clone_into","clone_to_uninit","clone_to_uninit","data","from","head","into","mips_detect_memory","mips_vm_init","next","page_alloc","page_decref","page_free","page_init","page_insert","page_lookup","page_remove","pgdir_walk","pp_ref","prev","to_owned","try_from","try_into","type_id","KERNBASE","KSEG1","KSTACKTOP","NASID","PAGE_SIZE","PDMAP","PDSHIFT","PGSHIFT","PTE_C_CACHEABLE","PTE_D","PTE_G","PTE_HARDFLAG_SHIFT","PTE_V","PTMAP","UCOW","UENVS","ULIM","UPAGES","USTACKTOP","UTEMP","UTEXT","UTOP","UVPT","UXSTACKTOP","MEMORY_POOL","MemoryPool","MemoryPoolEntry","POOL_I","bind","borrow","borrow","borrow_mut","borrow_mut","crate_pool","default","default","deref","destory_env","envs","envs","fork_bind","from","from","insert_page","into","into","lock","lock","mkpoolid","new","new","pages","pools","reference","try_from","try_from","try_into","try_into","type_id","type_id","unbind","unlock","unlock","write_lock","_do_tlb_refill","do_tlb_mod","passive_alloc","tlb_invalidate","tlb_out","elf_loader","envs","scheduler","EI_NIDNET","Elf32Addr","Elf32Ehdr","Elf32Half","Elf32Off","Elf32Phdr","Elf32Word","ElfMapperFn","PF_R","PF_W","PF_X","PT_LOAD","_align","_ehsize","_flags","_machine","_paddr","_shentsize","_shnum","_shoff","_shstrndx","_version","borrow","borrow","borrow_mut","borrow_mut","clone","clone","clone_into","clone_into","clone_to_uninit","clone_to_uninit","clone_to_uninit","clone_to_uninit","elf_load_seg","entry","filesz","flags","foreach","from","from","from","ftype","ident","into","into","memsz","offset","phentsize","phnum","phoff","stype","to_owned","to_owned","try_from","try_from","try_into","try_into","type_id","type_id","vaddr","ASID_BMAP","BASE_PGDIR","CUR_ENV_IDX","ENVS_DATA","ENV_FREE_LIST","ENV_I","ENV_SCHE_LIST","EnvData","EnvStatus","EnvsWrapper","Free","IpcData","LOG2NENV","NENV","NotRunnable","Runnable","_place_holder_env_link","_place_holder_env_sched_link","asid","asid_alloc","asid_free","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","clone","clone","clone","clone_into","clone_into","clone_into","clone_to_uninit","clone_to_uninit","clone_to_uninit","clone_to_uninit","clone_to_uninit","clone_to_uninit","const_construct","default","default","dstva","env_alloc","env_create","env_destory","env_free","env_init","env_pop_tf","env_run","env_runs","env_setup_vm","envid2env","eq","fmt","from","from","from","from","from_id","id","into","into","into","into","ipc_data","load_icode","map_segment","mkenvid","new","parent_id","perm","pgdir","priority","receiving","status","to_owned","to_owned","to_owned","trap_frame","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","user_tlb_mod_entry","value","ENV_REST_COUNT","schedule","array_based_list","bitmap","bitops","io","linked_list","sync_ref_cell","Aligned","ArrayLinkNode","ArrayLinkedList","array","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","clone","clone_into","clone_to_uninit","clone_to_uninit","contains","default","default","empty","fmt","from","from","from","head","insert_head","insert_tail","into","into","into","new","new","next","peek_head","pop_head","prev","remove","tail","to_owned","try_from","try_from","try_from","try_into","try_into","try_into","type_id","type_id","type_id","Bitmap","alloc","bitmap","borrow","borrow_mut","default","empty","fmt","free","from","into","new","peek","pointer","try_from","try_into","type_id","used","ioread_into_va","iowrite_from_va","LinkList","LinkNode","borrow","borrow","borrow_mut","borrow_mut","clone","clone","clone_into","clone_into","clone_to_uninit","clone_to_uninit","clone_to_uninit","clone_to_uninit","data","default","empty","from","from","head","insert_head","into","into","new","new","next","pop_head","prev","remove","to_owned","to_owned","try_from","try_from","try_into","try_into","type_id","type_id","SyncImplRef","borrow","borrow","borrow_mut","borrow_mut","from","into","new","try_from","try_into","type_id"],"q":[[0,"rusty_mos"],[27,"rusty_mos::arch_mipsel"],[32,"rusty_mos::arch_mipsel::machine"],[43,"rusty_mos::arch_mipsel::panic"],[44,"rusty_mos::arch_mipsel::syscall"],[75,"rusty_mos::arch_mipsel::syscall_impl"],[111,"rusty_mos::arch_mipsel::trap"],[144,"rusty_mos::consts"],[147,"rusty_mos::consts::cp0reg"],[165,"rusty_mos::consts::error"],[200,"rusty_mos::kernel_tests"],[207,"rusty_mos::kernel_tests::test_array_link_list"],[208,"rusty_mos::kernel_tests::test_buddy_alloc"],[209,"rusty_mos::kernel_tests::test_memory_pool"],[210,"rusty_mos::library"],[211,"rusty_mos::library::print"],[222,"rusty_mos::memory"],[228,"rusty_mos::memory::buddy_allocator"],[256,"rusty_mos::memory::pmap"],[295,"rusty_mos::memory::regions"],[319,"rusty_mos::memory::shared_pool"],[359,"rusty_mos::memory::tlbex"],[364,"rusty_mos::process"],[367,"rusty_mos::process::elf_loader"],[428,"rusty_mos::process::envs"],[524,"rusty_mos::process::scheduler"],[526,"rusty_mos::utils"],[532,"rusty_mos::utils::array_based_list"],[578,"rusty_mos::utils::bitmap"],[596,"rusty_mos::utils::io"],[598,"rusty_mos::utils::linked_list"],[635,"rusty_mos::utils::sync_ref_cell"],[646,"core::panic::panic_info"],[647,"core::result"],[648,"core::any"],[649,"core::fmt"],[650,"alloc::string"],[651,"core::alloc::layout"],[652,"core::option"],[653,"alloc::vec"],[654,"core::ops::function"],[655,"core::clone"],[656,"core::marker"],[657,"core::cell"]],"i":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,52,52,52,52,52,52,52,52,52,52,52,52,52,52,52,52,52,52,52,52,52,52,0,52,52,52,52,52,52,52,0,0,0,0,10,10,0,10,10,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,10,10,10,10,0,0,9,9,9,9,9,9,9,9,9,9,9,9,0,0,9,9,0,0,0,0,0,0,9,9,9,9,0,9,9,9,9,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,16,53,53,16,16,0,16,53,53,16,16,16,16,53,53,16,16,16,0,16,53,16,53,16,16,53,16,53,16,53,16,53,16,53,16,0,0,0,0,0,0,0,0,0,0,0,0,0,0,19,19,19,19,19,19,19,19,0,0,0,0,0,0,0,0,20,22,20,22,20,22,20,22,20,22,20,20,22,20,22,20,22,20,22,20,20,22,20,22,20,22,0,0,0,0,0,0,0,0,0,0,0,0,26,26,26,26,26,26,23,26,54,26,0,0,23,0,0,0,0,0,0,0,0,26,23,26,26,26,26,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,31,33,31,33,31,31,33,31,33,31,33,31,31,33,31,31,33,31,33,31,0,33,31,33,31,33,33,31,33,31,33,31,31,33,31,33,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,36,35,35,35,36,35,35,35,35,35,35,36,35,36,35,36,35,36,35,35,36,36,0,35,36,36,35,35,35,36,35,35,35,36,36,36,35,35,35,36,35,36,35,36,35,36,35,36,36,0,0,0,0,0,0,0,0,0,0,14,0,0,0,14,14,40,40,40,0,0,55,14,39,40,55,14,39,40,14,39,40,14,39,40,14,14,39,39,40,40,39,14,40,39,0,0,0,0,0,0,0,40,0,0,14,14,55,14,39,40,39,40,55,14,39,40,40,0,0,0,40,40,39,40,40,39,40,14,39,40,40,55,14,39,40,55,14,39,40,55,14,39,40,40,39,0,0,0,0,0,0,0,0,0,0,0,42,56,42,41,56,42,41,41,41,41,41,42,42,41,42,41,56,42,41,42,42,42,56,42,41,42,41,41,42,42,41,42,42,41,56,42,41,56,42,41,56,42,41,0,43,43,43,43,43,43,43,43,43,43,43,43,43,43,43,43,43,0,0,0,0,44,47,44,47,44,47,44,47,44,44,47,47,47,44,44,44,47,44,44,44,47,44,47,47,44,47,44,44,47,44,47,44,47,44,47,0,48,48,48,48,48,48,48,48,48,48],"f":"````````````````````````````````````````{{}b}{df}{{}d}{{{j{h}}}b}````````````````````````{{{j{c}}}{{j{e}}}{}{}}{{{j{lc}}}{{j{le}}}{}{}}{cc{}}{ce{}{}}{c{{n{e}}}{}{}}0{{{j{c}}}A`{}}````54{Abf}{{{j{Ad}}{j{lAf}}}Ah}54{{AjAjAj}Aj}:0{AjAj}{{}Aj}01{{AjAjAjAj}Aj}23{{AjAjAjAjAj}Aj}{{AjAj}Aj}{db}{{dAj}Aj}{df}8{{AjAl}Aj}4{{AjAb}Aj}9:{{}b}{{{j{c}}}An{}}{c{{n{e}}}{}{}}0{{{j{c}}}A`{}}``{{{j{c}}}{{j{e}}}{}{}}{{{j{lc}}}{{j{le}}}{}{}}{{{j{Ab}}}Ab}{{{j{c}}{j{le}}}f{}{}}{{{j{c}}}f{}}0{{}Ab}````0{Abf}0{{{j{Ab}}{j{lAf}}}Ah}{cc{}}```````{ce{}{}}``{Ajf}{{{j{c}}}e{}{}}==<`````````````````````````````````````````;;::{{{j{B`}}{j{lAf}}}Ah}4433>>>>==``{{{j{Bb}}}f}```{{}f}000``{Bdf}2?>76{c{{n{e}}}{}{}}0{{{j{c}}}A`{}}{{{j{lBf}}{j{Bb}}}Ah}````````{{{j{lBh}}Bj}d}{{{j{Bl}}Bj}d}{{{j{c}}}{{j{e}}}{}{}}0{{{j{lc}}}{{j{le}}}{}{}}0{{{j{lBh}}dBj}f}{{{j{Bl}}dBj}f}{{}Bh}{{}Bl}`{cc{}}0{{{j{lBh}}BnC`}f}{{{j{Bl}}BnC`}f}{ce{}{}}054`>>>>==```````````{{{j{lC`}}C`C`C`Cb}Bn}:9{{{j{Cd}}}Cd}{{{j{c}}{j{le}}}f{}{}}{{{j{c}}}f{}}0`7`4{C`f}{{{j{lC`}}C`}f}`{{}{{n{BnB`}}}}{{{j{lBn}}}f}0{{{j{lC`}}}f}{{CfC`AjAjBn}{{n{fB`}}}}{{CfC`}{{Cl{{Cj{BnCh}}}}}}{{CfC`Aj}f}{{CfC`Cb}{{n{ChB`}}}}``{{{j{c}}}e{}{}}{c{{n{e}}}{}{}}0{{{j{c}}}A`{}}````````````````````````````{{{j{lCn}}C`C`}{{n{{j{{D`{C`}}}}B`}}}}{{{j{c}}}{{j{e}}}{}{}}0{{{j{lc}}}{{j{le}}}{}{}}0{{{j{lCn}}C`}C`}{{}Db}{{}Cn}{{{j{lDb}}C`}{{n{CbB`}}}}{{{j{lCn}}C`}f}``{{{j{lCn}}C`C`}f}{cc{}}0{{{j{lCn}}C`C`}{{n{fB`}}}}{ce{}{}}0{{{j{lDb}}C`}Cb}{{{j{lCn}}C`C`}{{n{CbB`}}}}{C`C`}:9```{c{{n{e}}}{}{}}000{{{j{c}}}A`{}}06{{{j{lDb}}C`}{{n{fB`}}}}7`{{{j{l{Dd{Aj}}}}C`Aj}f}{Abf}{{C`CfAj}f}{{AjC`}f}``````````````````````````{{{j{c}}}{{j{e}}}{}{}}0{{{j{lc}}}{{j{le}}}{}{}}0{{{j{Df}}}Df}{{{j{Dh}}}Dh}{{{j{c}}{j{le}}}f{}{}}0{{{j{c}}}f{}}000{{DhdDjC`}{{n{fB`}}}}```{{{j{Df}}c}f{{Dl{Aj}}}}{cc{}}{{dC`}}1``{ce{}{}}0``````{{{j{c}}}e{}{}}0{c{{n{e}}}{}{}}000{{{j{c}}}A`{}}0````````````````````{{}{{n{AjB`}}}}{Ajf}????>>>>{{{j{Al}}}Al}{{{j{Dn}}}Dn}{{{j{E`}}}E`}>>>======{{}Dn}{{}Al}{{}E`}`{Aj{{n{C`B`}}}}{{dC`Aj}{{Cl{C`}}}}{C`f}0{{}f}`{C`b}`{C`{{n{fB`}}}}{{AjCb}{{n{C`B`}}}}{{{j{Al}}{j{Al}}}Cb}{{{j{Al}}{j{lAf}}}Ah}{cc{}}000``{ce{}{}}000`{{C`dC`}f}{{CfAjC`C`C`Aj}f}{C`Aj}>``````{{{j{c}}}e{}{}}00`{c{{n{e}}}{}{}}0000000{{{j{c}}}A`{}}000```{Cbb}``````````{{{j{c}}}{{j{e}}}{}{}}00{{{j{lc}}}{{j{le}}}{}{}}00{{{j{Eb}}}Eb}{{{j{c}}{j{le}}}f{}{}}{{{j{c}}}f{}}0{{{j{Ed}}C`}Cb}{{}Ed}{{}Eb}{{{j{Ed}}}Cb}{{{j{Eb}}{j{lAf}}}Ah}{cc{}}00`{{{j{lEd}}C`}f}0{ce{}{}}0065`{{{j{Ed}}}{{Cl{C`}}}}{{{j{lEd}}}{{Cl{C`}}}}`3`{{{j{c}}}e{}{}}{c{{n{e}}}{}{}}00000{{{j{c}}}A`{}}00`{{{j{lEf}}}{{Cl{C`}}}}`{{{j{c}}}{{j{e}}}{}{}}{{{j{lc}}}{{j{le}}}{}{}}{{}Ef}{{{j{Ef}}}Cb}{{{j{Ef}}{j{lAf}}}Ah}{{{j{lEf}}C`}f}><3{{{j{Ef}}C`}Cb}`998`{{C`C`}f}0``7766{{{j{{Eh{c}}}}}{{Eh{c}}}{EjEl}}{{{j{{En{c}}}}}{{En{c}}}{EjEl}}{{{j{c}}{j{le}}}f{}{}}0{{{j{c}}}f{}}000`{{}{{Eh{c}}}El}{{{j{{Eh{c}}}}}CbEl}{cc{}}0`{{{j{l{Eh{c}}}}En}fEl}{ce{}{}}04{c{{En{c}}}El}`{{{j{l{Eh{c}}}}}{{Cl{En}}}El}`{Enf}{{{j{c}}}e{}{}}0{c{{n{e}}}{}{}}000{{{j{c}}}A`{}}0`{{{j{c}}}{{j{e}}}{}{}}{{{j{{F`{c}}}}}{{Fb{c}}}Fd}{{{j{{F`{c}}}}}{{Ff{c}}}Fd}{{{j{lc}}}{{j{le}}}{}{}}<:{c{{F`{c}}}{}}665","D":"CFb","p":[[1,"never"],[1,"u8"],[1,"unit"],[5,"PanicInfo",646],[1,"reference"],[0,"mut"],[6,"Result",647],[5,"TypeId",648],[5,"TrapFrame",111],[5,"CLikeStr",75],[5,"Formatter",649],[8,"Result",649],[1,"u32"],[6,"EnvStatus",428],[5,"String",650],[6,"KError",165],[1,"str"],[5,"Arguments",649],[5,"Stdout",211],[5,"BuddyInner",228],[5,"Layout",651],[5,"BuddyAllocator",228],[8,"PageNode",256],[1,"usize"],[1,"bool"],[5,"PageData",256],[8,"Pde",256],[8,"Pte",256],[1,"tuple"],[6,"Option",652],[5,"MemoryPool",319],[5,"Vec",653],[5,"MemoryPoolEntry",319],[1,"array"],[5,"Elf32Ehdr",367],[5,"Elf32Phdr",367],[8,"ElfMapperFn",367],[10,"Fn",654],[5,"IpcData",428],[5,"EnvData",428],[5,"ArrayLinkNode",532],[5,"ArrayLinkedList",532],[5,"Bitmap",578],[5,"LinkList",598],[10,"Clone",655],[10,"Copy",656],[5,"LinkNode",598],[5,"SyncImplRef",635],[5,"Ref",657],[10,"Sized",656],[5,"RefMut",657],[6,"SyscallNo",44],[6,"UError",165],[8,"PageList",256],[5,"EnvsWrapper",428],[5,"Aligned",532]],"r":[],"b":[],"c":"OjAAAAAAAAA=","e":"OzAAAAEAANUAKwAQAAAARQABAEkAAgBQAAEAUwAAAGwAAwByAAUAfQAAAIAAAACNAAMAlAARALoABADDAA8A1wABANsAAgDkAAAA6QADAPsABQANAQUAJAEDAEUBAwBPAQAAXgEFAG0BAACGAQsApAEHAMIBEwDXAQEA5AEBAPsBAgD/AQsAGQIJACUCAAAnAgAAOQIJAEYCAQBKAgAAUQICAFkCCwB0AgcAfQIAAIACAACEAgIA"}]\
]'));
if (typeof exports !== 'undefined') exports.searchIndex = searchIndex;
else if (window.initSearch) window.initSearch(searchIndex);
