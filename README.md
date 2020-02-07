# BrainfuckInterpreter

## What does it do
This project contains both an interpreter, and a transpiler. The interpreter is just a basic brainfuck interpreter, that takes a string containing brainfuck code, and steps through it.
The transpiler however, is a bit more interesting. It takes a very naive approach by looping through all brainfuck characters, and replaces them with a line of rust code that does the same thing.
It outputs a rust file that you can compile into a fully working exe with the `rustc` command.

## Why
Why not? It's a fun project!
