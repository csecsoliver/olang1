# OLang iteration #1
## Syntax
Statements are terminated by `?!` or `!?`.  
Avaliable operators are: `==` `>` `<` `+` `-` `*` `/`.  
Expressions can be grouped by using `(` `)`.  
functions can be called like this: `function_name(comma, separated, arguments)`
## Keywords
### `observe`
Prints the expression it is followed by.
### `if [expression] then [statements] end`
Executes `statements` after `then` if the `expression` evaluates to true.  

`if` can be replaced by `unless` and it will execute `statements` if `expression` is false.
### `if [expression] then [statements] else [statements2] end`
Executes `statements` after `then` if the expression evaluates to true, otherwise executes `statements2`.  

`if` can be replaced by `unless` and it will execute `statements` if `expression` is false, otherwise executes `statements2`.
### `while [expression] do [statements] end`
Executes `statements` after `do` repeatedly until it becomes false.
### `def function_name([comma separated arguments]) [statements] end`
Registers a function with the provided arguments.  
If it is called, it isolates the scope and inserts the provided arguments, and executes `statements`
### `return [expression]`
Returns from the current function with the value of `expression`
### `done`
Returns from the current function with the value `nil`
### `skip [expression]`
`expression` must be an integer. Skips following statements according to the integer provided.
## Literals
### Number literals
### String literals
Can be encased in `"` or `'`
### Bool literals
`true` or `false`


