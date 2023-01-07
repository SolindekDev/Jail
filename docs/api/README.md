# Mao API

## How to start with our API?##
If you want to use our API in C or C++ this is an litt
le example how you can use it:
```c
#include <mao_api.h>

mao_return_t* our_print_func(mao_args_t* args) {
    printf("Our printf: %s", args->first_argument->raw);
    return mao_empty_argument();
}

int main(int argc, char** argv) {
    mao_interpreter_t* mao_interpreter = mao_init_interpreter(
       // Script
       "proc main()\nour_print("Lol") \nend",
       // Script filename
       "mao_script.mao");

    mao_create_func(
       mao_interpreter, 
       "our_print", 
       1,
       use_func);
    
    // It will process the output to stdout
    mao_start_interpreter(mao_interpreter);
}
```
As you see mao available to create your own functions 
that can be wrapped to C function. Let's say something
about this code firstly in functin main we initialize
interpreter process we are giving script value and a 
script filename, next we create an function in mao by
function `mao_create_func` firstly we are sending
`mao_interpreter_t` structure next function name, how 
many arguments we expect and the c function that wrapps
this function, and finally we start the interpreter 
process, that's bascially how you can use Mao API

## How does mao even works?
Mao is an interpreter and it works something like this:
- `src/mao_main.c` parse arguments after it read file that have been send in argv and after it start interpreter process
- `src/api/mao_interpreter.c` start interpreter process first run lexer after then parser and finally executor
- `src/api/mao_lexer.c` tokenize value of this file and return token array
- `src/api/mao_parser.c` parse tokens and generate abstract syntax tree that will be executed
- `src/api/mao_executor.c` go by the ast and execute it

## Functions
// TODO: write here some functions

## Structures
// TODO: write here some structures