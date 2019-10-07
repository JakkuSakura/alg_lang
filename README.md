# Alg Lang
Alg_lang, the same as algorithm language, aims to provide somewhat basic features in almost all modern programming languages and to enable programmers to use their codes on multi-platform at code level.

## Why Ala Lang
When you have to write the same algorithms, such as markdown parser, for different platforms, like php and javascript, again and again, you will be bored about it. Maintaining two versions of your code may be a difficulty, especially when they have to perform identically.

## Brief description

Ala lang applies Rust-like grammar and similar features, since Rust Language is a beautiful and powerful programming tool. However, restrictions in Rust are too many and its features may be not enough for higher-level abstraction, compared to Java, for instance, These are for Rust users:

- No borrow, no lifetime, no move semantics
- Objects in dump can be passed by pointers or references.
- Some new builtin functions like Allocators: new() and delete()
- Advanced templates
- Const expressions and const functions, which may be used for template arguments.
- Aliases for useful types: int uint short ushort long ulong float double pointer etc.
 

## Types
### Primitives
- signed integers: `i8` `i16` `i32` `i64`
- unsigned integers: `u8` `u16` `u32` `u64`
- floats: `f32` `f64`
- aliases for signed integers: `short` `int` `long` `isize`
- aliases for unsigned integers: `byte` `char` `ushort` `uint` `ulong` `uchar` `usize`
- aliases for floats: `float` `double`
- others: pointer(`*`) reference(`&` or `ref`) function pointer(`fn(arguments) -> return_type`)

### Advanced Types
- arrays: Rust-like array(`[type; len]`, with length in the very front of the array) C-like array(`type[len]`, without length stored in array or somewhere else)
- length-ignored arrays: `[type]` or `[type; _]` or `[type; n]`(match array for any lengths and copy length into immutable variable `n`), `type[]`, `type[_]` `type[n]`
- tuple: `(type,)` `(type1, type2 ...)`
- strings: `str`(string (maybe)without zero byte at very end, but with size at the beginning, like `[char]`) `cstr`(string with zero at very end, like `char[]`)
- template types: `type<arguments>`

### Std types
These are some Java-like standard library types. 
- String
- Map: TreeMap, HashMap
- Set: TreeSet, HashSet
- List: ArrayList, LinkedList, DoubleLikedList

## Declaration
### Variable
#### Primitives and advances types.
Variable declaration is almost the same as Rust. Use `mut` to mark mutability.
```
let i = 5;
let l: u64 = 8888888888ul;
let l2 = 8888888888i64;
let b: byte = 0x6C;
let o = 0o744;
let mut b2: byte = 0b10011100;
```
#### Std types and user-defined types, knows as class

You may get a class by call `type()`. Assume Foo is a class.
```
let foo1 = Foo(); // this way, Foo is allocated in stack, not in dump.
let foo2 = new(Foo()); # now Foo is allocated in dump, and the type of foo2 is "ref Foo" or "&Foo"

delete(foo2); # now foo2 is deleted and the reference itself is reset to null. you may have to delete everything on dump carefully at appropriate times.

let foo3 = foo1; # foo3 is the copy of foo1. Needs `copy` trait
                 # otherwise, it's prohibited.
                 # Note that in Rust foo1 will be moved.
                 
let foo4 = &foo1; # the same as "ref foo1", foo4 is "ref Foo"
                  # You cannot delete it manually, which will cause memory error.

let foo5 = &foo1 as *Foo; # clearly now foo5 is *Foo

let foo6 = new(Foo()) as *Foo;
delete(foo6.clone()); # to preserve the value of foo6

let s1 = "hello"; # s1 is ref str
let s2 = c"hello"; # s2 is ref cstr.

let mut a = [i8; 9];
```

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

