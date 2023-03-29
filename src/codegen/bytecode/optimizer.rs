// Description: Optimizer for the Bytecode
//
use super::ByteArray;
use super::ByteInstruction;
use super::Instruction;
use super::Register;
pub fn optimize(bytecode: &mut ByteArray)
{

    optimize_push_pop_simple(bytecode);
    optimize_moves(bytecode);
    optimize_unnecesary_mov(bytecode);
}
fn optimize_unnecesary_mov(bytecode: &mut ByteArray)
{
    // TODO: Not done
    let mut i = bytecode.data.len() - 1;
    while i > 0
    {
        let current = bytecode.data[i].clone();
        if current.get_instruction() != ByteInstruction::MovMemToReg
        {
            i -= 1;
            continue;
        }
        let mut j = i -1;
        while j > 0
        {
            let curr = bytecode.data[j].clone();
            if curr.get_instruction() == ByteInstruction::MovRegToMem || curr.get_instruction() == ByteInstruction::MovLitToMem
            {
                if is_same_position(curr.clone(), current.clone())
                {
                    break;
                }
            }
            if is_modifiing_rax(curr.clone())
            {
                if curr.is_same(&current)
                {
                    bytecode.data.remove(i);
                    break;
                }
                break;
            }
            j -= 1;
        }
        i -= 1;
    }
}
fn optimize_moves(bytecode: &mut ByteArray)
{
    let mut i = bytecode.data.len() - 1;
    while i > 0
    {
        let current = bytecode.data[i].clone();
        if current.get_instruction() != ByteInstruction::MovRegToReg
        {
            i-=1;
            continue;
        }
        if current.get_register(1).unwrap() == current.get_register(2).unwrap()
        {
            bytecode.data.remove(i);
        }
        i-=1;
    }
}
fn is_same_position(i1: Instruction, i2: Instruction) -> bool
{
    let i1_r2: Register;
    if i1.get_instruction() == ByteInstruction::MovLitToMem
    {
        i1_r2 = i1.get_register(1).unwrap();
    }
    else {
        i1_r2 = i1.get_register(2).unwrap();
    }
    let i1_arguments = i1.get_arguments();
    let i1_offset = i1_arguments.get(0).unwrap();
    let i2_r1 = i2.get_register(1).unwrap();
    let i2_arguments = i2.get_arguments();
    let i2_offset = i2_arguments.get(0).unwrap();
    if i1_r2 != i2_r1
    {
        return false;
    }
    if i1_offset != i2_offset
    {
        return false;
    }
    return true;
}
fn is_modifiing_rax(instruction: Instruction) -> bool
{
    let is_set_inst = match instruction.get_instruction()
    {
        ByteInstruction::Sete => true,
        ByteInstruction::Setne => true,
        ByteInstruction::Slt => true,
        ByteInstruction::Sle => true,
        ByteInstruction::Sgt => true,
        ByteInstruction::Sge => true,
        _ => false,
    };
    if is_set_inst
    {
        return true;
    }
    let reg = instruction.get_register(2);
    if reg.is_none()
    {
        return false;
    }
    let reg = reg.unwrap();
    if reg != Register::RAX
    {
        return false;
    }
    return true;
}
fn optimize_push_pop_simple(bytecode: &mut ByteArray)
{
    let mut i = bytecode.data.len() - 1;
    while i > 0
    {
        let current = bytecode.data[i].clone();
        let previous = bytecode.data[i - 1].clone();
        if current.get_instruction() == ByteInstruction::Pop && previous.get_instruction() == ByteInstruction::Push
        {
            let before = bytecode.data.clone()[..(i-1)].to_vec();
            let mut after = bytecode.data.clone()[(i+1)..].to_vec();
            bytecode.data = before;
            let start = previous.get_register(1).unwrap();
            let end = current.get_register(1).unwrap();
            bytecode.add_move_reg_to_reg(start, end, super::SizeType::QWORD);
            bytecode.data.append(&mut after);
            i -= 1;
        }
        i -= 1;
    }
}
