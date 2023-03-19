# Xenx Compiler
## Comipation
### build the compiler
```
root folder:
cargo build --release
```
### run compiler
```
cargo run --release <filename>
```
### options
```
-o <output file>
-f <os to compile for> (only elf is supported at the moment)
```
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
else if is not working right at the moment
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
## Known Bugs
- else if() is throwing an error
- <variable> == <literal> is not working correctly
## TODO
- suport unary operators
- support if
- support more datatypes

## Done
- int:
    - binary operators
    - unary operators 
    - return values
    - arguments