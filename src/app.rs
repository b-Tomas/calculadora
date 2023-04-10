use std::{io::{self, stdin, stdout, Write}, collections::HashMap, error::Error, num::ParseFloatError};

use crate::{exp_interpreter::{Definitions, Value, calculate}, structs::Matrix, math};

pub struct App {
    definitions: Definitions,
}

impl App {
    pub fn new() -> App {
        // Load some default variables
        App { definitions: Definitions(HashMap::from([
            (String::from("A"), Value::Matrix(Matrix::new_from(2, 2, &[&[1.0, 2.0], &[3.0, 4.0]]).unwrap())),
            (String::from("B"), Value::Matrix(Matrix::new_from(2, 2, &[&[3.0, 4.5], &[8.0, 2.0]]).unwrap())),
            (String::from("C"), Value::Matrix(Matrix::new_empty(1, 1))),
            (String::from("PI"), Value::Scalar(3.1415)),
        ]))}
    }

    pub fn start(&mut self) -> io::Result<()> {
        // Clear screen
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        loop {
            let mut user_input = String::new();
            prompt(&mut user_input)?;

            let elements: Vec<&str> = user_input.trim().split(' ').into_iter().collect();

            match elements[0] {
                "salir" => break,
                "ayuda" => ayuda(),
                "var" => declare_var(elements.as_slice(), &mut self.definitions),
                "mostrar" => show_var(elements.as_slice(), &mut self.definitions),
                "ecu" => solve_equation(elements.as_slice(), &self.definitions),
                "eqsys" => system_solve(),
                _ => println!("Entrada inválida: {}", user_input),
            }
        }

        Ok(())
    }
}

static FORBIDDEN_IDS: [&str; 8] = [
    "+" ,
    "-" ,
    "*" ,
    "/" ,
    "DET",
    "^" ,
    "INV",
    "T",
];

fn show_var(elements: &[&str], definitions: &mut Definitions) {
    if elements.len() > 1 {
        for i in 1..elements.len() {
            if let Some((k, v)) = definitions.0.get_key_value(elements[i]) {
                print!("{} = ", k);
                print_value(v);
                println!();
            } else {
                println!("La variable `{}` no está definida", elements[i]);
            }
        }
    } else {
        for (k, v) in &definitions.0 {
            print!("{} = ", k);
            print_value(v);
            println!();
        }
    }
}

fn solve_equation(command: &[&str], definitions: &Definitions) {
    if let Ok(result) = calculate(&command[1..].join(" "), definitions) {
        if let Some(scalar) = result.as_scalar() {
            println!("Resultado: {}", *scalar);
            return
        } else if let Some(matrix) = result.as_matrix() {
            println!("Resultado:");
            print_matrix(matrix);
            return
        }
    }
    println!("Ocurrió un error");
}

fn declare_var(command: &[&str], definitions: &mut Definitions) {
    if let Some(id) = command.get(1) {
        if FORBIDDEN_IDS.contains(id) {
            println!("Identificador reservado");
            return;
        }
        if let Some(tipo) = command.get(2) {
            match *tipo {
                "ESCALAR" => {
                    if let Some(valor) = command.get(3) {
                        if let Ok(valor) = valor.parse::<f32>() {
                            let valor = Value::Scalar(valor);
                            if let Some(anterior) = definitions.0.insert(id.to_string(), valor) {
                                println!("Valor anterior:");
                                print_value(&anterior);
                            } 
                            return;
                        } else {
                            println!("El valor debe ser un número");
                        }
                    } else {
                        println!("Valor requerido");
                    }
                }
                "MATRIZ" => {
                    if let (Some(m), Some(n)) = (command.get(3), command.get(4)) {
                        if let (Ok(m), Ok(n)) = (m.parse::<usize>(), n.parse::<usize>()) {
                            let valor;
                            if let Ok(matrix) = read_matrix(m, n) {
                                valor = Value::Matrix(matrix);
                            } else {
                                return;
                            }
                            if let Some(anterior) = definitions.0.insert(id.to_string(), valor) {
                                println!("Valor anterior:");
                                print_value(&anterior);
                            }
                            return;

                        } else {
                            println!("Las dimensiones deben ser números enteros");
                        }
                    }
                }
                _ => {
                    println!("Tipo inválido");
                    return;
                }
            }
        }
    }
    println!("Comando inválido. Probá con `ayuda`");
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
    * `mostrar [identificador]`: Sin argumentos, muestra los detalles de todas las variables declaradas. Filtra por los nombres dados
    * `ecu`: Resolver una ecuación. La sintaxis para ecuaciones se detalla en el archivo README.md
    * `eqsys`: Ingresar un sistema de ecuaciones para determinar la compatibilidad del sistema
    * `salir`: Termina el programa
";
    print!("{}", message);
}

fn prompt(input: &mut String) -> Result<usize, io::Error> {
    print!(">>> ");
    stdout().flush()?;
    stdin().read_line(input)
}

fn print_matrix(mat: &Matrix) {
    for row in 0..mat.m {
        for col in 0..mat.n {
            print!("{} ", mat[row][col]);
        }
        print!("\n");
    }
}

fn print_value(value: &Value) {
    if let Some(matrix) = value.as_matrix() {
        if matrix.m > 1 {
            println!()
        }
        print_matrix(matrix);
    } else if let Some(scalar) = value.as_scalar() {
        println!("{}", *scalar);
    }
}

fn read_matrix(m: usize, n: usize) -> Result<Matrix, Box<dyn Error>> {
    println!("{} filas, {} columnas. Ingrese los datos separados por espacios, y presione Enter despues de cada fila", m, n);
    let mut mat = Matrix::new_empty(m, n);
    for i in 0..m {
        let mut line = String::new();
        print!("Fila {}: ", i);
        stdout().flush()?;
        stdin().read_line(&mut line)?;
        let values: Vec<Result<f32, ParseFloatError>> = line.split_ascii_whitespace().map(|x| x.parse::<f32>()).collect();
        if values.len() != n {
            println!("Numero de columnas incorrecto");
            return Err("Numero de columnas incorrecto")?;
        }
        for j in 0..values.len() {
            if let Ok(value) = values[j] {
                mat.set(i, j, value);
            } else {
                println!("Solo numeros por favor");
                return Err("Bad data")?;
            }
        }

    }
    return Ok(mat);
}

fn system_solve() {
    println!("Cantidad de incógnitas: ");
    let mut incognitas = String::new();
    stdout().flush().unwrap();
    stdin().read_line(&mut incognitas).unwrap();
    println!("Cantidad de ecuaciones: ");
    let mut ecuaciones = String::new();
    stdout().flush().unwrap();
    stdin().read_line(&mut ecuaciones).unwrap();
    if let (Ok(cant_incognitas), Ok(cant_ecuaciones)) = (incognitas.trim().parse::<usize>(), ecuaciones.trim().parse::<usize>()) {
        println!("Ingrese los datos separados por espacios, y presione Enter luego de cada fila. Escriba los datos en formato matriz expandida A|b siendo b el vector independiente");    
        if let Ok(mat) = read_matrix(cant_ecuaciones, cant_incognitas+1) {
            let result = math::solve_system(&mat);
            if result.is_incompatible() {
                println!("El sistema de ecuaciones");
                print_matrix(&mat);
                println!("Es incompatible");
            } else if result.is_compatible_indeterminado() {
                println!("El sistema de ecuaciones");
                print_matrix(&mat);
                println!("Es compatible Indeterminado");
            }else {
                println!("El sistema de ecuaciones");
                print_matrix(&mat);
                println!("Es compatible determinado");
            }
        } else {
        println!("Error en la carga de datos");
        }
    } else {
        println!("No ha ingresado los datos");
    }
}
