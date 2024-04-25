use super::{common::instruction_boilerplate_with_panic, free_panic};
use crate::{
    addressing_modes::{Arguments, Immediate1, Register1, Register2, Source},
    instruction::InstructionResult,
    modified_world::Event,
    Instruction, Predicate, VirtualMachine,
};
use u256::H160;
use zkevm_opcode_defs::ADDRESS_EVENT_WRITER;

fn event(vm: &mut VirtualMachine, instruction: *const Instruction) -> InstructionResult {
    instruction_boilerplate_with_panic(vm, instruction, |vm, args, continue_normally| {
        if vm.state.current_frame.is_static {
            return free_panic(vm);
        }
        if vm.state.current_frame.address == H160::from_low_u64_be(ADDRESS_EVENT_WRITER as u64) {
            let key = Register1::get(args, &mut vm.state);
            let value = Register2::get(args, &mut vm.state);
            let is_first = Immediate1::get(args, &mut vm.state).low_u32() == 1;

            vm.world.record_event(Event {
                key,
                value,
                is_first,
                shard_id: 0, // shards currently aren't supported
                tx_number: vm.state.transaction_number,
            });
        }

        continue_normally
    })
}

impl Instruction {
    pub fn from_event(
        key: Register1,
        value: Register2,
        is_first: bool,
        predicate: Predicate,
    ) -> Self {
        Self {
            handler: event,
            arguments: Arguments::new(predicate, 34)
                .write_source(&key)
                .write_source(&value)
                .write_source(&Immediate1(is_first.into())),
        }
    }
}
