pub fn syscall() -> String
{
    format!("int 0x80\n")
}