#[derive(Clone, Debug)]
pub struct Instruction
{
    instruction: ByteInstruction,
    arguments: Vec<String>,
    size_type: SizeType,
    register1: Option<Register>,
    register2: Option<Register>,
}
#[derive(Clone, Copy, Debug)]
pub enum Register
{
    RAX,
    RBX,
    RCX,
    RDX,
    RSI,
    RDI,
    RBP,
    RSP,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SizeType
{
    BYTE,
    WORD,
    DWORD,
    QWORD,
    FLOAT,
    DOUBLE,
    CHAR,
    STRING,
    None,
}
#[derive(Clone, Copy, Debug)]
pub enum ByteInstruction
{
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Xor,
    Not,
    Shl,
    Shr,
    Cmp,
    Mov,
    Push,
    Pop,
    Ret,
    Call,
    Jmp,
    Jz,
    Jnz,
    Je,
    Jne,
    Jg,
    Jl,
    Jge,
    Jle,
    Jn, // Jump Negative
    LoadVariable,
    StoreVariable,
    LoadConstant,
    StoreConstant,
    LoadArgument,
    StoreArgument,
    StoreValue,
    LoadValue,
    External,
    Section,
    Entry,
    Global,
}
impl Instruction
{
    pub fn new(instruction: ByteInstruction, arguments: Vec<String>, size_type: SizeType) -> Instruction
    {
        Instruction { instruction, arguments, size_type, register1: None, register2: None }
    }
    pub fn set_register(&mut self, register: Register, pos: u32)
    {
        match pos
        {
            1 => self.register1 = Some(register),
            2 => self.register2 = Some(register),
            _ => panic!("Invalid register position"),
        }
    }
    pub fn get_instruction(&self) -> ByteInstruction
    {
        self.instruction
    }
    pub fn get_arguments(&self) -> Vec<String>
    {
        self.arguments.clone()
    }
    pub fn get_size_type(&self) -> SizeType
    {
        self.size_type
    }
    pub fn get_register(&self, pos: u32) -> Option<Register>
    {
        match pos
        {
            1 => self.register1,
            2 => self.register2,
            _ => panic!("Invalid register position"),
        }
    }
}