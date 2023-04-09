use std::{io::{self, stdin, stdout, Write}, collections::HashMap};

use crate::{exp_interpreter::{Definitions, Value, calculate}, structs::Matrix};

pub struct App {
    definitions: Definitions,
}

impl App {
    pub fn new() -> App {
        // App { definitions: Definitions(HashMap::new()) }

        App { definitions: Definitions(HashMap::from([
            (String::from("A"), Value::Matrix(Matrix::new_from(2, 2, &[&[1.0, 2.0], &[3.0, 4.0]]).unwrap())),
            (String::from("B"), Value::Matrix(Matrix::new_from(2, 2, &[&[3.0, 4.5], &[8.0, 2.0]]).unwrap())),
            (String::from("C"), Value::Matrix(Matrix::new_empty(1, 1))),
            (String::from("D"), Value::Scalar(2.0)),
        ]))}
    }

    pub fn start(&mut self) -> io::Result<()> {
        loop {
            let mut user_input = String::new();
            prompt(&mut user_input)?;

            let elements: Vec<&str> = user_input.trim().split(' ').into_iter().collect();

            match elements[0] {
                "salir" => break,
                "ayuda" => ayuda(),
                "var" => declare_var(elements, &mut self.definitions),
                "mostrar" => declare_var(elements, &mut self.definitions),
                "ecu" => solve_equation(elements, &self.definitions),
                _ => println!("Entrada inválida: {}", user_input),
            }
        }

        Ok(())
    }
}

fn solve_equation(command: Vec<&str>, definitions: &Definitions) {
    if let Ok(result) = calculate(&command[1..].join(" "), definitions) {
        if let Some(scalar) = result.as_scalar() {
            println!("Resultado: {}", *scalar);
            return
        } else if let Some(matrix) = result.as_matrix() {
            println!("Resultado:");
            for row in 0..matrix.m {
                for col in 0..matrix.n {
                    print!("{} ", matrix[row][col]);
                }
                print!("\n");
            }
            return
        }
    }
    println!("Ocurrió un error");
}

fn declare_var(command: Vec<&str>, definitions: &mut Definitions) {
    // if let (Some(id), Some(value)) = command[1..] {
        
    // }
    // self.definitions.0.insert(k, v)
    println!("todo!")
}

fn ayuda() {
    let message = 
"Calculadora: TODO: descripcion, mensajes, etc

Uso:
    * `ayuda`: Muestra este mensaje
    * `var <NOMBRE> <TIPO> [dimensiones]`: Declara una variable
        * TIPO: `ESCALAR` | `MATRIZ`
        * dimensiones: Para tipo `MATRIZ` solamente. Formato `n m`
        * Ejemplos:
            - `var PI ESCALAR 3.14`
            - `var MAT MATRIZ 2 2` El programa pedirá ingresar los datos separados por espacios y saltos de linea
    * `mostrar`
    * `ecu`
    * `salir`: Termina el programa
";
    print!("{}", message);
}

fn prompt(input: &mut String) -> Result<usize, io::Error> {
    print!(">>> ");
    stdout().flush()?;
    stdin().read_line(input)
}