pub struct AST
{
    pub root: Node,
}
pub struct Node
{
    pub name: String,
    pub children: Vec<Node>,
}
impl AST
{
    pub fn new() -> AST
    {
        AST { root: Node::new() }
    }
}
impl Node
{
    pub fn new() -> Node
    {
        Node { name: "".to_string(), children: Vec::new() }
    }
    pub fn add_child(&mut self, child: Node)
    {
        self.children.push(child);
    }
    pub fn add_child_with_name(&mut self, name: String)
    {
        self.children.push(Node::new_with_name(name));
    }
    pub fn new_with_name(name: String) -> Node
    {
        Node { name: name, children: Vec::new() }
    }
    pub fn to_string(&self) -> String
    {
        let mut string = String::new();
        string.push_str(&self.name);
        string
    }
}