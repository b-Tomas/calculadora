use phf::{phf_map, Map};

// Tools for interpreting and calculating expressions

use std::{collections::{HashMap, btree_set::Union}, mem::ManuallyDrop};

use crate::structs::Matrix;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Operations {
    Mul,
    Div,
    Sum,
    Sub,
    Pow,
    Transp,
    Det,
    Inv,
}

pub enum Operand<'a> {
    Operation(Operations),
    Scalar(f32),
    Matrix(&'a Matrix),
}

impl<'a> Operand<'a> {
    /// Returns `true` if the operand is [`Operation`].
    ///
    /// [`Operation`]: Operand::Operation
    #[must_use]
    pub fn is_operation(&self) -> bool {
        matches!(self, Self::Operation(..))
    }

    /// Returns `true` if the operand is [`Scalar`].
    ///
    /// [`Scalar`]: Operand::Scalar
    #[must_use]
    pub fn is_scalar(&self) -> bool {
        matches!(self, Self::Scalar(..))
    }

    /// Returns `true` if the operand is [`Matrix`].
    ///
    /// [`Matrix`]: Operand::Matrix
    #[must_use]
    pub fn is_matrix(&self) -> bool {
        matches!(self, Self::Matrix(..))
    }

    pub fn as_operation(&self) -> Option<&Operations> {
        if let Self::Operation(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_scalar(&self) -> Option<&f32> {
        if let Self::Scalar(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_matrix(&self) -> Option<&&'a Matrix> {
        if let Self::Matrix(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

pub struct ExpTree<'a> {
    op: Operand<'a>,
    left_op: Option<Box<ExpTree<'a>>>,
    right_op: Option<Box<ExpTree<'a>>>,
}

impl<'a> ExpTree<'a> {
    pub fn new(op: Operand<'a>) -> ExpTree<'a> {
        return ExpTree { op, left_op: None, right_op: None };
    }

    pub fn left_op(&self) -> &Option<Box<ExpTree<'a>>> {
        &self.left_op
    }

    pub fn set_left_op(&mut self, left_op: ExpTree<'a>) {
        self.left_op = Some(Box::new(left_op));
    }

    pub fn right_op(&self) -> &Option<Box<ExpTree<'a>>> {
        &self.right_op
    }

    pub fn set_right_op(&mut self, right_op: ExpTree<'a>) {
        self.right_op = Some(Box::new(right_op));
    }

    pub fn op(&self) -> &Operand {
        &self.op
    }

    pub fn set_op(&mut self, op: Operand<'a>) {
        self.op = op;
    }

    pub fn is_leaf(self) -> bool {
        return self.left_op().is_none() && self.right_op().is_none();
    }
}

static OPERATIONS: Map<&str, Operations> = phf_map! {
    "+" => Operations::Sum,
    "-" => Operations::Sub,
    "/" => Operations::Div,
    "*" => Operations::Mul,
    "^" => Operations::Pow,
    "V" => Operations::Inv,
    "T" => Operations::Transp,
    "DET" => Operations::Det,
};

static OP_PRECEDENCE: Map<&str, usize> = phf_map! {
    "+" => 1,
    "-" => 1,
    "/" => 2,
    "*" => 1,
    "^" => 3,
    "V" => 3,
    "T" => 3,
};

// All operations are binary unless specified here
static UNARY_OPS: Map<&str, bool> = phf_map! {
    "V" => true,
    "T" => true,
    "DET" => true,
};

fn is_operator(key: &str) -> bool {
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
        } else if *elem == ")" {
            while let Some(_elem) = stack.pop() {
                if _elem == "(" {
                    break;
                }
                postfix.push(_elem);
            }
        }
        // If number or declared variable
        else if elem.trim().parse::<f32>().is_ok() || in_hashmap_keys(elem, definitions) {
            postfix.push(elem);
        }
        // If number is operator
        if is_operator(elem) {
            // Pop all other operators and push to output (except parenthesis)
            while let Some(_elem) = stack.pop() {
                if _elem == "(" || OP_PRECEDENCE.get(_elem) <= OP_PRECEDENCE.get(elem) {
                    stack.push(_elem);
                    break;
                } else {
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

fn postfix_to_tree<'a>(
    postfix_exp: &'a Vec<&'a str>,
    definitions: &'a HashMap<&'a str, Matrix>,
) -> Option<ExpTree<'a>> {
    let mut stack: Vec<ExpTree> = Vec::new();

    for elem in postfix_exp {
        // If operand
        if let Some(num) = elem.trim().parse::<f32>().ok() {
            stack.push(ExpTree::new(Operand::Scalar(num)))
        } else if let Some(mat) = definitions.get(elem) {
            stack.push(ExpTree::new(Operand::Matrix(mat)))
        }
        // If operator
        else if let Some(operand) = OPERATIONS.get(elem) {
            let mut node = ExpTree::new(Operand::Operation(*operand));
            // If unary add one child to the left, otherwise add both
            if let Some(is_unary) = UNARY_OPS.get(elem) {
                if *is_unary {
                    if let Some(left_child) = stack.pop() {
                        node.set_left_op(left_child);
                    }
                }
            } else {
                if let Some(right_child) = stack.pop() {
                    node.set_right_op(right_child);
                } else {
                    panic!("Run out of stack")
                }
                if let Some(left_child) = stack.pop() {
                    node.set_left_op(left_child);
                } else {
                    panic!("Run out of stack")
                }
            }
            stack.push(node);
        } else {
            return None;
        }
    }
    if stack.len() > 0 {
        if let Some(tree) = stack.pop() {
            return Some(tree);
        } else {
            return None;
        }
    }
    return None;
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::structs::Matrix;

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

        assert!(do_vecs_match(
            &infix_to_postfix(&infix, &definitions),
            &postfix
        ));
    }

    #[test]
    fn test_postfix_to_tree() {
        let postfix: Vec<&str> = "2 A B + C 2 ^ T * +".split(' ').collect();

        let definitions: HashMap<&str, Matrix> = HashMap::from([
            ("A", Matrix::new_empty(1, 1)),
            ("B", Matrix::new_empty(1, 1)),
            ("C", Matrix::new_empty(1, 1)),
        ]);

        let tree = postfix_to_tree(&postfix, &definitions).unwrap();
        assert!(tree.op().is_operation());
        assert_eq!(tree.op().as_operation(), Some(&Operations::Sum));
        assert_eq!(tree.left_op().as_ref().unwrap().op().as_scalar(), Some(&2.0));
        assert_eq!(tree.right_op().as_ref().unwrap().op().as_operation(), Some(&Operations::Mul));
        assert_eq!(tree.right_op().as_ref().unwrap().left_op().as_ref().unwrap().op().as_operation(), Some(&Operations::Sum));
        // Me cansé, pero creo que anda bien
    }
}
