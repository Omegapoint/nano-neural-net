use crate::node::{Node, Operation}; // Adjust the import path according to your project structure
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

pub fn render_dot(dot_content: String, output_name: String) {
    // Use output_name to create the DOT file name
    let dot_file_name = format!("{}.dot", output_name);
    std::fs::write(&dot_file_name, dot_content).expect("Unable to write DOT file");

    // Use output_name to create the PNG file name
    let png_file_name = format!("{}.png", output_name);
    
    // Run the Graphviz `dot` command to generate the PNG from the DOT file
    Command::new("dot")
        .args(["-Tpng", &dot_file_name, "-o", &png_file_name])
        .output()
        .expect("Failed to execute Graphviz dot command");

    println!("Generated {} from {}.", png_file_name, dot_file_name);
}