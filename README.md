# Tiny C Compiler in Rust

A toy compiler written in Rust that translates a small subset of C code directly into ARM64 assembly.

This project was built to explore the end-to-end compilation pipeline, moving from raw text parsing down to hardware-specific machine instructions.

## Architecture

The compiler is divided into three main phases:

1. **Lexer:** Reads raw C source code strings and tokenizes them, automatically handling whitespace and identifying keywords, numbers, and operators.
2. **Parser:** Uses a recursive descent approach to convert the flat stream of tokens into a hierarchical Abstract Syntax Tree (AST), ensuring grammatical correctness and order of operations.
3. **Code Generator:** Traverses the AST bottom-up (post-order) to generate raw ARM64 assembly instructions suitable for Apple Silicon (M-Series) processors, handling basic register allocation.

## Example

**Input (C Code):**

```c
int x = 5 + 3;
```

**Output (ARM64 Assembly):**

```asm
mov w0, #5
mov w1, #3
add w0, w0, w1
```

## Running the Compiler

Ensure you have Rust and Cargo installed, then run:

```bash
cargo run
```
