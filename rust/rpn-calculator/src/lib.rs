#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

use CalculatorInput::*;

struct Stack<T> {
    stack: Vec<T>
}

impl<T> Stack<T> {
    fn new() -> Stack<T> {
        Stack { stack: Vec::new() }
    }
    fn push(&mut self, item: T) {
        self.stack.push(item);
    }
    fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack = Stack::new();
    for input in inputs {
        match *input {
            Value(x) => stack.push(x),
            Add => {
                let op2 = stack.pop();
                let op1 = stack.pop();
                if let (Some(x1), Some(x2)) = (op1, op2) {
                    stack.push(x1 + x2);
                } else {
                    return None;
                }
            },
            Multiply => {
                let op2 = stack.pop();
                let op1 = stack.pop();
                if let (Some(x1), Some(x2)) = (op1, op2) {
                    stack.push(x1 * x2);
                } else {
                    return None;
                }
            },
            Divide => {
                let op2 = stack.pop();
                let op1 = stack.pop();
                if let (Some(x1), Some(x2)) = (op1, op2) {
                    stack.push(x1 / x2);
                } else {
                    return None;
                }
            },
            Subtract => {
                let op2 = stack.pop();
                let op1 = stack.pop();
                if let (Some(x1), Some(x2)) = (op1, op2) {
                    stack.push(x1 - x2);
                } else {
                    return None;
                }
            }
        }
    }
    if stack.stack.len() > 1 {
        return None;
    } else {
        return stack.pop();
    }
}
