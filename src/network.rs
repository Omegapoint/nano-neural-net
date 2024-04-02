use crate::node::{Node, Operation};

pub struct Edge {
    pub from: usize,
    pub to: usize,
}

pub struct Network {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

impl Network {
    pub fn new() -> Self {
        Network {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) -> usize {
        self.nodes.push(node);
        self.nodes.len() - 1
    }

    fn add_edge(&mut self, source_node_idx: usize, target_node_idx: usize) {
        self.edges.push(Edge {
            from: source_node_idx,
            to: target_node_idx,
        });
    }

    pub fn add(&mut self, idx_a: usize, idx_b: usize) -> usize {
        let node_a = &self.nodes[idx_a];
        let node_b = &self.nodes[idx_b];

        let new_node = Node::new_with_operation(node_a.value + node_b.value, Operation::Add);
        let new_node_idx = self.add_node(new_node);
        self.add_edge(idx_a, new_node_idx);
        self.add_edge(idx_b, new_node_idx);

        new_node_idx
    }

    pub fn multiply(&mut self, idx_a: usize, idx_b: usize) -> usize {
        let node_a = &self.nodes[idx_a];
        let node_b = &self.nodes[idx_b];

        let new_node = Node::new_with_operation(node_a.value * node_b.value, Operation::Multiply);
        let new_node_idx = self.add_node(new_node);
        self.add_edge(idx_a, new_node_idx);
        self.add_edge(idx_b, new_node_idx);

        new_node_idx
    }

    pub fn subtract(&mut self, idx_a: usize, idx_b: usize) -> usize {
        let node_a = &self.nodes[idx_a];
        let node_b = &self.nodes[idx_b];

        let new_node = Node::new_with_operation(node_a.value - node_b.value, Operation::Subtract);
        let new_node_idx = self.add_node(new_node);
        self.add_edge(idx_a, new_node_idx);
        self.add_edge(idx_b, new_node_idx);

        new_node_idx
    }

    pub fn divide(&mut self, idx_a: usize, idx_b: usize) -> usize {
        let node_a = &self.nodes[idx_a];
        let node_b = &self.nodes[idx_b];

        let new_node = Node::new_with_operation(node_a.value / node_b.value, Operation::Divide);
        let new_node_idx = self.add_node(new_node);
        self.add_edge(idx_a, new_node_idx);
        self.add_edge(idx_b, new_node_idx);

        new_node_idx
    }

    pub fn tanh(&mut self, idx_a: usize) -> usize {
        let node_a = &self.nodes[idx_a];

        let new_node = Node::new_with_operation(node_a.value.tanh(), Operation::Tanh);
        let new_node_idx = self.add_node(new_node);
        self.add_edge(idx_a, new_node_idx);

        new_node_idx
    }

    pub fn topological_sort(&self) -> Vec<usize> {
        let mut visited = vec![false; self.nodes.len()];
        let mut order = Vec::new();

        for idx in 0..self.nodes.len() {
            if !visited[idx] {
                self.dfs(idx, &mut visited, &mut order);
            }
        }

        order
    }

    fn dfs(&self, node_idx: usize, visited: &mut Vec<bool>, order: &mut Vec<usize>) {
        visited[node_idx] = true;
        for edge in &self.edges {
            if edge.from == node_idx && !visited[edge.to] {
                self.dfs(edge.to, visited, order);
            }
        }
        order.push(node_idx);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arithmetic() {
        let mut network = Network::new();

        let idx_a = network.add_node(Node::new(1.0));
        let idx_b = network.add_node(Node::new(2.0));
        let idx_c = network.add(idx_a, idx_b);
        let idx_d = network.multiply(idx_a, idx_b);
        let idx_e = network.subtract(idx_a, idx_b);
        let idx_f = network.divide(idx_a, idx_b);
        let idx_g = network.tanh(idx_a);

        assert_eq!(network.nodes[idx_c].value, 3.0);
        assert_eq!(network.nodes[idx_d].value, 2.0);
        assert_eq!(network.nodes[idx_e].value, -1.0);
        assert_eq!(network.nodes[idx_f].value, 0.5);
        assert_eq!(network.nodes[idx_g].value, 0.7615941559557649);
    }

    #[test]
    fn test_topological_ordering() {
        let mut network = Network::new();

        let idx_a = network.add_node(Node::new(1.0)); // 0
        let idx_b = network.add_node(Node::new(2.0)); // 1
        let idx_c = network.add(idx_a, idx_b); // 2
        let idx_d = network.add_node(Node::new(3.0)); // 3
        let idx_e = network.multiply(idx_c, idx_d); // 4
        let idx_f = network.tanh(idx_e); // 5

        let topo_order = network.topological_sort();
        assert_eq!(topo_order, vec![idx_f, idx_e, idx_c, idx_a, idx_b, idx_d]); // Using DFS, so will explore "left" paths first
    }
}
