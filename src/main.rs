mod graphviz;
mod node;


use crate::graphviz::{to_dot, render_dot};
use crate::node::Node;



fn generate_simple_network() {
    let a = Node::new_with_label(3.0, "a".to_string());
    let b = Node::new_with_label(4.0, "b".to_string());
    let mut c = &a + &b;

    c.set_label("c".to_string());

    let d = Node::new_with_label(5.0, "d".to_string());
    let mut e = &c * &d;
    e.set_label("e".to_string());

    let f = Node::new_with_label(2.0, "f".to_string());

    let mut L = &e / &f;
    L.set_label("L".to_string());

    let dot_content = to_dot(&L);

    render_dot(dot_content, "simple".to_string());
}

fn generate_neuron() {
    let w1 = Node::new_with_label(1.0, "w1".to_string());
    let w2 = Node::new_with_label(2.0, "w2".to_string());

    let x1 = Node::new_with_label(3.0, "x1".to_string());
    let x2 = Node::new_with_label(4.0, "x2".to_string());

    let b = Node::new_with_label(1.0, "b".to_string());

    let mut x1w1 = &w1 * &x1;
    x1w1.set_label("x1*w1".to_string());

    let mut x2w2 = &w2 * &x2;
    x2w2.set_label("x2*w2".to_string());
    let mut x1w1x2w2 = &x1w1 + &x2w2;
    x1w1x2w2.set_label("x1*w1 + x2*w2".to_string());

    let mut n = &x1w1x2w2 + &b;
    n.set_label("n".to_string());

    let mut o = n.tanh();
    o.set_label("o".to_string());


    let dot_content = to_dot(&o);

    render_dot(dot_content, "neuron".to_string());
}

fn main() {
    generate_simple_network();
    generate_neuron();
}
