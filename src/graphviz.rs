use crate::network::Network;
use crate::node::{Node, Operation};
use std::process::Command;

impl Operation {
    pub fn as_str(&self) -> &'static str {
        match *self {
            Operation::Add => "+",
            Operation::Subtract => "-",
            Operation::Multiply => "*",
            Operation::Divide => "/",
            Operation::Tanh => "tanh",
        }
    }
}

impl Network {
    pub fn to_dot(&self) -> String {
        let mut dot = String::from("digraph G {\n    rankdir=LR;\n");

        dot.push_str("    node [shape=box];\n");

        // Iterate over nodes to add them to the dot graph
        for (idx, node) in self.nodes.iter().enumerate() {
            let label = match &node.label {
                Some(l) => l.clone(),
                None => "".to_string(),
            };
            let operation = match &node.operation {
                Some(op) => format!("\nop: {}", op.as_str()),
                None => "".to_string(),
            };

            let value_str = if node.value.fract() == 0.0 {
                format!("{}", node.value.trunc())
            } else {
                format!("{:.3}", node.value)
            };

            dot.push_str(&format!(
                "    node{} [label=\"{}: {}{}\"]; \n",
                idx, label, value_str, operation
            ));
        }

        // Iterate over edges to add them to the dot graph
        for edge in &self.edges {
            dot.push_str(&format!("    node{} -> node{};\n", edge.from, edge.to));
        }

        dot.push_str("}\n");
        dot
    }
}
