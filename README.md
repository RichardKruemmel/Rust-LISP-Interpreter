
# Rust-LISP-Interpreter

This project is a minimal Lisp interpreter written in Rust. It supports basic arithmetic operations, defining variables, and a few list manipulation functions. The interpreter can be run in an interactive mode, which allows you to enter Lisp expressions directly into the command line.

## Features

- Arithmetic operations: +, -
- Variable definition with define
- Printing values with print
- List manipulation with car and cdr

## Running the Interpreter

To run the interpreter, follow these steps:

1. Ensure you have Rust and Cargo installed on your system. If you don't have them, follow the instructions at <https://www.rust-lang.org/tools/install> to install Rust and Cargo.
2. Clone this repository to your local machine.
3. Navigate to the project directory using the command line.
4. Run the interpreter with the command cargo run.

After executing cargo run, you will enter the interactive mode, where you can type Lisp expressions directly. Press CTRL+D or CTRL+C to exit the interpreter.

## Example Usage

Here are some examples of using the Lisp interpreter:

1. Arithmetic operations:

```bash
> (+ 2 3)
5
> (* 4 (- 5 3))
8
```

2. Defining variables

```bash
> (define a 5)
a
> (define b (+ a 3))
b
```

3. Printing values:

```bash
> (print a)
5
> (print b)
8
```

4. List manipulation with car and cdr:

```bash
> (car (list 1 2 3))
1
> (cdr (list 1 2 3))
(2 3)
```

### Limitations

This Lisp interpreter is minimal and lacks many features found in full-fledged Lisp dialects. It is intended primarily for educational purposes and to demonstrate Rust's capabilities in building interpreters.
