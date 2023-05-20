[![builds.sr.ht status](https://builds.sr.ht/~ecmma/alef-parser.svg)](https://builds.sr.ht/~ecmma/alef-parser?)
# alef-parser 
An Alef parser written in Rust; this crate is a work in progress. 
The objective is to have a working lexer, parser, typechecker and importer for Alef source files. 
Technically the reference documentation is complete, as Alef has been described by Phil Winterbottom at 
Bell Labs in the '90s; however, this is to be considered a somewhat modernized implementation, 
therefore some changes are expected. 

## Todos: 
* [] Syntax analysis
    * [] Implement the parser 
        * [] Figure out how to parse ambiguous nodes (x * y: is it a declaration or a multiplication?) 
        or surrend and use a symbol table in the parser
* [] Type system
    * [] (?) Figure out whether it's ok to modernize the type system or not: could we still call it *Alef*?
        * [] Remove distinction between strings and runestrings: every string is UTF-8
        * [] Implement pointers as low-fat safe pointers
    * [] (?) Write a formal documentation for the type system, or at least the safe part of it, and try to 
    prove soundness 
    * [] Implement the typechecker
* [] Module system
    * [] (?) Figure out whether it's ok to drop the preprocessor system used originally in favour of a more
    modern (module based) approach
    * [] Remove support for preprocessor and implement the module system
