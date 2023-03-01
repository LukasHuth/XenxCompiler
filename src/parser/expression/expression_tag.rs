#[derive(Clone)]
pub enum ExpressionTag
{
    IntegerLiteral,
    StringLiteral,
    BooleanLiteral,
    VariableExpr,
    BinaryExpr,
    UnaryExpr,
    CallExpr,
    AssignmentExpr,
    ReturnExpr,
    ArgVariableExpr,
    FunctionDeclarationExpr,
    IfExpr,
    OverwriteVariableExpr,
}