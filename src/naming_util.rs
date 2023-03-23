pub fn get_keywords() -> Vec<String>
{
    let mut keywords = KeywordArray::new();
    keywords.insert("if");
    keywords.insert("else");
    keywords.insert("while");
    keywords.insert("for");
    keywords.insert("return");
    keywords.insert("break");
    keywords.insert("continue");
    keywords.insert("func");
    keywords.insert("struct");
    keywords.insert("import");
    keywords.insert("as");
    keywords.insert("null");
    keywords.insert("float");
    keywords.insert("int");
    keywords.insert("bool");
    keywords.insert("string");
    keywords.insert("char");
    keywords.insert("namespace");
    return keywords.array;
}
struct KeywordArray
{
    array: Vec<String>
}
impl KeywordArray
{
    pub fn new() -> KeywordArray
    {
        return KeywordArray { array: Vec::<String>::new() };
    }
    pub fn insert(&mut self, str: &str)
    {
        self.array.push(str.to_string());
    }
}