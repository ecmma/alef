## Alef 
[Alef](https://en.wikipedia.org/wiki/Alef_(programming_language)) is a general 
purpose concurrent programming language, designed as part of the Plan 9 operating system. 

In this repository we'll try to give a modern implementation of Alef using 
the Rust programming language.

### The plan 
* Modernize the [reference](docs/reference)
* Implement AST, lexer and parser (wip in [alef-parser](parser)). 
* Target a backend such as [qbe](https://c9x.me/compile/), LLVM or Cranelift.
