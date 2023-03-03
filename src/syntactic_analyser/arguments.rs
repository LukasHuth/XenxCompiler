use super::Statement;
pub struct Arguments
{
    pub arguments: Vec<Statement>,
}
impl Arguments
{
    pub fn new(arguments: Vec<Statement>) -> Arguments
    {
        Arguments
        {
            arguments,
        }
    }
    pub fn push(&mut self, argument: Statement)
    {
        self.arguments.push(argument);
    }
}