use eframe::egui;
use petgraph::graph::NodeIndex;
use std::collections::HashMap;

use crate::graph::CollatzGraph;

pub struct GraphView {
    graph: CollatzGraph,
    node_positions: HashMap<NodeIndex, egui::Pos2>,
    title: String,
}

impl GraphView {
    pub fn new(graph: CollatzGraph, title: String) -> Self {
        let node_positions = Self::calculate_layout(&graph);
        Self {
            graph,
            node_positions,
            title,
        }
    }

    fn calculate_layout(graph: &CollatzGraph) -> HashMap<NodeIndex, egui::Pos2> {
        let mut positions = HashMap::new();
        let node_count = graph.node_count();
        
        if node_count == 0 {
            return positions;
        }

        // Simple layout: arrange nodes in a tree-like structure
        // Group nodes by their level (BFS from roots)
        let mut levels: Vec<Vec<NodeIndex>> = Vec::new();
        let mut visited = std::collections::HashSet::new();
        
        // Find root nodes (nodes with no incoming edges)
        let roots: Vec<_> = graph
            .node_indices()
            .filter(|&n| graph.neighbors_directed(n, petgraph::Direction::Incoming).count() == 0)
            .collect();
        
        if roots.is_empty() {
            // If no roots (cycle), just take the first node
            levels.push(vec![graph.node_indices().next().unwrap()]);
        } else {
            levels.push(roots);
        }
        
        // BFS to assign levels
        let mut current_level = 0;
        while current_level < levels.len() {
            let mut next_level = Vec::new();
            
            for &node in &levels[current_level] {
                visited.insert(node);
                
                for neighbor in graph.neighbors_directed(node, petgraph::Direction::Outgoing) {
                    if !visited.contains(&neighbor) && !next_level.contains(&neighbor) {
                        next_level.push(neighbor);
                    }
                }
            }
            
            if !next_level.is_empty() {
                levels.push(next_level);
            }
            current_level += 1;
        }
        
        // Position nodes
        let horizontal_spacing = 120.0;
        let vertical_spacing = 80.0;
        let start_x = 400.0;
        let start_y = 100.0;
        
        for (level_idx, level_nodes) in levels.iter().enumerate() {
            let y = start_y + level_idx as f32 * vertical_spacing;
            let total_width = (level_nodes.len() - 1) as f32 * horizontal_spacing;
            let start_x_level = start_x - total_width / 2.0;
            
            for (node_idx, &node) in level_nodes.iter().enumerate() {
                let x = start_x_level + node_idx as f32 * horizontal_spacing;
                positions.insert(node, egui::Pos2::new(x, y));
            }
        }
        
        positions
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.heading(&self.title);
        
        ui.separator();
        
        // Draw the graph
        let (response, painter) = ui.allocate_painter(
            egui::Vec2::new(ui.available_width(), 600.0),
            egui::Sense::hover(),
        );
        
        let to_screen = egui::emath::RectTransform::from_to(
            egui::Rect::from_min_size(egui::Pos2::ZERO, response.rect.size()),
            response.rect,
        );
        
        // Draw edges
        for edge in self.graph.edge_indices() {
            if let Some((source, target)) = self.graph.edge_endpoints(edge) {
                if let (Some(&start_pos), Some(&end_pos)) = 
                    (self.node_positions.get(&source), self.node_positions.get(&target)) {
                    
                    let start = to_screen.transform_pos(start_pos);
                    let end = to_screen.transform_pos(end_pos);
                    
                    painter.line_segment(
                        [start, end],
                        egui::Stroke::new(2.0, egui::Color32::GRAY),
                    );
                    
                    // Draw arrow
                    let dir = (end - start).normalized();
                    let arrow_size = 8.0;
                    let arrow_pos = end - dir * 15.0;
                    let perp = egui::Vec2::new(-dir.y, dir.x);
                    
                    painter.line_segment(
                        [arrow_pos, arrow_pos - dir * arrow_size + perp * arrow_size * 0.5],
                        egui::Stroke::new(2.0, egui::Color32::GRAY),
                    );
                    painter.line_segment(
                        [arrow_pos, arrow_pos - dir * arrow_size - perp * arrow_size * 0.5],
                        egui::Stroke::new(2.0, egui::Color32::GRAY),
                    );
                }
            }
        }
        
        // Draw nodes
        for node_idx in self.graph.node_indices() {
            if let Some(&pos) = self.node_positions.get(&node_idx) {
                let screen_pos = to_screen.transform_pos(pos);
                let value = self.graph[node_idx];
                
                // Draw circle
                painter.circle_filled(
                    screen_pos,
                    15.0,
                    egui::Color32::from_rgb(70, 130, 180),
                );
                
                // Draw border
                painter.circle_stroke(
                    screen_pos,
                    15.0,
                    egui::Stroke::new(2.0, egui::Color32::WHITE),
                );
                
                // Draw label
                let label = format!("{}", value);
                painter.text(
                    screen_pos,
                    egui::Align2::CENTER_CENTER,
                    label,
                    egui::FontId::proportional(12.0),
                    egui::Color32::WHITE,
                );
            }
        }
    }
}

impl eframe::App for GraphView {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.ui(ui);
        });
    }
}
