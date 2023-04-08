use phf::{Map, phf_map};

// Tools for interpreting and calculating expressions

use std::collections::HashMap;

use crate::structs::Matrix;

enum Operations {
    Muliply,
    Sum,
    Transpose,
    Determinant,
    Inverse,
}

union Operand<'a> {
    operation: std::mem::ManuallyDrop<Operations>,
    scalar: f32,
    matrix: &'a Matrix,
}

struct ExpTree<'a> {
    op: Operand<'a>,
    // NOTE: Otra opcion acá que he visto que se usa mucho es Boxing, para almacenar en el heap.
    // No me pareció necesario por el reducido tamaño que va a tener el arbol
    left_op: Option<&'a ExpTree<'a>>,
    right_op: Option<&'a ExpTree<'a>>,
}


static OP_PRECEDENCE: Map<&str, usize> = phf_map! {
    "+" => 1,
    "-" => 1,
    "/" => 2,
    "*" => 1,
    "^" => 3, // Potencia
    "V" => 3, // Inversa
    "T" => 3, // Transpuesta
};

fn is_operator(key: &str) -> bool{
    OP_PRECEDENCE.contains_key(key)
}

fn in_hashmap_keys<V>(key: &str, map: &HashMap<&str, V>) -> bool {
    map.keys().find(|&&x| *x == *key).is_some()
}

fn infix_to_postfix<'a>(
    infix_exp: &'a Vec<&'a str>,
    definitions: &'a HashMap<&'a str, Matrix>,
) -> Vec<&'a str> {

    let mut stack: Vec<&str> = Vec::new();

    let mut postfix: Vec<&str> = Vec::new();
    for elem in infix_exp {
        if *elem == "(" {
            stack.push(elem);
        }
        else if *elem == ")" {
            while let Some(_elem) = stack.pop() {
                if _elem == "(" { break }
                postfix.push(_elem);
            }
        }
        // If number or declared variable
        else if elem.trim().parse::<f64>().is_ok() || in_hashmap_keys(elem, definitions) {
            postfix.push(elem);
        }
        // If number is operator
        if is_operator(elem) {
            // Pop all other operators and push to output (except parenthesis)
            while let Some(_elem) = stack.pop() {
                if _elem == "(" || OP_PRECEDENCE.get(_elem) <= OP_PRECEDENCE.get(elem){ 
                    stack.push(_elem);
                    break 
                } else  {
                    postfix.push(_elem);
                }
            }
            stack.push(elem);
        }
    }
    // Finish sending stack to output
    while let Some(_elem) = stack.pop() {
        postfix.push(_elem);
    }
    return postfix;
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::structs::Matrix;
    use super::*;

    fn do_vecs_match<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
        let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
        matching == a.len() && matching == b.len()
    }

    #[test]
    fn test_infix_to_postfix() {
        let infix: Vec<&str> = "2 + ( ( A + B ) * ( C ^ 2 ) T )".split(' ').collect();
        let postfix: Vec<&str> = "2 A B + C 2 ^ T * +".split(' ').collect();

        let definitions: HashMap<&str, Matrix> = HashMap::from([
            ("A", Matrix::new_empty(1, 1)),
            ("B", Matrix::new_empty(1, 1)),
            ("C", Matrix::new_empty(1, 1)),
        ]);

        assert!(do_vecs_match(&infix_to_postfix(&infix, &definitions), &postfix));
    }
}
