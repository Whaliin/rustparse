# rustparse

A fast, zero-dependency, recursive descent mathematical expression parser and REPL built from scratch in Rust.

## Features

- **PEMDAS & Operators**: Full operator precedence and left-associativity for `+`, `-`, `*`, `/`, `%`, and right-associative exponents `^`.
- **Unary Operators**: Unary negation (`-5`, `-(2+3)`, `- -5`) and unary plus (`+5`).
- **Implicit Multiplication**: Supports handwriting-style multiplication like `2(3)`, `(1+1)(2+2)`, `2pi`, `2sin(90)`, and `x(5)`.
- **Postfix Factorial**: Factorial operator `!` supporting chaining (`4! = 24`, `3!! = 720`).
- **Mathematical Constants**: `pi`, `e`, `tau`, and `goldr` (golden ratio).
- **Functions**:
  - **Trigonometry**: `sin`, `cos`, `tan`, and inverse trig `asin`, `acos`, `atan` with interactive degrees/radians mode toggles.
  - **Single-Arg**: `sqrt`, `abs`, `log` (base-10), `floor`, `ceil`, `round`.
  - **Multi-Arg & Variadic**: `pow(base, exp)`, `clamp(val, min, max)`, and variadic `min(...)` / `max(...)` supporting $\ge 2$ arguments.
- **Variables & Session Memory**: Set variables with `x = 10`, reuse them across expressions (`x * 5`), and inspect them with `:vars`.
- **Calculator History (`ans`)**: The result of the previous computation is automatically preserved in `ans`.
- **Interactive REPL**: Built-in interactive console with commands (`:deg`, `:rad`, `:vars`, `:clear`, `:help`, `:exit`).
- **Performance**: Zero-copy state-machine lexer with `Rc<str>` identifier interning and recursive descent parsing.

## Getting Started

### Prerequisites

Ensure you have [Rust and Cargo](https://www.rust-lang.org/tools/install) installed.

### Running the Interactive REPL

```powershell
cargo run
```

Example session:
```text
> 2 + 3 * 4
= 14
> (1 + 2)(3 + 4)
= 21
> 4!
= 24
> clamp(6!, 32, 64)
= 64
> :deg
> sin(90)
= 1
> x = 10
= 10
> ans * 2
= 20
> x + ans
= 30
> :vars
 ans = 30
 x = 10
```

### Running the Unit Test Suite

```powershell
cargo test
```

## Project Architecture

```
src/
├── main.rs          # Interactive REPL session & command handler
├── types.rs         # Enums for Token, OpType, FuncType, and ConstType
├── tokenizer.rs     # Zero-copy state-machine lexer
└── solver.rs        # Recursive descent parser, variable environment & unit test suite
```
