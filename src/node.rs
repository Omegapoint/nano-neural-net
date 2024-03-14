use std::fmt;
use std::fmt::format;
use std::ops;

#[derive(Debug, PartialEq)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, PartialEq)]
pub struct Node<'a> {
    pub value: f64,
    pub operands: Vec<&'a Node<'a>>,
    pub operation: Option<Operation>,
    pub label: Option<String>,
}

impl<'a> Node<'a> {
    pub fn new(value: f64) -> Self {
        Node {
            value,
            operands: Vec::new(),
            operation: None,
            label: None,
        }
    }

    pub fn new_with_label(value: f64, label: String) -> Self {
        Node {
            value,
            operands: Vec::new(),
            operation: None,
            label: Some(label),
        }
    }

    fn with_references(value: f64, operands: Vec<&'a Node<'a>>, operation: Operation) -> Self {
        Node {
            value,
            operands: operands,
            operation: Some(operation),
            label: None,
        }
    }

    pub fn set_label(&mut self, label: String) {
        self.label = Some(label);
    }
}

impl<'a> fmt::Display for Node<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let label_display = match &self.label {
            Some(label) => label,
            None => "None",
        };

        let operation_display = match &self.operation {
            Some(operation) => format!("{:?}", operation),
            None => "None".to_string(),
        };

        let operands_display: String = if self.operands.is_empty() {
            "None".to_string()
        } else {
            self.operands.len().to_string()
        };

        write!(
            f,
            "Node(label: {}, value: {}, operation: {}, operands: {})",
            label_display, self.value, operation_display, operands_display
        )
    }
}

impl<'a> ops::Add for &'a Node<'a> {
    type Output = Node<'a>;

    fn add(self, other: Self) -> Node<'a> {
        Node::with_references(self.value + other.value, vec![self, other], Operation::Add)
    }
}

impl<'a> ops::Sub for &'a Node<'a> {
    type Output = Node<'a>;

    fn sub(self, other: Self) -> Node<'a> {
        Node::with_references(
            self.value - other.value,
            vec![self, other],
            Operation::Subtract,
        )
    }
}

impl<'a> ops::Mul for &'a Node<'a> {
    type Output = Node<'a>;

    fn mul(self, other: Self) -> Node<'a> {
        Node::with_references(
            self.value * other.value,
            vec![self, other],
            Operation::Multiply,
        )
    }
}

impl<'a> ops::Div for &'a Node<'a> {
    type Output = Node<'a>;

    fn div(self, other: Self) -> Node<'a> {
        Node::with_references(
            self.value / other.value,
            vec![self, other],
            Operation::Divide,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_node() {
        let node = Node {
            value: 1.0,
            operands: Vec::new(),
            operation: Some(Operation::Add),
            label: Some("Test Node".to_string()),
        };

        let output = format!("{}", node);

        let expected = "Node(label: Test Node, value: 1, operation: Add, operands: None)";
        assert_eq!(output, expected);
    }

    #[test]
    fn display_node_with_operands() {
        let operand1 = Node::new(2.0);
        let operand2 = Node::new(3.0);

        let operands = vec![&operand1, &operand2];
        let mut parent_node = Node::with_references(5.0, operands, Operation::Add);
        parent_node.set_label("Parent Node".to_string());
        let output = format!("{}", parent_node);

        let expected = "Node(label: Parent Node, value: 5, operation: Add, operands: 2)";

        assert_eq!(output, expected);
    }

    #[test]
    fn algebra() {
        let a = Node::new(4.0);
        let b = Node::new(2.0);
        let c = &a + &b; // 4 + 2 = 6

        assert_eq!(c.value, a.value + b.value);
        assert!(std::ptr::eq(c.operands[0], &a) && std::ptr::eq(c.operands[1], &b));
        assert_eq!(c.operation, Some(Operation::Add));

        let d = Node::new(3.0);
        let e = &c - &d; // 6 - 3 = 3

        assert_eq!(e.value, c.value - d.value);
        assert!(std::ptr::eq(e.operands[0], &c) && std::ptr::eq(e.operands[1], &d));
        assert_eq!(e.operation, Some(Operation::Subtract));

        let f = &e * &d; // 3 * 3 = 9

        assert_eq!(f.value, e.value * d.value);
        assert!(std::ptr::eq(f.operands[0], &e) && std::ptr::eq(f.operands[1], &d));
        assert_eq!(f.operation, Some(Operation::Multiply));

        let g = &f / &b; // 9 / 2 = 4.5

        assert_eq!(g.value, f.value / b.value);
        assert!(std::ptr::eq(g.operands[0], &f) && std::ptr::eq(g.operands[1], &b));
        assert_eq!(g.operation, Some(Operation::Divide));
    }
}
