mod instruction;
pub mod bytecode_util;
use bytecode_util as util;
pub use instruction::{
    Instruction,
    ByteInstruction,
    SizeType,
    SizeType::None,
    Register,
};
mod optimizer;
#[derive(Clone, Debug)]
pub struct ByteArray{
    data: Vec<Instruction>,
}
impl ByteArray{
    pub fn new() -> ByteArray{
        ByteArray{
            data: Vec::new(),
        }
    }
    pub fn add_array(&mut self, array: &ByteArray){
        for i in 0..array.data.len(){
            self.data.push(array.data[i].clone());
        }
    }
    pub fn optimize(&mut self){
        optimizer::optimize(self);
    }
    pub fn is_same(&self, other: &ByteArray) -> bool{
        if self.data.len() != other.data.len(){
            return false;
        }
        for i in 0..self.data.len(){
            if !self.data[i].is_same(&other.data[i]){
                return false;
            }
        }
        return true;
    }
    pub fn add(&mut self, instruction: Instruction){
        self.data.push(instruction);
    }
    pub fn add_byte(&mut self, instruction: ByteInstruction, arguments: Vec<String>, size: SizeType){
        self.data.push(Instruction::new(instruction, arguments, size));
    }
    pub fn add_comment(&mut self, comment: &str){
        self.add_byte(ByteInstruction::Comment, vec![comment.to_string()], None);
    }
    pub fn add_external(&mut self, name: &str){
        self.add_byte(ByteInstruction::External, vec![name.to_string()], None);
    }
    pub fn add_section(&mut self, name: &str){
        self.add_byte(ByteInstruction::Section, vec![name.to_string()], None);
    }
    pub fn add_entry(&mut self, name: &str){
        self.add_byte(ByteInstruction::Entry, vec![name.to_string()], None);
    }
    pub fn add_load_variable(&mut self, name: &str, size: SizeType){
        self.add_byte(ByteInstruction::LoadVariable, vec![name.to_string()], size);
    }
    pub fn add_global(&mut self, name: &str){
        self.add_byte(ByteInstruction::Global, vec![name.to_string()], None);
    }
    pub fn add_store_variable(&mut self, position: &str, size: SizeType){
        self.add_byte(ByteInstruction::StoreVariable, vec![position.to_string()], size);
    }
    pub fn add_load_constant(&mut self, name: &str, size: SizeType){
        self.add_byte(ByteInstruction::LoadConstant, vec![name.to_string()], size);
        self.set_register_in_last_instruction(Register::RAX, 1);
    }
    pub fn add_store_constant(&mut self, name: &str, value: &str, size: SizeType){
        self.add_byte(ByteInstruction::StoreConstant, vec![name.to_string(), value.to_string()], size);
    }
    pub fn add_store_constant_string(&mut self, name: &str, value: &str){
        self.add_byte(ByteInstruction::StoreConstant, vec![name.to_string(), value.to_string()], SizeType::STRING);
    }
    pub fn add_load_argument(&mut self, name: &str, size: SizeType){
        self.add_byte(ByteInstruction::LoadArgument, vec![name.to_string()], size);
    }
    pub fn add_store_argument(&mut self, name: &str, size: SizeType){
        self.add_byte(ByteInstruction::StoreArgument, vec![name.to_string()], size);
    }
    pub fn add_add(&mut self, size: SizeType){
        self.add_byte(ByteInstruction::Add, Vec::new(), size);
        self.set_register_in_last_instruction(Register::RAX, 1);
        self.set_register_in_last_instruction(Register::RBX, 2);
    }
    pub fn add_sub(&mut self, size: SizeType){
        self.add_byte(ByteInstruction::Sub, Vec::new(), size);
        self.set_register_in_last_instruction(Register::RAX, 1);
        self.set_register_in_last_instruction(Register::RBX, 2);
    }
    pub fn add_sub_reg(&mut self,to: Register, from: Register, size: SizeType){
        self.add_byte(ByteInstruction::Sub, Vec::new(), size);
        self.set_register_in_last_instruction(to, 1);
        self.set_register_in_last_instruction(from, 2);
    }
    pub fn add_mul(&mut self, size: SizeType){
        self.add_byte(ByteInstruction::Mul, Vec::new(), size);
        self.set_register_in_last_instruction(Register::RBX, 1);
    }
    pub fn add_div(&mut self, size: SizeType){
        self.add_byte(ByteInstruction::Div, Vec::new(), size);
        self.set_register_in_last_instruction(Register::RBX, 1);
    }
    pub fn add_and(&mut self, size: SizeType){
        self.add_byte(ByteInstruction::And, Vec::new(), size);
        self.set_register_in_last_instruction(Register::RAX, 1);
        self.set_register_in_last_instruction(Register::RBX, 2);
    }
    pub fn add_or(&mut self, size: SizeType){
        self.add_byte(ByteInstruction::Or, Vec::new(), size);
        self.set_register_in_last_instruction(Register::RAX, 1);
        self.set_register_in_last_instruction(Register::RBX, 2);
    }
    pub fn add_xor(&mut self, size: SizeType){
        self.add_byte(ByteInstruction::Xor, Vec::new(), size);
        self.set_register_in_last_instruction(Register::RAX, 1);
        self.set_register_in_last_instruction(Register::RBX, 2);
    }
    pub fn add_xor_reg(&mut self, r1: Register, r2: Register, size: SizeType){
        self.add_byte(ByteInstruction::Xor, Vec::new(), size);
        self.set_register_in_last_instruction(r1, 1);
        self.set_register_in_last_instruction(r2, 2);
    }
    pub fn add_not(&mut self, size: SizeType){
        self.add_byte(ByteInstruction::Not, Vec::new(), size);
        self.set_register_in_last_instruction(Register::RAX, 1);
        self.set_register_in_last_instruction(Register::RBX, 2);
    }
    pub fn add_shl(&mut self, size: SizeType){
        self.add_byte(ByteInstruction::Shl, Vec::new(), size);
        self.set_register_in_last_instruction(Register::RAX, 1);
        self.set_register_in_last_instruction(Register::RBX, 2);
    }
    pub fn add_shr(&mut self, size: SizeType){
        self.add_byte(ByteInstruction::Shr, Vec::new(), size);
        self.set_register_in_last_instruction(Register::RAX, 1);
        self.set_register_in_last_instruction(Register::RBX, 2);
    }
    fn add_cmp(&mut self, size: SizeType){
        self.add_byte(ByteInstruction::Cmp, Vec::new(), size);
    }
    pub fn add_cmp_reg(&mut self, source: Register, destination: Register, size: SizeType){
        self.add_cmp(size);
        self.set_register_in_last_instruction(source, 2);
        self.set_register_in_last_instruction(destination, 1);
    }
    pub fn add_cmp_let(&mut self, source: String, destination: Register, size: SizeType){
        self.add_cmp(size);
        let length = self.data.len();
        self.data[length - 1].push_argument(source);
        self.set_register_in_last_instruction(destination, 1);
    }
    pub fn add_move_reg_to_reg(&mut self, from: Register, to: Register, size: SizeType){
        self.add_byte(ByteInstruction::MovRegToReg, Vec::new(), size);
        self.set_register_in_last_instruction(from, 1);
        self.set_register_in_last_instruction(to, 2);
    }
    pub fn add_move_mem_to_reg(&mut self, from: Register, offset: &str, to: Register, size: SizeType){ //////
        self.add_byte(ByteInstruction::MovMemToReg, vec![offset.to_string()], size);
        self.set_register_in_last_instruction(from, 1);
        self.set_register_in_last_instruction(to, 2);
    }
    pub fn add_move_lit_to_reg(&mut self, value: &str, to: Register, size: SizeType){
        self.add_byte(ByteInstruction::MovLitToReg, vec![value.to_string()], size);
        self.set_register_in_last_instruction(to, 1);
    }
    pub fn add_move_lit_from_reg_to_reg(&mut self, offset: &str, from: Register, to: Register, size: SizeType){
        self.add_byte(ByteInstruction::MovLitToReg, vec![offset.to_string()], size);
        self.set_register_in_last_instruction(from, 1);
        self.set_register_in_last_instruction(to, 1);
    }
    pub fn add_move_reg_to_mem(&mut self, from: Register, offset: &str, to: Register, size: SizeType){
        self.add_byte(ByteInstruction::MovRegToMem, vec![offset.to_string()], size);
        self.set_register_in_last_instruction(from, 1);
        self.set_register_in_last_instruction(to, 2);
    }
    pub fn add_move_lit_to_mem(&mut self, offset: &str, literal: &str, r1: Register, byte: SizeType) {
        self.add_byte(ByteInstruction::MovLitToMem, vec![offset.to_string(), literal.to_string()], byte);
        self.set_register_in_last_instruction(r1, 1);
    }
    pub fn add_push(&mut self){
        self.add_byte(ByteInstruction::Push, Vec::new(), SizeType::QWORD);
        self.set_register_in_last_instruction(Register::RAX, 1);
    }
    pub fn add_push_reg(&mut self, register: Register){
        self.add_byte(ByteInstruction::Push, Vec::new(), SizeType::QWORD);
        self.set_register_in_last_instruction(register, 1);
    }
    pub fn add_pop(&mut self, register: Register){
        self.add_byte(ByteInstruction::Pop, Vec::new(), SizeType::QWORD);
        self.set_register_in_last_instruction(register, 1);
    }
    pub fn add_pop_size(&mut self, register: Register, size: SizeType){
        self.add_byte(ByteInstruction::Pop, Vec::new(), size);
        self.set_register_in_last_instruction(register, 1);
    }
    pub fn add_ret(&mut self){
        self.add_byte(ByteInstruction::Ret, Vec::new(), None);
    }
    pub fn add_call(&mut self, name: &str){
        self.add_byte(ByteInstruction::Call, vec![name.to_string()], None);
    }
    pub fn add_jmp(&mut self, name: &str){
        self.add_byte(ByteInstruction::Jmp, vec![name.to_string()], None);
    }
    pub fn add_jmp_if_eq(&mut self, name: &str){
        self.add_byte(ByteInstruction::Je, vec![name.to_string()], None);
    }
    pub fn add_jmp_if_not_eq(&mut self, name: &str){
        self.add_byte(ByteInstruction::Jne, vec![name.to_string()], None);
    }
    pub fn add_jmp_if_less(&mut self, name: &str){
        self.add_byte(ByteInstruction::Jl, vec![name.to_string()], None);
    }
    pub fn add_jmp_if_less_eq(&mut self, name: &str){
        self.add_byte(ByteInstruction::Jle, vec![name.to_string()], None);
    }
    pub fn add_jmp_if_greater(&mut self, name: &str){
        self.add_byte(ByteInstruction::Jg, vec![name.to_string()], None);
    }
    pub fn add_jmp_if_greater_eq(&mut self, name: &str){
        self.add_byte(ByteInstruction::Jge, vec![name.to_string()], None);
    }
    pub fn add_jmp_if_zero(&mut self, name: &str){
        self.add_byte(ByteInstruction::Jz, vec![name.to_string()], None);
    }
    pub fn add_jmp_if_not_zero(&mut self, name: &str){
        self.add_byte(ByteInstruction::Jnz, vec![name.to_string()], None);
    }
    pub fn add_jmp_if_negative(&mut self, name: &str){
        self.add_byte(ByteInstruction::Jn, vec![name.to_string()], None);
    }
    pub fn add_swap(&mut self, register1: Register, register2: Register){
        self.add_byte(ByteInstruction::Swap, Vec::new(), SizeType::QWORD);
        self.set_register_in_last_instruction(register1, 1);
        self.set_register_in_last_instruction(register2, 2);
    }
    pub fn add_sub_lit(&mut self, lit: &str, size: SizeType){
        self.add_byte(ByteInstruction::Sub, vec![lit.to_string()], size);
        self.set_register_in_last_instruction(Register::RAX, 1);
    }
    pub fn add_sub_lit_reg(&mut self, lit: &str, reg: Register, size: SizeType){
        self.add_byte(ByteInstruction::Sub, vec![lit.to_string()], size);
        self.set_register_in_last_instruction(reg, 1);
    }
    pub fn add_set_equal(&mut self, reg: Register){
        self.add_byte(ByteInstruction::Sete, Vec::new(), SizeType::QWORD);
        self.set_register_in_last_instruction(reg, 1);
    }
    pub fn add_set_if_less_that(&mut self, reg: Register){
        self.add_byte(ByteInstruction::Slt, Vec::new(), SizeType::QWORD);
        self.set_register_in_last_instruction(reg, 1);
    }
    pub fn add_neg(&mut self, reg: Register){
        self.add_byte(ByteInstruction::Neg, Vec::new(), SizeType::QWORD);
        self.set_register_in_last_instruction(reg, 1);
    }
    pub fn add_syscall(&mut self)
    {
        self.add_byte(ByteInstruction::Syscall, Vec::new(), None);
    }
    pub fn add_clear(&mut self, register: Register)
    {
        self.add_byte(ByteInstruction::Xor, vec![], SizeType::QWORD);
        self.set_register_in_last_instruction(register, 1);
        self.set_register_in_last_instruction(register, 2);
    }
    fn set_register_in_last_instruction(&mut self, register: Register, pos: u32){
        self.data.last_mut().unwrap().set_register(register, pos);
    }
    pub fn get_data(&self) -> Vec<Instruction>{
        self.data.clone()
    }
    pub fn get_instruction_at(&self, index: usize) -> Instruction{
        self.data[index].clone()
    }
    pub fn remove_instruction_at(&mut self, index: usize){
        self.data.remove(index);
    }
    pub fn generate(&self, os: super::OS, comments: bool) -> String
    {
        util::generate(self.data.clone(), os, comments)
    }
}
