use crate::{
    addressing_modes::{
        AbsoluteStack, AdvanceStackPointer, AnyDestination, AnySource, Arguments, CodePage,
        Immediate1, Immediate2, Register, Register1, Register2, RegisterAndImmediate,
        RelativeStack, Source, SourceWriter,
    },
    end_execution,
    instruction_handlers::{
        Add, And, AuxHeap, Div, Heap, Mul, Or, PtrAdd, PtrPack, PtrShrink, PtrSub, RotateLeft,
        RotateRight, ShiftLeft, ShiftRight, Sub, Xor,
    },
    jump_to_beginning,
    state::{ExecutionResult, Panic},
    Instruction, Predicate, State,
};
use zkevm_opcode_defs::{
    decoding::{EncodingModeProduction, VmEncodingMode},
    ImmMemHandlerFlags, Opcode,
    Operand::*,
    RegOrImmFlags, SET_FLAGS_FLAG_IDX, SWAP_OPERANDS_FLAG_IDX_FOR_ARITH_OPCODES,
    SWAP_OPERANDS_FLAG_IDX_FOR_PTR_OPCODE, UMA_INCREMENT_FLAG_IDX,
};

pub fn decode_program(raw: &[u64]) -> Vec<Instruction> {
    raw.iter()
        .take(1 << 16)
        .map(|i| decode(*i))
        .chain(std::iter::once(if raw.len() >= 1 << 16 {
            jump_to_beginning()
        } else {
            // TODO execute invalid instruction or something instead
            end_execution()
        }))
        .collect()
}

fn unimplemented_instruction(variant: Opcode) -> Instruction {
    let mut arguments = Arguments::new(Predicate::Always);
    let variant_as_number: u16 = unsafe { std::mem::transmute(variant) };
    Immediate1(variant_as_number).write_source(&mut arguments);
    Instruction {
        handler: unimplemented_handler,
        arguments,
    }
}
fn unimplemented_handler(state: &mut State, instruction: *const Instruction) -> ExecutionResult {
    let variant: Opcode = unsafe {
        std::mem::transmute(Immediate1::get(&(*instruction).arguments, state).low_u32() as u16)
    };
    eprintln!("Unimplemented instruction: {:?}!", variant);
    Err(Panic::JumpingOutOfProgram)
}

fn decode(raw: u64) -> Instruction {
    let (parsed, _) = EncodingModeProduction::parse_preliminary_variant_and_absolute_number(raw);

    let predicate = match parsed.condition {
        zkevm_opcode_defs::Condition::Always => crate::Predicate::Always,
        zkevm_opcode_defs::Condition::Gt => crate::Predicate::IfGT,
        zkevm_opcode_defs::Condition::Lt => crate::Predicate::IfLT,
        zkevm_opcode_defs::Condition::Eq => crate::Predicate::IfEQ,
        zkevm_opcode_defs::Condition::Ge => crate::Predicate::IfGE,
        zkevm_opcode_defs::Condition::Le => crate::Predicate::IfLE,
        zkevm_opcode_defs::Condition::Ne => crate::Predicate::IfNotEQ,
        zkevm_opcode_defs::Condition::GtOrLt => crate::Predicate::IfGtOrLT,
    };

    let stack_in = RegisterAndImmediate {
        immediate: parsed.imm_0,
        register: Register::new(parsed.src0_reg_idx),
    };
    let src1: AnySource = match parsed.variant.src0_operand_type {
        RegOnly | RegOrImm(RegOrImmFlags::UseRegOnly) | Full(ImmMemHandlerFlags::UseRegOnly) => {
            Register1(Register::new(parsed.src0_reg_idx)).into()
        }
        RegOrImm(RegOrImmFlags::UseImm16Only) | Full(ImmMemHandlerFlags::UseImm16Only) => {
            Immediate1(parsed.imm_0).into()
        }
        Full(ImmMemHandlerFlags::UseAbsoluteOnStack) => AbsoluteStack(stack_in).into(),
        Full(ImmMemHandlerFlags::UseStackWithPushPop) => AdvanceStackPointer(stack_in).into(),
        Full(ImmMemHandlerFlags::UseStackWithOffset) => RelativeStack(stack_in).into(),
        Full(ImmMemHandlerFlags::UseCodePage) => CodePage(stack_in).into(),
    };

    let stack_out = RegisterAndImmediate {
        immediate: parsed.imm_1,
        register: Register::new(parsed.dst0_reg_idx),
    };
    let out: AnyDestination = match parsed.variant.dst0_operand_type {
        RegOnly | RegOrImm(RegOrImmFlags::UseRegOnly) | Full(ImmMemHandlerFlags::UseRegOnly) => {
            Register1(Register::new(parsed.dst0_reg_idx)).into()
        }
        RegOrImm(RegOrImmFlags::UseImm16Only) | Full(ImmMemHandlerFlags::UseImm16Only) => {
            panic!("Parser wants to output to immediate")
        }
        Full(ImmMemHandlerFlags::UseAbsoluteOnStack) => AbsoluteStack(stack_out).into(),
        Full(ImmMemHandlerFlags::UseStackWithPushPop) => AdvanceStackPointer(stack_out).into(),
        Full(ImmMemHandlerFlags::UseStackWithOffset) => RelativeStack(stack_out).into(),
        Full(ImmMemHandlerFlags::UseCodePage) => panic!("Parser wants to write to code page"),
    };

    let src2 = Register2(Register::new(parsed.src1_reg_idx));
    let out2 = Register2(Register::new(parsed.dst1_reg_idx));

    macro_rules! binop {
        ($op: ident, $snd: tt) => {
            Instruction::from_binop::<$op>(
                src1,
                src2,
                out,
                $snd,
                predicate,
                parsed.variant.flags[SWAP_OPERANDS_FLAG_IDX_FOR_ARITH_OPCODES],
                parsed.variant.flags[SET_FLAGS_FLAG_IDX],
            )
        };
    }

    macro_rules! ptr {
        ($op: ident) => {
            Instruction::from_ptr::<$op>(
                src1,
                src2,
                out,
                predicate,
                parsed.variant.flags[SWAP_OPERANDS_FLAG_IDX_FOR_PTR_OPCODE],
            )
        };
    }

    match parsed.variant.opcode {
        zkevm_opcode_defs::Opcode::Add(_) => binop!(Add, ()),
        zkevm_opcode_defs::Opcode::Sub(_) => binop!(Sub, ()),
        zkevm_opcode_defs::Opcode::Mul(_) => binop!(Mul, out2),
        zkevm_opcode_defs::Opcode::Div(_) => binop!(Div, out2),
        zkevm_opcode_defs::Opcode::Binop(x) => match x {
            zkevm_opcode_defs::BinopOpcode::Xor => binop!(Xor, ()),
            zkevm_opcode_defs::BinopOpcode::And => binop!(And, ()),
            zkevm_opcode_defs::BinopOpcode::Or => binop!(Or, ()),
        },
        zkevm_opcode_defs::Opcode::Shift(x) => match x {
            zkevm_opcode_defs::ShiftOpcode::Shl => binop!(ShiftLeft, ()),
            zkevm_opcode_defs::ShiftOpcode::Shr => binop!(ShiftRight, ()),
            zkevm_opcode_defs::ShiftOpcode::Rol => binop!(RotateLeft, ()),
            zkevm_opcode_defs::ShiftOpcode::Ror => binop!(RotateRight, ()),
        },
        zkevm_opcode_defs::Opcode::Jump(_) => Instruction::from_jump(src1, predicate),
        zkevm_opcode_defs::Opcode::Context(x) => match x {
            zkevm_opcode_defs::ContextOpcode::This => {
                Instruction::from_this(out.try_into().unwrap(), predicate)
            }
            zkevm_opcode_defs::ContextOpcode::Caller => {
                Instruction::from_caller(out.try_into().unwrap(), predicate)
            }
            zkevm_opcode_defs::ContextOpcode::CodeAddress => {
                Instruction::from_code_address(out.try_into().unwrap(), predicate)
            }
            zkevm_opcode_defs::ContextOpcode::ErgsLeft => {
                Instruction::from_ergs_left(out.try_into().unwrap(), predicate)
            }
            zkevm_opcode_defs::ContextOpcode::GetContextU128 => {
                Instruction::from_context_u128(out.try_into().unwrap(), predicate)
            }
            zkevm_opcode_defs::ContextOpcode::SetContextU128 => {
                Instruction::from_set_context_u128(src1.try_into().unwrap(), predicate)
            }
            /*zkevm_opcode_defs::ContextOpcode::Meta => ,
            zkevm_opcode_defs::ContextOpcode::Sp => ,
            zkevm_opcode_defs::ContextOpcode::SetErgsPerPubdataByte => ,
            zkevm_opcode_defs::ContextOpcode::IncrementTxNumber => ,*/
            x => unimplemented_instruction(zkevm_opcode_defs::Opcode::Context(x)),
        },
        zkevm_opcode_defs::Opcode::Ptr(x) => match x {
            zkevm_opcode_defs::PtrOpcode::Add => ptr!(PtrAdd),
            zkevm_opcode_defs::PtrOpcode::Sub => ptr!(PtrSub),
            zkevm_opcode_defs::PtrOpcode::Pack => ptr!(PtrPack),
            zkevm_opcode_defs::PtrOpcode::Shrink => ptr!(PtrShrink),
        },
        zkevm_opcode_defs::Opcode::NearCall(_) => Instruction::from_near_call(
            Register1(Register::new(parsed.src0_reg_idx)),
            Immediate1(parsed.imm_0),
            Immediate2(parsed.imm_1),
            predicate,
        ),
        zkevm_opcode_defs::Opcode::FarCall(kind) => match kind {
            zkevm_opcode_defs::FarCallOpcode::Normal => Instruction::from_far_call(
                src1.try_into().unwrap(),
                src2,
                Immediate1(parsed.imm_0),
                false,
                predicate,
            ),
            x => unimplemented_instruction(zkevm_opcode_defs::Opcode::FarCall(x)),
            //zkevm_opcode_defs::FarCallOpcode::Delegate => todo!(),
            //zkevm_opcode_defs::FarCallOpcode::Mimic => todo!(),
        },
        zkevm_opcode_defs::Opcode::Ret(kind) => match kind {
            zkevm_opcode_defs::RetOpcode::Ok => {
                Instruction::from_ret(src1.try_into().unwrap(), predicate)
            }
            /*zkevm_opcode_defs::RetOpcode::Revert => ,
            zkevm_opcode_defs::RetOpcode::Panic => ,*/
            x => unimplemented_instruction(zkevm_opcode_defs::Opcode::Ret(x)),
        },
        zkevm_opcode_defs::Opcode::Log(x) => match x {
            zkevm_opcode_defs::LogOpcode::StorageRead => Instruction::from_sload(
                src1.try_into().unwrap(),
                out.try_into().unwrap(),
                predicate,
            ),
            zkevm_opcode_defs::LogOpcode::StorageWrite => {
                Instruction::from_sstore(src1.try_into().unwrap(), src2, predicate)
            }
            /*zkevm_opcode_defs::LogOpcode::ToL1Message => ,
            zkevm_opcode_defs::LogOpcode::Event => ,*/
            zkevm_opcode_defs::LogOpcode::PrecompileCall => {
                Instruction::from_precompile_call(src1.try_into().unwrap(), src2, predicate)
            }
            x => unimplemented_instruction(zkevm_opcode_defs::Opcode::Log(x)),
        },
        zkevm_opcode_defs::Opcode::UMA(x) => {
            let increment = parsed.variant.flags[UMA_INCREMENT_FLAG_IDX];
            match x {
                zkevm_opcode_defs::UMAOpcode::HeapRead => Instruction::from_load::<Heap>(
                    src1.try_into().unwrap(),
                    out.try_into().unwrap(),
                    increment.then_some(out2),
                    predicate,
                ),
                zkevm_opcode_defs::UMAOpcode::HeapWrite => Instruction::from_store::<Heap>(
                    src1.try_into().unwrap(),
                    src2,
                    increment.then_some(out.try_into().unwrap()),
                    predicate,
                ),
                zkevm_opcode_defs::UMAOpcode::AuxHeapRead => Instruction::from_load::<AuxHeap>(
                    src1.try_into().unwrap(),
                    out.try_into().unwrap(),
                    increment.then_some(out2),
                    predicate,
                ),
                zkevm_opcode_defs::UMAOpcode::AuxHeapWrite => Instruction::from_store::<AuxHeap>(
                    src1.try_into().unwrap(),
                    src2,
                    increment.then_some(out.try_into().unwrap()),
                    predicate,
                ),
                zkevm_opcode_defs::UMAOpcode::FatPointerRead => Instruction::from_load_pointer(
                    src1.try_into().unwrap(),
                    out.try_into().unwrap(),
                    increment.then_some(out2),
                    predicate,
                ),
            }
        }
        //zkevm_opcode_defs::Opcode::Invalid(_) => ,
        zkevm_opcode_defs::Opcode::Nop(_) => {
            let no_sp_movement = AdvanceStackPointer(RegisterAndImmediate {
                immediate: 0,
                register: Register::new(0),
            });
            Instruction::from_nop(
                if let AnySource::AdvanceStackPointer(pop) = src1 {
                    pop
                } else {
                    no_sp_movement.clone()
                },
                if let AnyDestination::AdvanceStackPointer(push) = out {
                    push
                } else {
                    no_sp_movement
                },
                predicate,
            )
        }

        x => unimplemented_instruction(x),
    }
}
