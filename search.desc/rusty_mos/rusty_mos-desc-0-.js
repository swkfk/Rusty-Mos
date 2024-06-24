searchState.loadedDescShard("rusty_mos", 0, "Get the array[i] via the raw pointer.\nSpawn the index of the given <code>envid</code>.\nGen a mask from the $h-bit (MSB) to $l-bit (LSB) with 1.\nGet the virtual address (in <strong>kernel segment</strong>) from the …\nGet the physical address of the virtual address in <strong>kernel </strong>…\nGet the <strong>Page Directory Offset</strong> from the virtual address\nGet the Page Number from the physical address\nGet the address(or the frame number etc.) from the page …\nGet the <strong>Page Table Offset</strong> from the virtual address\nRound-up to the specified fractor.\nRound-down to the specified fractor.\nThings related to the architecture. <strong>Mipsel</strong> here is.\nGet the page object of the phisical address.\nGet the kernel virtual address of the page object.\nGet the physical address of the page object.\nGet the page number through the page object.\nGet the physical address of the given virtual address. …\nMachine related implementations.\nThe syscall number enum definition.\nSyscall implementations.\nHandle exceptions(traps) and handler definitions.\nActually trigger the reboot of the board. But in QEMU, we …\nPut the character (whose size is 1 byte) into teh serial. …\nGet a character (whose size is 1 byte) from the serial. …\nThe count of the syscalls. It should be updated manually.\nScan a char from the console\nCreate a memory pool.\nDestory a env by its id and kill it.\nAllocate a new env and make it child of the current env.\nBind a memory pool.\nGet the id of the current env.\nReceive a ipc data or wait.\nTry to send an ipc data to a env.\nAllocate memory and map it.\nMap the virtual address to a specified physical page.\nUnmap the address and the page.\nDo kernel panic.\nPrint a straight string ends with zero into the console.\nPrint a char into the console.\nRead from a dev.\nSet the env status and move it between the lists.\nRegister the user-space TLB mod handler for the specified …\nSet the trapframe to he specified env.\nWrite to a dev.\nGive out the CPU time and re-schedule.\nSyscall identifier. The number is ordered by enum …\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nSyscall function table. Indexed with the syscall number.\nGet the syscall number and all the five arguments. Invoke …\nException handlers table. This will be exported as …\nThings need to be stored in the trapframe.\nThe default value of the registers.\nThe BadAddr register in CP0.\nThe Cause register in CP0.\nThe EPC register in CP0.\nThe Status register in CP0.\nInvoke a panic.\nSkip the current instruction.\nReturns the argument unchanged.\nHandle the clock-interrupt.\nHandle the TLB Mod exception.\nHandle unknown exception code. Invoke a kernel panic.\nSkip the current instruction.\nHandle the syscall.\nHandle the TLBL or TLBS exception.\nThe HI register.\nCalls <code>U::from(self)</code>.\nThe LO register.\nAll the regular registers.\nDefine all the status bits for the cp0 reg.\nDefine the Error Code in mos.\nDefinition of some misc macros about type and offset.\nThe environment does not exist or otherwise cannot be used …\nBad path\nFile already exisits\nThe parameter is invalid\nAttempt to send to env that is not recving\nError Codes Enum only for <strong>the Kernel</strong>\nThe maximum count of opened file exceeded\nNo free space left on disk\nThe environment maximum count exceeded\nRun out of memory\nInvalid syscall number\nFile is not a valid executable\nFile or block not found\nAn env try to bind to a pool twice\nThe env is not binded to the pool\nInvalid memory pool id\nError Codes Enum only for <strong>the User’s File System</strong>\nUnspecified or unknown problem\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nDefinitions of the conversion macros for the pages and …\nDefinitions about the memory / page and conventions. …\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nThe data stored in the link list, with the type <code>T</code>.\nReturns the argument unchanged.\nPointing to the first node of this link list. The list is …\nCalls <code>U::from(self)</code>.\nPointing the next node. If this is the last node, the …\nSafety\nPointing the previous node’s <code>next</code> field. If this is the …\nKernel memory starts from here\nKSEG1 Segment\nKernel stack end at here (the <code>end</code> in the linking script).\nThe maximum count of all the <strong>asid</strong>\nThe physical page size (in bytes).\nBytes mapped by a page directory entry.\nShifted the <em>Page Table Offset</em> and <em>In-Page Offset</em> out to …\nShifted the <em>In-Page Offset</em> out to get the <strong>Page Table Offset</strong>\nCache Coherency Attributes bit. If set, this entry is …\nDirty bit, but really a write-enable bit. 1 to allow …\nGlobal bit. When this bit in a TLB entry is set, that TLB …\nValid bit. If 0 any address matching this entry will cause …\nReserved for COW (start address).\nThe kernel array <code>ENVS</code> will be mapped here.\nThe high-limits of user’s memory\nThe kernel array <code>PAGES</code> will be mapped here.\nNormal user stack top.\nReserved for temporary usage (start address).\nUser test segment start.\nThe uer’s space higher boundary.\nUser’s page tables are stored here (for a PDMAP size).\nThe exception stack top for the user. See also: UTOP\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nSafety\nCore methods for the env module.\nDo the schedule job.\nThe ELF32 file header structure. The members are defined …\nThe program segment header structure. The members are …\nThe type used in the walker of the header.\nMark the segment as readable.\nMark the segment as writable.\nMark the segment as executable.\nMark the segment as loadble and load-needed.\nLoad an elf-format binary file in memory. This method will …\nThe <strong>virtual</strong> address for the entry point.\nThe segment size in <strong>file</strong>.\nThe segment flag.\nWalk all the program header entry, use the function <code>apply</code> …\nBuild the Elf32Ehdr object from the binary.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nThe segment size in <strong>memory</strong>.\nThe segment offset.\nThe segment type.\nThe <strong>virtual</strong> address of the segment.\nThe envs array used in the <em>kernel</em> space. The element in it …\nThe global pgdir.\nThe current env.\nThe envs array in <em>kernel</em>, mapped to the UENVS and used by …\nFree env list.\nRunnable env list.\nThe PCB struct. Compatible with the C-Like memory …\nThe env status enum. Compatible with the C-Like memory …\nWrapper to make it aligned to a page.\nThe env is not used (free).\nThe IPC data collected together. Compatible with the …\nThe log of NENV.\nThe count of the envs.\nThe env is blocked.\nThe env is running or to be run.\nThe asid for TLB.\nUsed for the static construction. All members are filled …\nThe target virtual address.\nAlloc an <code>env</code> and setup its vm and PCB.\nCreate an env and load the icode, set the priority. For …\nDestory an env and free it. Re-schedule will be performed.\nFree an env, and remove all its pages. The TLB will be …\nInit the env environment. Put the envs into the free list, …\nRecover from exception, load the specified <strong>TrapFrame</strong>.\nRun the env. Save the [CUR_ENV]’s trapframe if <code>CUR_ENV</code> …\nUsed in Lab 6. ///\nSetup the virtual memory of the new-born env.\nGet the env’s PCB by its id. If <code>checkperm</code> is set, the …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nThe sender’s env id.\nThe env id.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nThe IPC data collected.\nUsed for the static construction. All members are filled …\nThe env’s parent env’s id;\nThe page permission.\nThe page directory address of this env.\nThe priority of this env.\nMark this env’s receiving status.\nThe running status of this env.\nTrap Frame stored in the PCB.\nThe entry of the tlb mod handler in user space.\nThe value passed directly.\nSchedule the envs. If <code>yield</code>, the current env will be moved …\nLink List implemented with Rust, which is similar to the …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nThe head struct of the LinkList\nThe node struct of the LinkList\nThe data stored in the link list, with the type <code>T</code>.\nConstructor for the default.\nJudge whether this list is empty.\nReturns the argument unchanged.\nReturns the argument unchanged.\nPointing to the first node of this link list. The list is …\nInsert a node to the head of the list\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCreate an empty link list with its head null.\nCreate an empty link list node with its linking-field all …\nPointing the next node. If this is the last node, the …\nGet the first node of this list and removce it\nPointing the previous node’s <code>next</code> field. If this is the …\nRemove a specified node from the list contains this node.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.")