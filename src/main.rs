mod collatz;
mod graph;
mod gui;

use clap::{Parser, Subcommand};
use eframe::egui;

#[derive(Parser)]
#[command(name = "collatz-viz")]
#[command(about = "Visualize Collatz sequences as a graph", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Visualize the normal Collatz sequence for a number
    Sequence {
        /// The starting number
        #[arg(value_parser = clap::value_parser!(u64).range(1..))]
        number: u64,
    },
    /// Visualize the inverse Collatz tree (numbers that lead to a target)
    Inverse {
        /// The target number
        #[arg(value_parser = clap::value_parser!(u64).range(1..))]
        number: u64,
        
        /// Maximum depth of the tree
        #[arg(short, long, default_value = "5")]
        depth: usize,
    },
}

fn main() -> Result<(), eframe::Error> {
    let cli = Cli::parse();

    let (graph, title) = match cli.command {
        Commands::Sequence { number } => {
            let sequence = collatz::collatz_sequence(number);
            let (graph, _) = graph::build_sequence_graph(sequence);
            (graph, format!("Collatz Sequence starting from {}", number))
        }
        Commands::Inverse { number, depth } => {
            let (graph, _) = graph::build_inverse_tree(number, depth);
            (
                graph,
                format!("Inverse Collatz Tree leading to {} (depth: {})", number, depth),
            )
        }
    };

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 700.0])
            .with_title("Collatz Visualizer"),
        ..Default::default()
    };

    eframe::run_native(
        "Collatz Visualizer",
        options,
        Box::new(|_cc| Ok(Box::new(gui::GraphView::new(graph, title)))),
    )
}
