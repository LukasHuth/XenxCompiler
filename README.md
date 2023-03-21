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
-c (dont delete asm file and add comments for debug purposses)
```
## Datatypes
- int
- string
- flaot
- bool
- char
## Working structure
### import files
```
import <filename>;
```
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
### using standard functions
```
std::print(<expression>);
```
## Known Bugs
- no at the moment
## TODO
- support more datatypes
- add support for multi dimensional arrays

## Done
- int:
    - binary operators
    - unary operators 
    - return values
    - arguments
- booleans
- string
    - decleration
    - printing
- standart functions (not many, only print at the moment)