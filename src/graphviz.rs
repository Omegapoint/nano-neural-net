use crate::node::{Node, Operation}; // Adjust the import path according to your project structure

impl Operation {
    pub fn as_str(&self) -> &'static str {
        match *self {
            Operation::Add => "+",
            Operation::Subtract => "-",
            Operation::Multiply => "*",
            Operation::Divide => "/",
        }
    }
}

pub fn generate_dot<'a>(node: &Node<'a>, dot: &mut String, node_counter: &mut usize) -> String {
    let node_id = format!("node{}", node_counter);
    *node_counter += 1;

    let label = match node.label.as_ref() {
        Some(label) => label.as_str(),
        None => "None",
    };

    let node_label = format!("{}: {}", label, node.value);

    dot.push_str(&format!(
        "    {} [shape=box, label=\"{}\"];\n",
        node_id, node_label
    ));

    if let Some(op) = &node.operation {
        let op_node_id = format!("op{}", node_counter);
        *node_counter += 1;

        dot.push_str(&format!(
            "    {} [shape=circle, label=\"{}\"];\n",
            op_node_id,
            op.as_str()
        ));
        dot.push_str(&format!("    {} -> {};\n", op_node_id, node_id));

        for &operand in &node.operands {
            let operand_node_id = generate_dot(operand, dot, node_counter);
            dot.push_str(&format!("    {} -> {};\n", operand_node_id, op_node_id));
        }
    }

    node_id
}

pub fn to_dot<'a>(root: &Node<'a>) -> String {
    let mut dot = String::from("digraph G {\n");
    dot.push_str("    rankdir=LR;\n");
    let mut node_counter: usize = 0;
    generate_dot(root, &mut dot, &mut node_counter);
    dot.push_str("}");
    dot
}
