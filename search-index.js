var searchIndex = new Map(JSON.parse('[\
["rusty_mos",{"t":"QQQQQQQQQQQQQQCCCCQQQQQQQCCCCCCCCCSSSSSSSSSSSSSSSSSSFFISSSSNNNNNOOONNNNNNOOONNNNNNOFIIGIPFSSPPONNNNNNNNNNNNONOONNNNNOOOONNNOOOOOOOOOOONNNNNNNNNOOPPPPPGPPPPPPPGPNNNNNNNNNNNNNNNSSSSSSSSSSSSSSSSSSSSSFFFNNNNNNNNNONNNNNNNNOONNNNNNNNNONNONNONNNNNNNNNCCCCCCHJJJJJJFNNHHHHHHNNHNNNHHJJJJFIIIINNNONONHHOHHHHHHHHOONNNHHHFNNNNOOOONNNONOONNNCHHHHHHHHHH","n":["ARRAY_PTR","CALL_TEST","ENVX","GEN_MASK","KADDR","PADDR","PDX","PPN","PTE_ADDR","PTX","ROUND","ROUNDDOWN","debug","debugln","kdef","kern","klib","ktests","pa2page","page2kva","page2pa","page2ppn","print","println","va2pa","bitops","cp0reg","elf","env","error","mmu","pmap","queue","types","STATUS_BEV","STATUS_CU0","STATUS_CU1","STATUS_CU2","STATUS_CU3","STATUS_ERL","STATUS_EXL","STATUS_IE","STATUS_IM0","STATUS_IM1","STATUS_IM2","STATUS_IM3","STATUS_IM4","STATUS_IM5","STATUS_IM6","STATUS_IM7","STATUS_R0","STATUS_UM","Elf32Ehdr","Elf32Phdr","ElfMapperFn","PF_R","PF_W","PF_X","PT_LOAD","borrow","borrow","borrow_mut","borrow_mut","clone","entry","filesz","flags","foreach","from","from","from","into","into","memsz","offset","stype","try_from","try_from","try_into","try_into","type_id","type_id","vaddr","EnvData","EnvList","EnvNode","EnvStatus","EnvTailList","Free","IpcData","LOG2NENV","NENV","NotRunnable","Runnable","asid","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","clone","clone","clone","const_construct","const_construct","const_construct","data","default","dstva","env_runs","eq","fmt","from","from","from","from_id","head","head","id","into","into","into","ipc_data","next","parent_id","perm","pgdir","prev","priority","receiving","status","tail","trap_frame","try_from","try_from","try_from","try_into","try_into","try_into","type_id","type_id","type_id","user_tlb_mod_entry","value","BadEnv","BadPath","FileExists","Invalid","IpcNotRecv","KError","MaxOpen","NoDisk","NoFreeEnv","NoMem","NoSys","NotExec","NotFound","UError","Unspecified","borrow","borrow","borrow_mut","borrow_mut","fmt","from","from","into","into","try_from","try_from","try_into","try_into","type_id","type_id","KERNBASE","KSTACKTOP","NASID","PAGE_SIZE","PDMAP","PDSHIFT","PGSHIFT","PTE_C_CACHEABLE","PTE_D","PTE_G","PTE_V","UCOW","UENVS","ULIM","UPAGES","USTACKTOP","UTEMP","UTEXT","UTOP","UVPT","UXSTACKTOP","LinkList","LinkNode","TailLinkList","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","clone","clone","const_construct","data","default","default","empty","empty","enable","from","from","from","head","head","insert_head","insert_head","insert_tail","into","into","into","new","new","new","next","pop_head","pop_head","prev","remove","remove","tail","try_from","try_from","try_from","try_into","try_into","try_into","type_id","type_id","type_id","elf","env","machine","pmap","tlbex","trap","elf_load_seg","ASID_BITMAP","BASE_PGDIR","CUR_ENV","ENVS","ENV_FREE_LIST","ENV_SCHE_LIST","EnvsWrapper","borrow","borrow_mut","env_alloc","env_create","env_free","env_init","env_setup_vm","envid2env","from","into","load_icode","try_from","try_into","type_id","halt","print_charc","CUR_PGDIR","NPAGE","PAGES","PAGE_FREE_LIST","PageData","PageList","PageNode","Pde","Pte","borrow","borrow_mut","clone","data","from","head","into","mips_detect_memory","mips_vm_init","next","page_alloc","page_decref","page_free","page_init","page_insert","page_lookup","page_remove","pgdir_walk","pp_ref","prev","try_from","try_into","type_id","_do_tlb_refill","tlb_invalidate","tlb_out","TrapFrame","borrow","borrow_mut","clone","const_construct","cp0_badvaddr","cp0_cause","cp0_epc","cp0_status","default","fmt","from","hi","into","lo","regs","try_from","try_into","type_id","print","_print","_write_str","test_envid2env","test_envs","test_icode_loader","test_linklist","test_page","test_page_strong","test_tailq","test_tlb_refill"],"q":[[0,"rusty_mos"],[25,"rusty_mos::kdef"],[34,"rusty_mos::kdef::cp0reg"],[52,"rusty_mos::kdef::elf"],[83,"rusty_mos::kdef::env"],[145,"rusty_mos::kdef::error"],[175,"rusty_mos::kdef::mmu"],[196,"rusty_mos::kdef::queue"],[244,"rusty_mos::kern"],[250,"rusty_mos::kern::elf"],[251,"rusty_mos::kern::env"],[272,"rusty_mos::kern::machine"],[274,"rusty_mos::kern::pmap"],[306,"rusty_mos::kern::tlbex"],[309,"rusty_mos::kern::trap"],[328,"rusty_mos::klib"],[329,"rusty_mos::klib::print"],[331,"rusty_mos::ktests"],[339,"core::ops::function"],[340,"core::result"],[341,"core::any"],[342,"core::fmt"],[343,"core::clone"],[344,"core::marker"],[345,"core::option"]],"i":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,23,1,23,1,1,1,23,23,1,23,1,1,23,1,23,23,23,23,1,23,1,23,1,23,0,0,0,0,0,9,0,0,0,9,9,11,9,10,11,9,10,11,9,10,11,12,10,11,12,9,10,11,9,9,9,10,11,10,35,36,11,9,10,11,11,12,11,10,11,12,11,10,11,36,11,9,10,11,9,10,11,9,10,11,11,10,16,37,37,16,16,0,37,37,16,16,16,37,37,0,16,37,16,37,16,16,37,16,37,16,37,16,37,16,37,16,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,21,17,20,21,17,20,17,20,20,20,21,17,21,17,21,21,17,20,21,17,21,17,21,21,17,20,21,17,20,20,21,17,20,21,17,21,21,17,20,21,17,20,21,17,20,0,0,0,0,0,0,0,0,0,0,0,0,0,0,38,38,0,0,0,0,0,0,38,38,0,38,38,38,0,0,0,0,0,0,0,0,0,0,0,26,26,26,27,26,39,26,0,0,27,0,0,0,0,0,0,0,0,26,27,26,26,26,0,0,0,0,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,0,0,0,0,0,0,0,0,0,0,0],"f":"```````````````````````````````````````````````````````````{ce{}{}}000{bb}```{{bc}d{{h{f}}}}{cc{}}0{{jl}}44```{c{{n{e}}}{}{}}000{cA`{}}0`````````````666666{AbAb}{AdAd}{AfAf}{{}Ah}{{}Ad}{{}Af}`{{}Ab}``{{AbAb}Aj}{{AbAl}An}<<<````???```````````::::::999`````````````````????{{B`Al}An}=={ce{}{}}0<<<<;;````````````````````````000000{{{Bb{c}}}{{Bb{c}}}{BdBf}}{{{Bh{c}}}{{Bh{c}}}{BdBf}}{{}{{Bh{Af}}}}`{{}{{Bj{c}}}Bf}{{}{{Bb{c}}}Bf}{{{Bj{c}}}AjBf}{{{Bb{c}}}AjBf}{{{Bj{c}}}dBf}{cc{}}00``{{{Bj{c}}Bh}dBf}{{{Bb{c}}Bh}dBf}1;;;76{c{{Bh{c}}}Bf}`{{{Bj{c}}}{{Bl{Bh}}}Bf}{{{Bb{c}}}{{Bl{Bh}}}Bf}`4{Bhd}`{c{{n{e}}}{}{}}00000{cA`{}}00``````{{BnjC`Ah}{{n{dB`}}}}```````{ce{}{}}0{f{{n{AhB`}}}}{{jlf}{{Bl{Ah}}}}{Ahd}{{}d}{Ah{{n{dB`}}}}{{fAj}{{n{AhB`}}}}{cc{}}7{{Ahjl}d};;:{{}Cb}{jd}`````````::{CdCd}`4`;{ld}{{ll}d}`{{}{{n{CfB`}}}}{Cfd}03{{ChlffCf}{{n{dB`}}}}{{Chl}{{Bl{{Cl{CfCj}}}}}}{{Chlf}d}{{ChlAj}{{n{CjB`}}}}``{c{{n{e}}}{}{}}0{cA`{}}{{{Cn{f}}lf}d}{{fl}d}``{ce{}{}}0{D`D`}{{}D`}````0{{D`Al}An}{cc{}}`4``887`{Dbd}{Ddd}{{}d}0000000","D":"Kn","p":[[5,"Elf32Ehdr",52],[1,"unit"],[1,"u32"],[10,"Fn",339],[1,"u8"],[1,"usize"],[6,"Result",340],[5,"TypeId",341],[6,"EnvStatus",83],[5,"IpcData",83],[5,"EnvData",83],[8,"EnvNode",83],[1,"bool"],[5,"Formatter",342],[8,"Result",342],[6,"KError",145],[5,"LinkList",196],[10,"Clone",343],[10,"Copy",344],[5,"LinkNode",196],[5,"TailLinkList",196],[6,"Option",345],[5,"Elf32Phdr",52],[8,"ElfMapperFn",52],[1,"never"],[5,"PageData",274],[8,"PageNode",274],[8,"Pde",274],[8,"Pte",274],[1,"tuple"],[1,"array"],[5,"TrapFrame",309],[5,"Arguments",342],[1,"str"],[8,"EnvList",83],[8,"EnvTailList",83],[6,"UError",145],[5,"EnvsWrapper",251],[8,"PageList",274]],"r":[],"b":[],"c":"OjAAAAAAAAA=","e":"OzAAAAEAAPQAIQAAAAAAAgACAAwABgAXAAYAIwAhAEcAAABKACEAbQAEAHUAAAB3AAEAfAAAAH4AAgCCAA8AoQAEAKoABwCzAAEAuAAAALoACgDHAAkA0gACANYAAADaAAAA4gAAAOsADwD8AAgACAEAAA4BEAAjAQEAJgEDACsBAwAwARAAQgEAAEQBDwA="}]\
]'));
if (typeof exports !== 'undefined') exports.searchIndex = searchIndex;
else if (window.initSearch) window.initSearch(searchIndex);