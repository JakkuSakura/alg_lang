# Alg Lang
Alg_lang, the same as algorithm language, aims to provide somewhat basic features in almost all modern programming languages and to enable programmers to use their codes on multi-platform at code level.

## Why Ala Lang
When you have to write the same algorithms, such as markdown parser, for different platforms, like php and javascript, again and again, you will be bored about it. Maintaining two versions of your code may be a difficulty, especially when they have to perform identically.

## Brief description

Ala lang applies Rust-like grammar and similar features, since Rust Language is a beautiful and powerful programming tool. However, restrictions in Rust are too many and its features may be not enough for higher-level abstraction, compared to Java, for instance, These are for Rust users:

- No borrow, no lifetime, no move semantics
- Objects in dump can be passed by pointers or references.
- Objects in stack cannot be passed to mutable arguments, neither be passed to another thread. Immutable arguments are acceptable.
- Some new builtin functions like Allocators: new() and delete()
- Advanced templates
- const expressions and const functions, which may be used for template arguments.
- Aliases for useful types: int uint short ushort long ulong float double pointer etc.
 

## Features

1. pass primitive-type arguments by copy
1. pass structure by reference
1. static and strong type
1. integers, floats, strings, arrays supported.
1. simple and flexible standard library for all supported platforms.
1. functions

## Advanced and optional features
1. optional runtime including GC if required
1. structure and class supported
1. dict supported
1. derivation supported
1. interface supported
1. native call for specific platforms.

## Dismissed features
1. Memory modification
1. 

## Target platforms
Python, C++, C, Java, PHP, Javascript, C#, Rust, etc.
