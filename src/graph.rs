use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::HashMap;

pub type CollatzGraph = DiGraph<u64, ()>;

/// Build a graph from a Collatz sequence (normal order)
pub fn build_sequence_graph(sequence: Vec<u64>) -> (CollatzGraph, HashMap<u64, NodeIndex>) {
    let mut graph = DiGraph::new();
    let mut node_map = HashMap::new();
    
    // Add all nodes
    for &num in &sequence {
        if !node_map.contains_key(&num) {
            let idx = graph.add_node(num);
            node_map.insert(num, idx);
        }
    }
    
    // Add edges following the sequence
    for i in 0..sequence.len() - 1 {
        let from = node_map[&sequence[i]];
        let to = node_map[&sequence[i + 1]];
        graph.add_edge(from, to, ());
    }
    
    (graph, node_map)
}

/// Build a tree graph showing inverse Collatz (predecessors)
/// Starting from a number, show all numbers that lead to it
pub fn build_inverse_tree(start: u64, depth: usize) -> (CollatzGraph, HashMap<u64, NodeIndex>) {
    let mut graph = DiGraph::new();
    let mut node_map = HashMap::new();
    
    // Add the root node
    let root_idx = graph.add_node(start);
    node_map.insert(start, root_idx);
    
    // Use BFS to build the tree
    let mut queue = vec![(start, root_idx, 0)];
    
    while let Some((num, num_idx, current_depth)) = queue.pop() {
        if current_depth >= depth {
            continue;
        }
        
        let predecessors = crate::collatz::collatz_predecessors(num);
        
        for pred in predecessors {
            // Avoid creating nodes that are too large
            if pred > 1_000_000 {
                continue;
            }
            
            let pred_idx = if let Some(&idx) = node_map.get(&pred) {
                idx
            } else {
                let idx = graph.add_node(pred);
                node_map.insert(pred, idx);
                idx
            };
            
            // Add edge from predecessor to current number
            graph.add_edge(pred_idx, num_idx, ());
            
            if !queue.iter().any(|(n, _, _)| *n == pred) {
                queue.push((pred, pred_idx, current_depth + 1));
            }
        }
    }
    
    (graph, node_map)
}
