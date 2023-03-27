// Description: Optimizer for the Bytecode
//
use super::ByteArray;
use super::ByteInstruction;
pub fn optimize(bytecode: &mut ByteArray)
{
    let mut i = bytecode.data.len() - 1;
    while i > 0
    {
        let current = bytecode.data[i].clone();
        let previous = bytecode.data[i - 1].clone();
        if current.get_instruction() == ByteInstruction::Pop && previous.get_instruction() == ByteInstruction::Push
        {
            if current.get_register(1) == previous.get_register(1)
            {
                bytecode.data.remove(i);
                bytecode.data.remove(i - 1);
                i -= 2;
            }
        }
        i -= 1;
    }
}
