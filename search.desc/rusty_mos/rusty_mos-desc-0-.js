searchState.loadedDescShard("rusty_mos", 0, "Get the array[i] via the raw pointer\nGet the virtual address (in <strong>kernel segment</strong>) from the …\nGet the physical address of the virtual address in <strong>kernel </strong>…\nGet the <strong>Page Directory Offset</strong> from the virtual address\nGet the Page Number from the physical address\nGet the address(or the frame number etc.) from the page …\nGet the <strong>Page Table Offset</strong> from the virtual address\nRound-up to the specified fractor\nGet the page object of the phisical address\nGet the kernel virtual address of the page object\nGet the physical address of the page object\nGet the page number through the page object\nDefine the Error Code in mos.\nDefinitions about the memory / page and conventions. …\nDefinitions of the conversion macros for the page\nLink List implemented with Rust, which is similar to the …\nDefinition of some misc macros about type and offset\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nThe data stored in the link list, with the type <code>T</code>.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nPointing to the first node of this link list. The list is …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nPointing the next node. If this is the last node, the …\nPointing the previous node’s <code>next</code> field. If this is the …\nThe environment does not exist or otherwise cannot be used …\nBad path\nFile already exisits\nThe parameter is invalid\nAttempt to send to env that is not recving\nError Codes Enum only for <strong>the Kernel</strong>\nThe maximum count of opened file exceeded\nNo free space left on disk\nThe environment maximum count exceeded\nRun out of memory\nInvalid syscall number\nFile is not a valid executable\nFile or block not found\nError Codes Enum only for <strong>the User’s File System</strong>\nUnspecified or unknown problem\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nThe maximum count of all the <strong>asid</strong>\nShifted the <em>Page Table Offset</em> and <em>In-Page Offset</em> out to …\nShifted the <em>In-Page Offset</em> out to get the <strong>Page Table Offset</strong>\nCache Coherency Attributes bit.\nValid bit. If 0 any address matching this entry will cause …\nThe head struct of the LinkList\nThe node struct of the LinkList\nThe data stored in the link list, with the type <code>T</code>.\nJudge whether this list is empty.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nPointing to the first node of this link list. The list is …\nSafety\nInsert a node to the head of the list\nSafety\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCreate an empty link list with its head null.\nCreate an empty link list node with its linking-field all …\nPointing the next node. If this is the last node, the …\nSafety\nGet the first node of this list and removce it\nPointing the previous node’s <code>next</code> field. If this is the …\nSafety\nRemove a specified node from the list contains this node.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nSafety\nRun before the <code>env_run</code> for <strong>tests</strong> only\nSafety\nSafety\nSafety\nSafety\nSafety\nSafety\nSafety\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nSafety\nThe data stored in the link list, with the type <code>T</code>.\nReturns the argument unchanged.\nPointing to the first node of this link list. The list is …\nCalls <code>U::from(self)</code>.\nPointing the next node. If this is the last node, the …\nSafety\nPointing the previous node’s <code>next</code> field. If this is the …\nSafety\nSafety\nSafety\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.")