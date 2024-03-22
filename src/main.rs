use crate::network::Network;
use crate::node::Node;

mod graphviz;
mod network;
mod node;

fn main() {
    use std::fs;

    let mut network = Network::new();

    let idx_a = network.add_node(Node::new_with_label(1.0, "a".to_string()));

    let idx_b = network.add_node(Node::new_with_label(2.0, "b".to_string()));

    let idx_c = network.add(idx_a, idx_b);
    network.nodes[idx_c].set_label("c".to_string());

    let idx_d = network.add_node(Node::new_with_label(3.0, "d".to_string()));

    let idx_e = network.add(idx_c, idx_d);
    network.nodes[idx_e].set_label("e".to_string());

    let idx_f = network.divide(idx_a, idx_b);
    network.nodes[idx_f].set_label("f".to_string());

    let idx_g = network.tanh(idx_f);
    network.nodes[idx_g].set_label("g".to_string());

    let dot_content = network.to_dot();

    fs::write("network.dot", dot_content).expect("Unable to write file");
}
