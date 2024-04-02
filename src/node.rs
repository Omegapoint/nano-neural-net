use std::fmt;
use std::fmt::format;

#[derive(Debug, PartialEq)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Tanh,
}

#[derive(Debug, PartialEq)]
pub struct Node {
    pub value: f64,
    pub gradient: f64,
    pub operation: Option<Operation>,
    pub label: Option<String>,
}

impl Node {
    pub fn new(value: f64) -> Self {
        Node {
            value,
            gradient: 0.0,
            operation: None,
            label: None,
        }
    }

    pub fn new_with_label(value: f64, label: String) -> Self {
        Node {
            value,
            gradient: 0.0,
            operation: None,
            label: Some(label),
        }
    }

    pub fn new_with_operation(value: f64, operation: Operation) -> Self {
        Node {
            value,
            gradient: 0.0,
            operation: Some(operation),
            label: None,
        }
    }

    pub fn set_label(&mut self, label: String) {
        self.label = Some(label);
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let label_display = match &self.label {
            Some(label) => label,
            None => "None",
        };

        let operation_display = match &self.operation {
            Some(operation) => format!("{:?}", operation),
            None => "None".to_string(),
        };

        write!(
            f,
            "Node(label: {}, value: {}, operation: {})",
            label_display, self.value, operation_display
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        let node = Node::new_with_label(1.0, "Test Node".to_string());

        let output = format!("{}", node);

        let expected = "Node(label: Test Node, value: 1, operation: None)";
        assert_eq!(output, expected);
    }

    #[test]
    fn new() {
        let node = Node::new(1.0);

        assert_eq!(node.value, 1.0);
        assert_eq!(node.operation, None);
        assert_eq!(node.label, None);
    }

    #[test]
    fn new_with_label() {
        let node = Node::new_with_label(1.0, "Test Node".to_string());

        assert_eq!(node.value, 1.0);
        assert_eq!(node.operation, None);
        assert_eq!(node.label, Some("Test Node".to_string()));
    }

    #[test]
    fn new_with_operation() {
        let node = Node::new_with_operation(1.0, Operation::Add);

        assert_eq!(node.value, 1.0);
        assert_eq!(node.operation, Some(Operation::Add));
        assert_eq!(node.label, None);
    }

    #[test]
    fn set_label() {
        let mut node = Node::new(1.0);

        node.set_label("Test Node".to_string());

        assert_eq!(node.label, Some("Test Node".to_string()));
    }
}
