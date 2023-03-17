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
    pub fn add(&mut self, instruction: Instruction){
        self.data.push(instruction);
    }
    pub fn add_byte(&mut self, instruction: ByteInstruction, arguments: Vec<String>, size: SizeType){
        self.data.push(Instruction::new(instruction, arguments, size));
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
    pub fn add_store_variable(&mut self, position: &str, value: &str, size: SizeType){
        self.add_byte(ByteInstruction::StoreVariable, vec![position.to_string(), value.to_string()], size);
    }
    pub fn add_load_constant(&mut self, name: &str, size: SizeType){
        self.add_byte(ByteInstruction::LoadConstant, vec![name.to_string()], size);
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
    }
    pub fn add_sub(&mut self, size: SizeType){
        self.add_byte(ByteInstruction::Sub, Vec::new(), size);
    }
    pub fn add_mul(&mut self, size: SizeType){
        self.add_byte(ByteInstruction::Mul, Vec::new(), size);
    }
    pub fn add_div(&mut self, size: SizeType){
        self.add_byte(ByteInstruction::Div, Vec::new(), size);
    }
    pub fn add_mod(&mut self, size: SizeType){
        self.add_byte(ByteInstruction::Mod, Vec::new(), size);
    }
    pub fn add_and(&mut self, size: SizeType){
        self.add_byte(ByteInstruction::And, Vec::new(), size);
    }
    pub fn add_or(&mut self, size: SizeType){
        self.add_byte(ByteInstruction::Or, Vec::new(), size);
    }
    pub fn add_xor(&mut self, size: SizeType){
        self.add_byte(ByteInstruction::Xor, Vec::new(), size);
    }
    pub fn add_not(&mut self, size: SizeType){
        self.add_byte(ByteInstruction::Not, Vec::new(), size);
    }
    pub fn add_shl(&mut self, size: SizeType){
        self.add_byte(ByteInstruction::Shl, Vec::new(), size);
    }
    pub fn add_shr(&mut self, size: SizeType){
        self.add_byte(ByteInstruction::Shr, Vec::new(), size);
    }
    pub fn add_cmp(&mut self, size: SizeType){
        self.add_byte(ByteInstruction::Cmp, Vec::new(), size);
    }
    pub fn add_move(&mut self, from: Register, to: Register, size: SizeType){
        self.add_byte(ByteInstruction::Mov, Vec::new(), size);
        self.set_register_in_last_instruction(from, 1);
        self.set_register_in_last_instruction(to, 2);
    }
    pub fn add_push(&mut self){
        self.add_byte(ByteInstruction::Push, Vec::new(), None);
    }
    pub fn add_pop(&mut self, register: Register, size: SizeType){
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
    fn set_register_in_last_instruction(&mut self, register: Register, pos: u32){
        self.data.last_mut().unwrap().set_register(register, pos);
    }
    pub fn get_data(&self) -> Vec<Instruction>{
        self.data.clone()
    }
    pub fn generate(&self, os: super::OS) -> String
    {
        util::generate(self.data.clone(), os)
    }
}