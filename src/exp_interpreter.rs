use phf::{phf_map, Map};

// Tools for interpreting and calculating expressions
use std::{collections::{HashMap}, error::Error};

use crate::{structs::Matrix, math::{mul_scalar, mul, sum, sub, pow, transp_squared_matrix, det, inv}};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Operators {
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
    Operation(Operators),
    Scalar(f32),
    Matrix(&'a Matrix),
}

pub enum Value {
    Scalar(f32),
    Matrix(Matrix),
}

impl Value {
    pub fn as_scalar(&self) -> Option<&f32> {
        if let Self::Scalar(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_matrix(&self) -> Option<&Matrix> {
        if let Self::Matrix(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the value is [`Scalar`].
    ///
    /// [`Scalar`]: Value::Scalar
    #[must_use]
    pub fn is_scalar(&self) -> bool {
        matches!(self, Self::Scalar(..))
    }

    /// Returns `true` if the value is [`Matrix`].
    ///
    /// [`Matrix`]: Value::Matrix
    #[must_use]
    pub fn is_matrix(&self) -> bool {
        matches!(self, Self::Matrix(..))
    }
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

    pub fn as_operation(&self) -> Option<&Operators> {
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

    pub fn is_leaf(&self) -> bool {
        return self.left_op().is_none() && self.right_op().is_none();
    }

    // Will return either a number or matrix as a result
    fn solve(&self) -> Result<Value, Box<dyn Error>> {
        if self.is_leaf() {
            if self.op().is_operation() {
                return Err("Leaf node has operator as only data")?;
            }
            if let Some(value) = self.op().as_scalar() {
                return Ok(Value::Scalar(*value));
            } else if let Some(value) = self.op().as_matrix() {
                return Ok(Value::Matrix(value.clone().to_owned()));
            }
        }
        
        if let Some(operator) = self.op().as_operation() {
            match *operator {
                Operators::Mul => {
                    if let (Some(left), Some(right)) = (self.left_op(), self.right_op()) {
                        if let (Ok(left), Ok(right)) = (left.solve(), right.solve()) {
                            // Left and right are scalars, left is matrix and the other scalar, the other way around, or both are matrices
                            if let (Some(left), Some(right)) = (left.as_scalar(), right.as_scalar()) {
                                return Ok(Value::Scalar(left * right));
                            } else if let (Some(left), Some(right)) = (left.as_scalar(), right.as_matrix()) {
                                return Ok(Value::Matrix(mul_scalar(right, *left)));
                            } else if let (Some(left), Some(right)) = (left.as_matrix(), right.as_scalar()) {
                                return Ok(Value::Matrix(mul_scalar(left, *right)));
                            } else if let (Some(left), Some(right)) = (left.as_matrix(), right.as_matrix()) {
                                if let Ok(result) = mul(left, right) {
                                    return Ok(Value::Matrix(result));
                                }
                            }
                        }
                    }
                },
                Operators::Div => todo!(), // TODO: Depends on matrix inverse
                Operators::Sum => {
                    if let (Some(left), Some(right)) = (self.left_op(), self.right_op()) {
                        if let (Ok(left), Ok(right)) = (left.solve(), right.solve()) {
                            // Left and right are scalars, or both are matrices
                            if let (Some(left), Some(right)) = (left.as_scalar(), right.as_scalar()) {
                                return Ok(Value::Scalar(left + right));
                            } else if let (Some(left), Some(right)) = (left.as_matrix(), right.as_matrix()) {
                                if let Ok(result) = sum(left, right) {
                                    return Ok(Value::Matrix(result));
                                }
                            }
                        }
                    }
                },
                Operators::Sub => {
                    if let (Some(left), Some(right)) = (self.left_op(), self.right_op()) {
                        if let (Ok(left), Ok(right)) = (left.solve(), right.solve()) {
                            // Left and right are scalars, or both are matrices
                            if let (Some(left), Some(right)) = (left.as_scalar(), right.as_scalar()) {
                                return Ok(Value::Scalar(left - right));
                            } else if let (Some(left), Some(right)) = (left.as_matrix(), right.as_matrix()) {
                                if let Ok(result) = sub(left, right) {
                                    return Ok(Value::Matrix(result));
                                }
                            }
                        }
                    }
                },
                Operators::Pow => {
                    if let (Some(left), Some(right)) = (self.left_op(), self.right_op()) {
                        if let (Ok(left), Ok(right)) = (left.solve(), right.solve()) {
                            // Left can be both, right always scalar
                            if let (Some(left), Some(right)) = (left.as_scalar(), right.as_scalar()) {
                                return Ok(Value::Scalar(left.powf(*right)));
                            } else if let (Some(left), Some(right)) = (left.as_matrix(), right.as_scalar()) {
                                if let Ok(result) = pow(left, *right as i8) {
                                    return Ok(Value::Matrix(result));
                                }
                            }
                        }
                    }
                }
                Operators::Transp => {
                    if self.right_op().is_some() {
                        return Err("Operador unario tiene dos operandos")?;
                    } else if let Some(left) = self.left_op() {
                        if let Ok(left) = left.solve() {
                            if left.is_scalar() {
                                return Err("No se puede aplicar la operacion Transponer a un escalar")?;
                            } else if let Ok(result) = transp_squared_matrix(left.as_matrix().unwrap()) {
                                return Ok(Value::Matrix(result));
                            }
                        }
                    }
                }
                Operators::Det => {
                    if self.right_op().is_some() {
                        return Err("Operador unario tiene dos operandos")?;
                    } else if let Some(left) = self.left_op() {
                        if let Ok(left) = left.solve() {
                            if left.is_scalar() {
                                return Err("No se puede aplicar la operacion Determinante a un escalar")?;
                            } else if let Ok(result) = det(left.as_matrix().unwrap()) {
                                return Ok(Value::Scalar(result));
                            }
                        }
                    }
                }
                Operators::Inv => {
                    if self.right_op().is_some() {
                        return Err("Operador unario tiene dos operandos")?;
                    } else if let Some(left) = self.left_op() {
                        if let Ok(left) = left.solve() {
                            if left.is_scalar() {
                                return Err("No se puede aplicar la operacion Inversa a un escalar")?;
                            } else if let Ok(result) = inv(left.as_matrix().unwrap()) {
                                return Ok(Value::Matrix(result));
                            }
                        }
                    }
                }
            }
        } else {
            return Err("Non leaf node is not an operator")?;
        }
        return Err("Something happened")?;
    }
}

static OPERATIONS: Map<&str, Operators> = phf_map! {
    "+"   => Operators::Sum,
    "-"   => Operators::Sub,
    "/"   => Operators::Div,
    "*"   => Operators::Mul,
    "^"   => Operators::Pow,
    "INV" => Operators::Inv,
    "T"   => Operators::Transp,
    "DET" => Operators::Det,
};

static OP_PRECEDENCE: Map<&str, usize> = phf_map! {
    "+"   => 1,
    "-"   => 1,
    "*"   => 1,
    "/"   => 2,
    "DET" => 2,
    "^"   => 3,
    "INV" => 3,
    "T"   => 3,
};

// All operations are binary unless specified here
static UNARY_OPS: Map<&str, bool> = phf_map! {
    "INV" => true,
    "T"   => true,
    "DET" => true,
};

fn is_operator(key: &str) -> bool {
    OP_PRECEDENCE.contains_key(key)
}

// Struct that holds the currently declared variables 
pub struct Definitions(pub HashMap<String, Value>);

fn in_variable_defintions(key: &str, map: &Definitions) -> bool {
    map.0.keys().find(|&x| *x == *key).is_some()
}

fn infix_to_postfix<'a>(
    infix_exp: &'a Vec<&'a str>,
    definitions: &'a Definitions,
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
        else if elem.trim().parse::<f32>().is_ok() || in_variable_defintions(elem, definitions) {
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
    definitions: &'a Definitions
) -> Option<ExpTree<'a>> {
    let mut stack: Vec<ExpTree> = Vec::new();

    for elem in postfix_exp {
        // If operand
        if let Some(num) = elem.trim().parse::<f32>().ok() {
            stack.push(ExpTree::new(Operand::Scalar(num)))
        } else if let Some(val) = definitions.0.get(*elem) {
            if let Some(num) = val.as_scalar() {
                stack.push(ExpTree::new(Operand::Scalar(*num)))
            } else if let Some(mat) = val.as_matrix() {
                stack.push(ExpTree::new(Operand::Matrix(mat)))
            }
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

pub fn calculate(infix_exp: &str, definitions: &Definitions) -> Result<Value, Box<(dyn std::error::Error)>> {
    if let Some(tree) = postfix_to_tree(&infix_to_postfix(&Vec::from_iter(infix_exp.split(' ').into_iter()), definitions), definitions) {
        return tree.solve()
    } else {
        return Err("Parsing error")?;
    }
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
        let infix: Vec<&str> = "2 + ( ( A + B ) * ( C ^ D ) T )".split(' ').collect();
        let postfix: Vec<&str> = "2 A B + C D ^ T * +".split(' ').collect();

        let definitions = Definitions(HashMap::from([
            (String::from("A"), Value::Matrix(Matrix::new_empty(1, 1))),
            (String::from("B"), Value::Matrix(Matrix::new_empty(1, 1))),
            (String::from("C"), Value::Matrix(Matrix::new_empty(1, 1))),
            (String::from("D"), Value::Scalar(2.0)),
        ]));

        assert!(do_vecs_match(
            &infix_to_postfix(&infix, &definitions),
            &postfix
        ));
    }

    #[test]
    fn test_postfix_to_tree() {
        let postfix: Vec<&str> = "2 A B + C D ^ T * +".split(' ').collect();

        let definitions = Definitions(HashMap::from([
            (String::from("A"), Value::Matrix(Matrix::new_empty(1, 1))),
            (String::from("B"), Value::Matrix(Matrix::new_empty(1, 1))),
            (String::from("C"), Value::Matrix(Matrix::new_empty(1, 1))),
            (String::from("D"), Value::Scalar(2.0)),
        ]));

        let tree = postfix_to_tree(&postfix, &definitions).unwrap();
        assert_eq!(tree.op().as_operation(), Some(&Operators::Sum));
        assert_eq!(tree.left_op().as_ref().unwrap().op().as_scalar(), Some(&2.0));
        assert_eq!(tree.right_op().as_ref().unwrap().op().as_operation(), Some(&Operators::Mul));
        assert_eq!(tree.right_op().as_ref().unwrap().left_op().as_ref().unwrap().op().as_operation(), Some(&Operators::Sum));
        // Me cansé, pero creo que anda bien
    }

    #[test]
    fn test_solve() {
        // TODO: test every operation
        let expected = Matrix::new_from(2, 2, &[&[12.5, 6.5], &[23.25, 12.0]]).unwrap();

        let definitions = Definitions(HashMap::from([
            (String::from("A"), Value::Matrix(Matrix::new_from(2, 2, &[&[1.0, 2.0], &[3.0, 4.0]]).unwrap())),
            (String::from("B"), Value::Matrix(Matrix::new_from(2, 2, &[&[3.0, 4.0], &[5.0, 6.0]]).unwrap())),
            (String::from("C"), Value::Matrix(Matrix::new_from(2, 2, &[&[1.25, 0.5], &[0.5, 0.5]]).unwrap())),
            (String::from("D"), Value::Scalar(2.0)),
        ]));
        
        // Power
        assert!(*calculate("4 ^ 3", &definitions).unwrap().as_scalar().unwrap() == 64.0);
        
        // A complex expression
        let infix_exp = "( A + B ) * ( C ^ D ) T";
        let result = calculate(infix_exp, &definitions).unwrap();
        let matrix = result.as_matrix().unwrap();
        assert!(matrix.equals(&expected));

        // With determinant
        let infix_exp = "( C ^ D ) T DET";
        let expected = 9.0/64.0;
        let result = *calculate(infix_exp, &definitions).unwrap().as_scalar().unwrap();
        assert!(result == expected);

        // Even more complex
        let infix_exp = "( ( A + B ) * ( C ^ D ) T ) DET";
        let expected = -9.0/8.0;
        let result = *calculate(infix_exp, &definitions).unwrap().as_scalar().unwrap();
        assert!(result == expected);

        // With inverse
        let infix_exp = "( A ^ D ) INV";
        let expected = Matrix::new_from(2, 2, &[&[5.5, -2.5], &[-3.75, 1.75]]).unwrap();
        let result = calculate(infix_exp, &definitions).unwrap();
        let result = result.as_matrix().unwrap();
        assert!(result.equals(&expected));
    }
}
