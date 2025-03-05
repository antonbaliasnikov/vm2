searchState.loadedDescShard("zksync_vm2_interface", 0, "EraVM Stable Interface\nPublic interface of an EraVM call frame.\nAll supported calling modes for <code>FarCall</code> opcode.\nThe VM should continue.\nCycle statistics emitted by the VM and supplied to …\nDecommitting an opcode.\nDelegate calling mode (similar to <code>delegatecall</code> in EVM).\nCall to the <code>ecrecover</code> precompile with the specified number …\nEvent emitted by EraVM.\nIdentifier of the heap used by the first executed program …\nIdentifier of the auxiliary heap used by the first …\nIdentifier of the calldata heap used by the first executed …\nVM execution flags. See the EraVM reference for more …\nState interface with access to global state like storage.\nIdentifier of a VM heap.\nCall to the <code>keccak256</code> precompile with the specified number …\nL2-to-L1 log emitted by EraVM.\nMimic calling mode (can only be used by system contracts; …\nNormal calling mode.\nNormal return.\nAll supported EraVM opcodes in a single enumeration.\nTrait mapping opcodes as types to the corresponding …\nPanic, i.e. a non-revert abnormal control flow termination …\nAll supported return types for the <code>Ret</code> opcode.\nRevert (e.g., a result of a Solidity <code>revert</code>).\nCall to the <code>secp256r1_verify</code> precompile with the specified …\nCall to the <code>sha256</code> precompile with the specified number of …\nReturned from <code>Tracer::after_instruction</code> to indicate if the …\nPublic interface of the VM state. Encompasses both read …\nThe VM should stop.\nReading a slot from the VM storage.\nWriting a slot to the VM storage.\nEraVM instruction tracer.\n<code>Opcode</code> variant corresponding to this opcode type.\nAddress of the storage context associated with this frame. …\nAddress of the contract that has emitted this log.\nThis method is executed after an instruction handler.\nThis method is executed after an instruction handler.\nConverts this ID to an integer value.\nReturns ID of the auxiliary heap used in this call.\nReturns the auxiliary heap boundary (number of paid bytes).\nThis method is executed before an instruction handler.\nThis method is executed before an instruction handler.\nAddress of the calling contract. Respects delegate and …\nReturns a mutable handle to a call frame with the …\nAddress of the contract being executed.\nReturns the context value for this call. This context is …\nReturns the value of the context register.\nReturns a mutable handle to the current call frame.\n“Equal” flag.\nIterates over events emitted during VM execution.\nReturns the program counter that the parent frame should …\nReturns current execution flags.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the remaining amount of gas.\nGets value of the specified storage slot.\nIterates over storage slots read or written during VM …\nGets value of the specified transient storage slot.\nIterates over all transient storage slots set during VM …\n“Greater than” flag.\nReturns ID of the main heap used in this call.\nReturns the main heap boundary (number of paid bytes).\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nChecks if this return type is normal.\nIs this event first in a chain of events?\nChecks whether the call is executed in kernel mode.\nChecks whether this frame corresponds to a near call.\nIs this a service log?\nChecks whether the call is static.\nEvent key.\nLog key.\nIterates over L2-to-L1 logs emitted during VM execution.\n“Less than” flag.\nReturns the total number of call frames.\nProvides cycle statistics for “complex” instructions …\nProvides cycle statistics for “complex” instructions …\nEraVM opcodes.\nReturns the current program counter (i.e., 0-based index …\nGets the current amount of published pubdata.\nReads a word from the bytecode of the executing contract.\nReads a single byte from the specified heap at the …\nReads an entire <code>U256</code> word in the big-endian order from the …\nReads a register with the specified zero-based index. …\nReads the specified stack slot. Returns a value together …\nSets the address of the executing contract.\nSets the auxiliary heap boundary.\nSets the address of the calling contract.\nSets the address of the contract being executed. Does not …\nSets the context value for this call.\nSets the value of the context register.\nSets the exception handler as specified above.\nSets current execution flags.\nSets the remaining amount of gas.\nSets the main heap boundary.\nSets the program counter. The VM will execute an invalid …\nSets the current amount of published pubdata.\nSets a register with the specified zero-based index\nSets the stack pointer.\nSets the current transaction number.\nShard identifier (currently, always set to 0).\nShard identifier (currently, always set to 0).\nReturns the stack pointer.\nReturns the currently set 0-based transaction number.\n0-based index of a transaction that has emitted this event.\n0-based index of a transaction that has emitted this event.\nEvent value.\nLog value.\nWrites an entire <code>U256</code> word in the big-endian order to the …\nSets the value and pointer flag for the specified stack …\nSets value of the specified transient storage slot.\n<code>Add</code> opcode.\n<code>And</code> opcode.\n<code>AuxHeapRead</code> opcode.\n<code>AuxHeapWrite</code> opcode.\n<code>AuxMutating0</code> opcode.\n<code>Caller</code> opcode.\n<code>CodeAddress</code> opcode.\n<code>ContextMeta</code> opcode.\n<code>ContextU128</code> opcode.\n<code>Decommit</code> opcode.\nDelegate <code>FarCall</code> mode.\n<code>Div</code> opcode.\n<code>ErgsLeft</code> opcode.\n<code>Event</code> opcode.\n<code>FarCall</code> group of opcodes distinguished by the calling mode …\n<code>HeapRead</code> opcode.\n<code>HeapWrite</code> opcode.\n<code>IncrementTxNumber</code> opcode.\n<code>Jump</code> opcode.\n<code>L2ToL1Message</code> opcode.\nMimic <code>FarCall</code> mode.\n<code>Mul</code> opcode.\n<code>NearCall</code> opcode.\n<code>Nop</code> opcode.\nNormal <code>Ret</code>urn mode / <code>FarCall</code> mode.\n<code>Or</code> opcode.\nPanic <code>Ret</code>urn mode.\n<code>PointerAdd</code> opcode.\n<code>PointerPack</code> opcode.\n<code>PointerRead</code> opcode.\n<code>PointerShrink</code> opcode.\n<code>PointerSub</code> opcode.\n<code>PrecompileCall</code> opcode.\n<code>Ret</code> group of opcodes distinguished by the return type …\nRevert <code>Ret</code>urn mode.\n<code>RotateLeft</code> opcode.\n<code>RotateRight</code> opcode.\n<code>SP</code> opcode.\n<code>SetContextU128</code> opcode.\n<code>ShiftLeft</code> opcode.\n<code>ShiftRight</code> opcode.\n<code>StorageRead</code> opcode.\n<code>StorageWrite</code> opcode.\n<code>Sub</code> opcode.\n<code>This</code> opcode.\n<code>TransientStorageRead</code> opcode.\n<code>TransientStorageWrite</code> opcode.\nCalling mode for the <code>FarCall</code> opcodes.\nReturn type for the <code>Ret</code> opcodes.\nConstant corresponding to this mode allowing to easily …\nConstant corresponding to this return type allowing to …\n<code>Xor</code> opcode.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.")