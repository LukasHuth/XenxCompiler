use std::mem::ManuallyDrop;
use super::{
    BinaryExpression,
    UnaryExpression,
    IfExpression,
    ReturnExpression,
    CallExpression,
    AssignmentExpression,
    OverwriteVariableExpression,
    ArgVariableExpression,
    FunctionDeclarationExpression
};
#[repr(C)]
// #[derive(Clone)]
pub union Syntax
{
    pub integer_literal: i32,
    pub boolean_literal: bool,
    pub string_literal: ManuallyDrop<String>,
    pub variable_expr: ManuallyDrop<String>,
    pub binary_expr: ManuallyDrop<BinaryExpression>,
    pub unary_expr: ManuallyDrop<UnaryExpression>,
    pub call_expr: ManuallyDrop<CallExpression>,
    pub assignment_expr: ManuallyDrop<AssignmentExpression>,
    pub return_expr: ManuallyDrop<ReturnExpression>,
    pub arg_variable_expr: ManuallyDrop<ArgVariableExpression>,
    pub function_declaration_expr: ManuallyDrop<FunctionDeclarationExpression>,
    pub if_expr: ManuallyDrop<IfExpression>,
    pub overwrite_variable_expr: ManuallyDrop<OverwriteVariableExpression>,
}
#[allow(unreachable_patterns)]
impl Clone for Syntax {
    fn clone(&self) -> Self {
        unsafe {
            match self {
                Syntax { integer_literal } => Syntax {
                    integer_literal: *integer_literal,
                },
                Syntax { string_literal } => Syntax {
                    string_literal: (*string_literal).clone(),
                },
                Syntax { variable_expr } => Syntax {
                    variable_expr: (*variable_expr).clone(),
                },
                Syntax { binary_expr } => Syntax {
                    binary_expr: (*binary_expr).clone(),
                },
                Syntax { unary_expr } => Syntax {
                    unary_expr: (*unary_expr).clone(),
                },
                Syntax { call_expr } => Syntax {
                    call_expr: (*call_expr).clone(),
                },
                Syntax { assignment_expr } => Syntax {
                    assignment_expr: (*assignment_expr).clone(),
                },
                Syntax { return_expr } => Syntax {
                    return_expr: (*return_expr).clone(),
                },
                Syntax { arg_variable_expr } => Syntax {
                    arg_variable_expr: (*arg_variable_expr).clone(),
                },
                Syntax { function_declaration_expr } => Syntax {
                    function_declaration_expr: (*function_declaration_expr).clone(),
                },
                Syntax { if_expr } => Syntax {
                    if_expr: (*if_expr).clone(),
                },
                Syntax { overwrite_variable_expr: override_variable_expr } => Syntax {
                    overwrite_variable_expr: (*override_variable_expr).clone(),
                },
            }
        }
    }
}