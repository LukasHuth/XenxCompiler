# Xenx Compiler
## Datatypes
- int
- string
- flaot
- bool
## Working structure
### declare variable
```
<name>: <datatype> = <value>;
```
### return statement
```
return <variable|literal>;
```
### function declaration
```
func <name>(<argname1>: <datatype1>, <argname2>: <datatype2>): <function_datatype> => {
    <body>
}
```
### redeclaration
```
<name> = <value>;
```
### if else
```
if(<condition>)
{
    <body>
}
else
if(<condition>)
{
    <body>
}
else
{
    <body>
}
```
## AST structure
### Done
#### a: int = 0;
```
{
    tag: AssignmentExpr,
    syntax:
    {
        assignment_expr:
        {
            type_: "a"
            value:
            {
                tag: IntegerLiteral,
                syntax:
                {
                    integer_literal: 0
                }
            }
        }
    }
}
```
#### return 2;
```
{
    tag: ReturnExpr,
    syntax:
    {
        return_expr:
        {
            value:
            {
                tag: IntegerLiteral,
                syntax:
                {
                    integer_literal: 2
                }
            }
        }
    }
}
```

#### return a;
```
{
    tag: ReturnExpr,
    syntax:
    {
        return_expr:
        {
            value:
            {
                tag: VariableExpr,
                syntax:
                {
                    variable_expr: "a"
                }
            }
        }
    }
}
```
### TODO
- suport unary operators
- support if
- support more datatypes

### Done
- int:
    - binary operators
    - unary operators 
    - return values
    - arguments