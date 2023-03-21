#[derive(Clone, Debug)]
pub struct Instruction
{
    instruction: ByteInstruction,
    arguments: Vec<String>,
    size_type: SizeType,
    register1: Option<Register>,
    register2: Option<Register>,
}
#[derive(Clone, Copy, Debug, PartialEq)]
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
    XMM0,
    XMM1,
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
impl SizeType {
    pub fn get_name(&self) -> String {
        match self
        {
            SizeType::BYTE | SizeType::CHAR => "byte".to_string(),
            SizeType::WORD => "word".to_string(),
            SizeType::DWORD => "dword".to_string(),
            SizeType::QWORD | SizeType::STRING => "qword".to_string(),
            _ => "".to_string()
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ByteInstruction
{
    Add,
    Sub,
    Mul,
    Div,
    And,
    Or,
    Xor,
    Not,
    Shl,
    Shr,
    Cmp,
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
    MovRegToMem,
    MovMemToReg,
    MovRegToReg,
    MovLitToReg,
    MovLitToMem,
    Swap,
    Sete,
    Neg,
    Syscall,
    Comment,
}
impl Instruction
{
    pub fn new(instruction: ByteInstruction, arguments: Vec<String>, size_type: SizeType) -> Instruction
    {
        Instruction { instruction, arguments, size_type, register1: None, register2: None }
    }
    pub fn push_argument(&mut self, argument: String)
    {
        self.arguments.push(argument);
    }
    pub fn is_same(&self, other: &Instruction) -> bool
    {
        if self.instruction != other.instruction
        {
            return false;
        }
        if self.size_type != other.size_type
        {
            return false;
        }
        if self.register1 != other.register1
        {
            return false;
        }
        if self.register2 != other.register2
        {
            return false;
        }
        if self.arguments.len() != other.arguments.len()
        {
            return false;
        }
        if self.arguments.len() != 0
        {
            for i in 0..self.arguments.len()
            {
                if self.arguments[i] != other.arguments[i]
                {
                    return false;
                }
            }
            return false;
        }
        return true;
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