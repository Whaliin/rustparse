# TODO: Math Parser Roadmap & Future Features

## 1. Math Constants
- [x] Add support for common constants in `push_state`:
  - [x] `pi` -> `std::f64::consts::PI`
  - [x] `e`  -> `std::f64::consts::E`
  - [x] `goldr` -> `std::f64::consts::GOLDEN_RATIO`
  - [x] `tau` -> `std::f64::consts::TAU`
- [x] Support implicit multiplication with constants (e.g., `2pi` -> `2 * pi`).

## 2. Single-Argument Functions
- [x] Add new variants to `FuncType`:
  - [x] `Sin`, `Cos`, `Tan`
  - [x] `Sqrt` -> `num.sqrt()`
  - [x] `Abs`  -> `num.abs()`
  - [x] `Log`  -> `num.log10()`
  - [x] `Floor`, `Ceil`, `Round`
  - [x] Inverse trig: `Asin`, `Acos`, `Atan`
- [x] Adjust inverse trig (`asin`, `acos`, `atan`) output to match angle mode (convert radians to degrees).
- [x] Support implicit multiplication before function calls (e.g., `2sin(90)` -> `2 * sin(90)`).

## 3. Angle Mode Toggle (Degrees vs. Radians) & REPL Commands
- [x] Intercept commands starting with `:` in `main.rs` before passing to solver:
  - [x] `:deg` - Switch trig functions to degrees mode.
  - [x] `:rad` - Switch trig functions to radians mode.
  - [x] `:vars` - Print all currently defined variables and their values.
  - [x] `:clear` - Clear all defined variables.
  - [x] `:help` - List available commands and functions.

## 4. Multi-Argument & Variadic Functions
- [x] Add comma token (`b',' => Token::Comma`) to the lexer.
- [x] Dedicated `parse_args(&mut self)` parser for comma-separated arguments.
- [x] Multi-argument function implementations:
  - [x] `pow(base, exp)`
  - [x] `clamp(val, min, max)`
  - [x] Variadic `min(...)` supporting $\ge 2$ arguments via `.reduce()`.
  - [x] Variadic `max(...)` supporting $\ge 2$ arguments via `.reduce()`.
- [x] Slice pattern matching (`[x]`, `[a, b]`, `[a, b, c]`) for arity validation.

## 5. Operators & Precedence
- [x] Core PEMDAS (`+`, `-`, `*`, `/`) with correct precedence and left-associativity.
- [x] Exponent operator (`^`) with right-associativity (`parse_exponential`).
- [x] Modulo operator (`%`) in `parse_term`.
- [x] Unary negation (`-5`, `-(2+3)`, `- -5`).
- [x] Postfix factorial (`!`) supporting chaining (e.g. `4!`, `3!!`).
- [x] Implicit multiplication before parentheses (`2(3)` and `(1+1)(2+2)`).

## 6. Architecture & Code Structure
- [x] Hand-written zero-copy state-machine lexer (`tokenizer.rs`).
- [x] Recursive descent parser with descriptive rule hierarchy (`solver.rs`).
- [x] Safe error handling using `Result<f64, String>` and the `?` operator.
- [x] Modularized project structure:
  - [x] `src/types.rs`
  - [x] `src/tokenizer.rs`
  - [x] `src/solver.rs`
  - [x] `src/main.rs` (REPL)

## 7. Variables & Environment (Memory)
- [x] Add a `HashMap<String, f64>` symbol table to `Solver`.
- [x] Support assignment syntax in REPL (e.g., `x = 5`).
- [x] In `push_state`, if a word is not a recognized function or constant, treat it as a variable lookup (`Token::Ident(String)`).
- [x] Provide error when referencing an undefined variable.

## 8. Higher Precision (Optional Future Exploration)
- [ ] Explore swapping `f64` with the `rust_decimal` crate for exact 128-bit decimal arithmetic without floating-point precision drift.
