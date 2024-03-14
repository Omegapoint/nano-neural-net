mod graphviz;
mod node;
use std::process::Command;

use crate::graphviz::to_dot;
use crate::node::Node;

fn generate_and_render_dot() {
    let a = Node::new_with_label(3.0, "a".to_string());
    let b = Node::new_with_label(4.0, "b".to_string());
    let mut c = &a + &b;

    c.set_label("c".to_string());

    let d = Node::new_with_label(5.0, "d".to_string());
    let mut e = &c * &d;
    e.set_label("e".to_string());

    let dot_content = to_dot(&e);

    std::fs::write("output.dot", dot_content).expect("Unable to write DOT file");

    Command::new("dot")
        .args(["-Tpng", "output.dot", "-o", "output.png"])
        .output()
        .expect("Failed to execute Graphviz dot command");

    println!("Generated output.png from output.dot.");
}

fn main() {
    generate_and_render_dot();
}
