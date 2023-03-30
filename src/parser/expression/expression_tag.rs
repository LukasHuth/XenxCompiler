#[derive(Clone, PartialEq, Debug)]
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
    FloatLiteral,
    OverwriteArrayExpr,
    ArrayExpr,
    ForExpression,
    WhileExpression,
    ContinueExpression,
    BreakExpression,
}
