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
- make code gen where code is returned by calling functions
- count in syntactic analyser how many times variables of an specific type are initialized and how much the max at the same time are
- function call argument types are not checked as of now

### Done
- a: int = 1;
- b: int = a;
- c: int = a + b;
- d: bool = true;
- e: bool = d;
- g: bool = e == f;
- h: bool = 1 >= 2;
- i: int = 1 + 2 * 3
- j: int = ( 1 + 2 ) * 3;
- f: bool = !e;
- k: int = -1;
- l: int = -(5+6);