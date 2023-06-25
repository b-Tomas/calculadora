# Matrix Calculator

Click [acá](https://github.com/b-Tomas/calculadora/blob/main/README.es-AR.md) para leer en español.

![](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)

> Authors: Tomás Badenes and Santiago Fernandez

![Demo Video](doc-assets/demo.gif)

## Description

This program was the result of a challenge from a mathematics professor. The assignment was:

> 1. Write a program that, given two matrices, analyzes if they can be added, and if so, calculates the sum.
> 2. Write a program that, given two matrices, analyzes if the product is defined, and if so, calculates it.
> 3. Write a program that, given a matrix, analyzes if it has an inverse, and if so, calculates it.
> 4. Write a program that, given a system of linear equations, analyzes the compatibility of the system.

We decided to go a step further and create a complete calculator, written in Rust, that integrates the solutions to the four proposed problems and more. Its features are detailed [below](#features), as well as the [installation and usage instructions](#usage).

## Features

The calculator works through commands. The general syntax is: `<COMMAND> [parameters]`

To obtain a list of possible commands, use the `help` command.

### Scalar and Matrix Operations

This calculator can solve combined calculations using operations between matrices and scalars. The supported operations are:

- `*`: Multiplication
- `/`: Division
- `+`: Addition
- `-`: Subtraction (or for matrices, addition with -1 times the subtrahend)
- `^`: Power
- `T`: Transpose of a matrix (syntax: `<matrix> T`)
- `DET`: Determinant of a matrix (syntax: `<matrix> DET`)
- `INV`: Inverse of a matrix (syntax: `<matrix> INV`)

### Solving Combined Calculations

Using the `ecu` command (from __equation__ in spanish) and providing a string of operands and operators separated by spaces, the calculator will solve the calculation if it has a solution, or display an error message if a malformed expression or undefined operation is given.

For example, to calculate `((((Matrix A + B) * (Matrix A squared)) <- Transpose) <- Determinant) + PI` write

```
>>> ecu ( ( A + B ) * ( A ^ 2 ) T ) DET + PI
Resultado: -186.8585
```

### Declaration and Storage of Variables

The `mostrar` command (__show__ in spanish) can be used to inspect the stored variables. The calculator includes some default variables:


```
>>> mostrar
PI = 3.1415

B =
3 4.5
8 2

A =
1 2
3 4

C = 0
```

The `mostrar` command can also be asked to filter by variable name:


```
>>> mostrar A PI E
A =
1 2
3 4

PI = 3.1415

La variable `E` no está definida
```
(The message says: _The variable \`E\` is not defined_)

These identifiers can then be used in `ecu` expressions.

### Systems of Equations

The `ecsis` command allows entering a system of equations in the form of the augmented matrix in the result vector (A|b) to determine if it has a solution or not.

## Setup

### Installation from Source Code

1. Clone the repository:

```
git clone https://github.com/b-Tomas/calculadora.git
```

Or use the green button to download a `.zip` archive.

2. Download and install the Rust version for your operating system from the [official page](https://www.rust-lang.org/tools/install).

3. Compile the project:

```
cargo build
```

4. Run unit tests:

```
$ cargo test
running 21 tests
test exp_interpreter::tests::test_postfix_to_tree ... ok
test exp_interpreter::tests::test_infix_to_postfix ... ok
test math::tests::compatible_determinado ... ok
test math::tests::identity_matrix ... ok
test math::tests::determinant ... ok
test math::tests::incompatible_equation ... ok
test math::tests::mat_pow ... ok
test math::tests::matrix_multiplication ... ok
test math::tests::inverse_test ... ok
test math::tests::matrix_sub ... ok
test math::tests::matrix_sum ... ok
test math::tests::multiplication_by_scalar ... ok
test math::tests::transposed ... ok
test exp_interpreter::tests::test_solve ... ok
test math::tests::test_adj ... ok
test math::tests::undetermined ... ok
test math::tests::very_incompatible ... ok
test structs::tests::create_empty_matrix ... ok
test structs::tests::create_matrix_from_data ... ok
test structs::tests::equals ... ok
test structs::tests::is_squared ... ok

test result: ok. 21 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```


5. Start the program:


```
cargo run
```


### Binary Installation

Visit the [releases](https://github.com/b-Tomas/calculadora/releases) page to download the compiled program for your platform, although installing from source code is recommended to have the latest version of the program.

## About the Solutions

The logic responsible for solving the mathematical problems is located in the `src/math.rs` file. Since it was a mathematics assignment, we decided to stay away from external libraries that could make the calculations easier and fry our brains programming the solutions from scratch.

Furthermore, this was our first encounter with the Rust language. While the code is not perfect or the cleanest (and we know rustaceans will cringe while reading it), we believe it is quite acceptable considering we had only a couple of days to write it.
