use super::ByteArray;
pub fn generate_constant_functions(bytecode: &mut ByteArray)
{
    bytecode.add_store_constant_string("format_int", "%d\\n");
    bytecode.add_store_constant_string("format_string", "%s\\n");
    bytecode.add_store_constant_string("format_bool", "%d\\n");
    bytecode.add_store_constant_string("format_char", "%c\\n");
    bytecode.add_store_constant_string("format_float", "%f\\n");
}