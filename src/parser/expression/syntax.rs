use super::{
    BinaryExpression,
    UnaryExpression,
    IfExpression,
    ReturnExpression,
    CallExpression,
    AssignmentExpression,
    OverwriteVariableExpression,
    ArgVariableExpression,
    FunctionDeclarationExpression,
    VariableExpression,
    ArrayExpression,
    OverwriteArrayExpression,
    ForExpression, WhileExpression,
};
#[derive(PartialEq)]
pub struct Syntax
{
    pub integer_literal: Option<i32>,
    pub boolean_literal: Option<bool>,
    pub float_literal: Option<f32>,
    pub string_literal: Option<String>,
    pub variable_expr: Option<VariableExpression>,
    pub binary_expr: Option<BinaryExpression>,
    pub unary_expr: Option<UnaryExpression>,
    pub call_expr: Option<CallExpression>,
    pub assignment_expr: Option<AssignmentExpression>,
    pub return_expr: Option<ReturnExpression>,
    pub arg_variable_expr: Option<ArgVariableExpression>,
    pub function_declaration_expr: Option<FunctionDeclarationExpression>,
    pub if_expr: Option<IfExpression>,
    pub overwrite_variable_expr: Option<OverwriteVariableExpression>,
    pub overwrite_array_expr: Option<OverwriteArrayExpression>,
    pub array_expr: Option<ArrayExpression>,
    pub for_expression: Option<ForExpression>,
    pub while_expression: Option<WhileExpression>,
    pub type_: SyntaxType,
}
#[derive(Clone, Copy, PartialEq)]
pub enum SyntaxType
{
    IntegerLiteral,
    BooleanLiteral,
    FloatLiteral,
    StringLiteral,
    VariableExpression,
    BinaryExpression,
    UnaryExpression,
    CallExpression,
    AssignmentExpression,
    ReturnExpression,
    ArgVariableExpression,
    FunctionDeclarationExpression,
    IfExpression,
    OverwriteVariableExpression,
    OverwriteArrayExpression,
    ArrayExpression,
    ForExpression,
    WhileExpression,
    Empty,
}
#[allow(unreachable_patterns)]
impl Clone for Syntax {
    fn clone(&self) -> Self {
        let mut temp = Self::new(
            self.type_.clone(),
            self.integer_literal.clone(),
            self.boolean_literal.clone(),
            self.float_literal.clone(),
            self.string_literal.clone(),
            self.variable_expr.clone(),
            self.binary_expr.clone(),
            self.unary_expr.clone(),
            self.call_expr.clone(),
            self.assignment_expr.clone(),
            self.return_expr.clone(),
            self.arg_variable_expr.clone(),
            self.function_declaration_expr.clone(),
            self.if_expr.clone(),
            self.overwrite_variable_expr.clone(),
            self.overwrite_array_expr.clone(),
            self.array_expr.clone(),
            self.for_expression.clone()
        );
        temp.while_expression = self.while_expression.clone();
        return temp;
    }
}
impl Syntax
{
    pub fn new (type_: SyntaxType,integer_literal: Option<i32>,boolean_literal: Option<bool>,float_literal:Option<f32>, string_literal: Option<String>,
        variable_expr: Option<VariableExpression>, binary_expr: Option<BinaryExpression>, unary_expr: Option<UnaryExpression>, call_expr: Option<CallExpression>, assignment_expr: Option<AssignmentExpression>,
        return_expr: Option<ReturnExpression>, arg_variable_expr: Option<ArgVariableExpression>, function_declaration_expr: Option<FunctionDeclarationExpression>,
        if_expr: Option<IfExpression>, overwrite_variable_expr: Option<OverwriteVariableExpression>, overwrite_array_expr: Option<OverwriteArrayExpression>,
        array_expr: Option<ArrayExpression>,for_expression: Option<ForExpression>) -> Syntax
    {
        let while_expression: Option<WhileExpression> = None;
        Syntax {
            integer_literal,
            boolean_literal,
            float_literal,
            string_literal,
            variable_expr,
            binary_expr,
            unary_expr,
            call_expr,
            assignment_expr,
            return_expr,
            arg_variable_expr,
            function_declaration_expr,
            if_expr,
            overwrite_variable_expr,
            overwrite_array_expr,
            array_expr,
            for_expression,
            while_expression,
            type_,
        }
    }
    pub fn new_two_so_the_argument_list_is_not_that_long(type_: SyntaxType,while_expr: Option<WhileExpression>) -> Syntax
    {
        let mut temp = Self::new_empty(type_.clone());
        temp.while_expression = while_expr;
        return temp;
    }
    pub fn new_empty(type_: SyntaxType) -> Syntax
    {
        let integer_literal: Option<i32> = None;
        let boolean_literal: Option<bool> = None;
        let float_literal: Option<f32> = None;
        let string_literal: Option<String> = None;
        let variable_expr: Option<VariableExpression> = None;
        let binary_expr: Option<BinaryExpression> = None;
        let unary_expr: Option<UnaryExpression> = None;
        let call_expr: Option<CallExpression> = None;
        let assignment_expr: Option<AssignmentExpression> = None;
        let return_expr: Option<ReturnExpression> = None;
        let arg_variable_expr: Option<ArgVariableExpression> = None;
        let function_declaration_expr: Option<FunctionDeclarationExpression> = None;
        let if_expr: Option<IfExpression> = None;
        let overwrite_variable_expr: Option<OverwriteVariableExpression> = None;
        let overwrite_array_expr: Option<OverwriteArrayExpression> = None;
        let array_expr: Option<ArrayExpression> = None;
        let for_expression: Option<ForExpression> = None;
        let while_expression: Option<WhileExpression> = None;
        Syntax {
            integer_literal,
            boolean_literal,
            float_literal,
            string_literal,
            variable_expr,
            binary_expr,
            unary_expr,
            call_expr,
            assignment_expr,
            return_expr,
            arg_variable_expr,
            function_declaration_expr,
            if_expr,
            overwrite_variable_expr,
            overwrite_array_expr,
            array_expr,
            for_expression,
            while_expression,
            type_,
        }
    }
    pub fn new_while_expr(while_expr: WhileExpression) -> Syntax
    {
        Syntax::new_two_so_the_argument_list_is_not_that_long(SyntaxType::WhileExpression, Some(while_expr))
    }
    pub fn new_integer_literal(integer_literal: i32) -> Syntax
    {
        Syntax::new(SyntaxType::IntegerLiteral, Some(integer_literal), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None)
    }
    pub fn new_boolean_literal(boolean_literal: bool) -> Syntax
    {
        Syntax::new(SyntaxType::BooleanLiteral, None, Some(boolean_literal), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None)
    }
    pub fn new_float_literal(float_literal: f32) -> Syntax
    {
        Syntax::new(SyntaxType::FloatLiteral, None, None, Some(float_literal), None, None, None, None, None, None, None, None, None, None, None, None, None, None)
    }
    pub fn new_string_literal(string_literal: String) -> Syntax
    {
        Syntax::new(SyntaxType::StringLiteral, None, None, None, Some(string_literal), None, None, None, None, None, None, None, None, None, None, None, None, None)
    }
    pub fn new_variable_expr(variable_expr: VariableExpression) -> Syntax
    {
        Syntax::new(SyntaxType::VariableExpression, None, None, None, None, Some(variable_expr), None, None, None, None, None, None, None, None, None, None, None, None)
    }
    pub fn new_binary_expr(binary_expr: BinaryExpression) -> Syntax
    {
        Syntax::new(SyntaxType::BinaryExpression, None, None, None, None, None, Some(binary_expr), None, None, None, None, None, None, None, None, None, None, None)
    }
    pub fn new_unary_expr(unary_expr: UnaryExpression) -> Syntax
    {
        Syntax::new(SyntaxType::UnaryExpression, None, None, None, None, None, None, Some(unary_expr), None, None, None, None, None, None, None, None, None, None)
    }
    pub fn new_call_expr(call_expr: CallExpression) -> Syntax
    {
        Syntax::new(SyntaxType::CallExpression, None, None, None, None, None, None, None, Some(call_expr), None, None, None, None, None, None, None, None, None)
    }
    pub fn new_assignment_expr(assignment_expr: AssignmentExpression) -> Syntax
    {
        Syntax::new(SyntaxType::AssignmentExpression, None, None, None, None, None, None, None, None, Some(assignment_expr), None, None, None, None, None, None, None, None)
    }
    pub fn new_return_expr(return_expr: ReturnExpression) -> Syntax
    {
        Syntax::new(SyntaxType::ReturnExpression, None, None, None, None, None, None, None, None, None, Some(return_expr), None, None, None, None, None, None, None)
    }
    pub fn new_arg_variable_expr(arg_variable_expr: ArgVariableExpression) -> Syntax
    {
        Syntax::new(SyntaxType::ArgVariableExpression, None, None, None, None, None, None, None, None, None, None, Some(arg_variable_expr), None, None, None, None, None, None)
    }
    pub fn new_function_declaration_expr(function_declaration_expr: FunctionDeclarationExpression) -> Syntax
    {
        Syntax::new(SyntaxType::FunctionDeclarationExpression, None, None, None, None, None, None, None, None, None, None, None, Some(function_declaration_expr), None, None, None, None, None)
    }
    pub fn new_if_expr(if_expr: IfExpression) -> Syntax
    {
        Syntax::new(SyntaxType::IfExpression, None, None, None, None, None, None, None, None, None, None, None, None, Some(if_expr), None, None, None, None)
    }
    pub fn new_overwrite_variable_expr(overwrite_variable_expr: OverwriteVariableExpression) -> Syntax
    {
        Syntax::new(SyntaxType::OverwriteVariableExpression, None, None, None, None, None, None, None, None, None, None, None, None, None, Some(overwrite_variable_expr), None, None, None)
    }
    pub fn new_overwrite_array_expr(overwrite_variable_expr: OverwriteArrayExpression) -> Syntax {
        Syntax::new(SyntaxType::OverwriteArrayExpression, None, None, None, None, None, None, None, None, None, None, None, None, None, None, Some(overwrite_variable_expr), None, None)
    }
    pub fn new_array_expr(arr_expr: ArrayExpression) -> Syntax
    {
        Syntax::new(SyntaxType::ArrayExpression, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, Some(arr_expr), None)
    }
    pub fn new_for_expr(for_expression: ForExpression) -> Syntax
    {
        Syntax::new(SyntaxType::ForExpression, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, Some(for_expression))
    }
    pub fn get_type(&self) -> SyntaxType
    {
        self.type_
    }
    pub fn get_integer_literal(&self) -> i32
    {
        if self.integer_literal.is_none()
        {
            panic!("integer_literal is none");
        }
        self.integer_literal.unwrap()
    }
    pub fn get_boolean_literal(&self) -> bool
    {
        if self.boolean_literal.is_none()
        {
            panic!("boolean_literal is none");
        }
        self.boolean_literal.unwrap()
    }
    pub fn get_float_literal(&self) -> f32
    {
        if self.float_literal.is_none()
        {
            panic!("float_literal is none");
        }
        let float_literal = self.float_literal.unwrap();
        return float_literal;
    }
    pub fn get_string_literal(&self) -> String
    {
        if self.string_literal.is_none()
        {
            panic!("string_literal is none");
        }
        self.string_literal.as_ref().unwrap().clone()
    }
    pub fn get_variable_expr(&self) -> VariableExpression
    {
        if self.variable_expr.is_none()
        {
            panic!("variable_expr is none");
        }
        self.variable_expr.as_ref().unwrap().clone()
    }
    pub fn get_array(&self) -> ArrayExpression
    {
        if self.array_expr.is_none()
        {
            panic!("array_expr is none");
        }
        self.array_expr.as_ref().unwrap().clone()
    }
    pub fn get_binary_expr(&self) -> BinaryExpression
    {
        if self.binary_expr.is_none()
        {
            panic!("binary_expr is none");
        }
        self.binary_expr.as_ref().unwrap().clone()
    }
    pub fn get_unary_expr(&self) -> UnaryExpression
    {
        if self.unary_expr.is_none()
        {
            panic!("unary_expr is none");
        }
        self.unary_expr.as_ref().unwrap().clone()
    }
    pub fn get_call_expr(&self) -> CallExpression
    {
        if self.call_expr.is_none()
        {
            panic!("call_expr is none");
        }
        self.call_expr.as_ref().unwrap().clone()
    }
    pub fn get_assignment_expr(&self) -> AssignmentExpression
    {
        if self.assignment_expr.is_none()
        {
            panic!("assignment_expr is none");
        }
        self.assignment_expr.as_ref().unwrap().clone()
    }
    pub fn get_return_expr(&self) -> ReturnExpression
    {
        if self.return_expr.is_none()
        {
            panic!("return_expr is none");
        }
        self.return_expr.as_ref().unwrap().clone()
    }
    pub fn get_arg_variable_expr(&self) -> ArgVariableExpression
    {
        if self.arg_variable_expr.is_none()
        {
            panic!("arg_variable_expr is none");
        }
        self.arg_variable_expr.as_ref().unwrap().clone()
    }
    pub fn get_function_declaration_expr(&self) -> FunctionDeclarationExpression
    {
        if self.function_declaration_expr.is_none()
        {
            panic!("function_declaration_expr is none");
        }
        self.function_declaration_expr.as_ref().unwrap().clone()
    }
    pub fn get_if_expr(&self) -> IfExpression
    {
        if self.if_expr.is_none()
        {
            panic!("if_expr is none");
        }
        self.if_expr.as_ref().unwrap().clone()
    }
    pub fn get_overwrite_variable_expr(&self) -> OverwriteVariableExpression
    {
        if self.overwrite_variable_expr.is_none()
        {
            panic!("overwrite_variable_expr is none");
        }
        self.overwrite_variable_expr.as_ref().unwrap().clone()
    }
    pub fn get_overwrite_array_expr(&self) -> OverwriteArrayExpression
    {
        if self.overwrite_array_expr.is_none()
        {
            panic!("overwrite_array_expr is none");
        }
        self.overwrite_array_expr.as_ref().unwrap().clone()
    }
    pub fn get_for_expr(&self) -> ForExpression
    {
        if self.for_expression.is_none()
        {
            panic!("for_expression is none");
        }
        self.for_expression.as_ref().unwrap().clone()
    }
    pub fn get_while_expr(&self) -> WhileExpression
    {
        if self.while_expression.is_none()
        {
            panic!("while_expression is none");
        }
        self.while_expression.as_ref().unwrap().clone()
    }
}
