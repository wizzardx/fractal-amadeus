use chrono::Utc;
use fractal_amadeus::MemoryGraph;
use fractal_amadeus::SymbolicNode;
use std::env;
use std::io::{self, Write};

// Get version information at compile time
const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

fn main() {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if version information was requested
    if args.len() > 1 && (args[1] == "--version" || args[1] == "-v") {
        println!("Fractal Amadeus v{}", VERSION);
        println!("{}", DESCRIPTION);
        println!("Authors: {}", AUTHORS);
        return;
    }

    // Regular application startup
    println!(
        "Welcome to Fractal Amadeus v{}: A Proof-of-Alignment Node",
        VERSION
    );
    println!("=====================================================");
    println!("Type 'help' for available commands or 'exit' to quit.");

    // Initialize the memory graph
    let mut memory_graph = MemoryGraph::new(None);

    // Main interaction loop
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        // Process command
        let parts: Vec<&str> = input.splitn(2, ' ').collect();
        let command = parts[0].to_lowercase();

        match command.as_str() {
            "exit" | "quit" => {
                println!("Exiting Fractal Amadeus. Symbolic coherence maintained.");
                break;
            }
            "help" => {
                display_help();
            }
            "add" => {
                if parts.len() < 2 {
                    println!("Usage: add <concept_key>|<definition>|<framework>|<confidence>");
                    continue;
                }

                // Parse the concept information
                let concept_parts: Vec<&str> = parts[1].splitn(4, '|').collect();
                if concept_parts.len() < 3 {
                    println!("Please provide concept in format: concept_key|definition|framework|confidence(optional)");
                    continue;
                }

                let key = concept_parts[0].trim().to_string();
                let content = concept_parts[1].trim().to_string();
                let framework = concept_parts[2].trim().to_string();

                // Parse confidence, default to 0.5 if not provided
                let confidence = if concept_parts.len() >= 4 {
                    concept_parts[3].trim().parse::<f32>().unwrap_or(0.5)
                } else {
                    0.5
                };

                // Create the symbolic node
                let node = SymbolicNode {
                    content,
                    confidence,
                    framework,
                    last_updated: Utc::now().to_rfc3339(),
                };

                // Add to memory graph
                match memory_graph.add_concept(key.clone(), node) {
                    Ok(_) => println!("Concept '{}' added to memory graph.", key),
                    Err(e) => println!("Error adding concept: {}", e),
                }
            }
            "get" => {
                if parts.len() < 2 {
                    println!("Usage: get <concept_key>");
                    continue;
                }

                let key = parts[1].trim();
                match memory_graph.get_concept(key) {
                    Some(concept) => {
                        println!("Concept: {}", key);
                        println!("  Definition: {}", concept.content);
                        println!("  Framework: {}", concept.framework);
                        println!("  Confidence: {:.2}", concept.confidence);
                        println!("  Last Updated: {}", concept.last_updated);
                    }
                    None => println!("Concept '{}' not found in memory graph.", key),
                }
            }
            "version" => {
                println!("Fractal Amadeus v{}", VERSION);
                println!("{}", DESCRIPTION);
                println!("Authors: {}", AUTHORS);
            }
            _ => {
                println!("Unknown command. Type 'help' for available commands.");
            }
        }
    }
}

fn display_help() {
    println!("Available commands:");
    println!("  help                - Display this help message");
    println!("  add <key>|<def>|<framework>|<conf>  - Add a concept to memory graph");
    println!("    Example: add consciousness_IIT|integrated information above threshold Φ|Integrated Information Theory|0.8");
    println!("  get <key>           - Retrieve a concept from memory graph");
    println!("  version             - Display version information");
    println!("  exit, quit          - Exit the application");
    println!();
    println!("You can also run the application with --version or -v flag to display version information.");
}
