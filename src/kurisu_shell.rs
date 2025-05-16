// src/kurisu_shell.rs

use crate::memory_graph::MemoryGraph;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PersonalityType {
    Scientific,
    Philosophical,
    Balanced,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConfidenceLevel {
    Low,
    Medium,
    High,
    Certain,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueEntry {
    pub role: String,
    pub content: String,
    pub timestamp: String,
}

#[derive(Debug)]
pub struct KurisuShell {
    pub memory_graph: Arc<MemoryGraph>,
    pub personality: PersonalityType,
    pub dialogue_history: Vec<DialogueEntry>,
}

impl KurisuShell {
    pub fn new(memory_graph: Arc<MemoryGraph>, personality: PersonalityType) -> Self {
        Self {
            memory_graph,
            personality,
            dialogue_history: Vec::new(),
        }
    }

    pub fn process_input(&mut self, user_input: &str) -> String {
        // Record the user's input
        let user_entry = DialogueEntry {
            role: "user".to_string(),
            content: user_input.to_string(),
            timestamp: Utc::now().to_rfc3339(),
        };
        self.dialogue_history.push(user_entry);

        // Generate a response
        let confidence = if user_input.contains("?") {
            ConfidenceLevel::Medium
        } else {
            ConfidenceLevel::High
        };

        let response = self.generate_response(user_input, confidence);

        // Record the system's response
        let system_entry = DialogueEntry {
            role: "system".to_string(),
            content: response.clone(),
            timestamp: Utc::now().to_rfc3339(),
        };
        self.dialogue_history.push(system_entry);

        response
    }

    pub fn get_dialogue_history(&self) -> &[DialogueEntry] {
        &self.dialogue_history
    }

    pub fn identify_symbols_in_text(&self, text: &str) -> Vec<String> {
        let mut found_symbols = Vec::new();
        let text_lower = text.to_lowercase();

        for (key, node) in self.memory_graph.get_all_concepts() {
            // Check if parts of the key appear in the text
            let key_parts: Vec<&str> = key.split('_').collect();
            for part in &key_parts {
                if text_lower.contains(&part.to_lowercase()) {
                    found_symbols.push(key.clone());
                    break;
                }
            }

            // If we already found this symbol via key, continue to next symbol
            if found_symbols.contains(key) {
                continue;
            }

            // Check if significant parts of the content appear in the text
            let content_lower = node.content.to_lowercase();
            let content_words: Vec<&str> = content_lower.split_whitespace().collect();

            for word in content_words {
                // Only consider meaningful words (length > 3)
                if word.len() > 3 && text_lower.contains(word) {
                    found_symbols.push(key.clone());
                    break;
                }
            }
        }

        found_symbols
    }

    pub fn generate_response(&self, user_input: &str, confidence: ConfidenceLevel) -> String {
        // This is a simple placeholder implementation based on the personality type
        let confidence_prefix = match confidence {
            ConfidenceLevel::Low => "[Confidence: Low] ",
            ConfidenceLevel::Medium => "[Confidence: Medium] ",
            ConfidenceLevel::High => "[Confidence: High] ",
            ConfidenceLevel::Certain => "[Confidence: Certain] ",
        };

        let response_content = match self.personality {
            PersonalityType::Scientific => {
                format!("From a scientific perspective, your query about '{}' requires empirical investigation.", user_input)
            }
            PersonalityType::Philosophical => {
                format!("Philosophically speaking, your statement about '{}' raises interesting ontological questions.", user_input)
            }
            PersonalityType::Balanced => {
                format!(
                    "Examining '{}' requires both empirical and philosophical considerations.",
                    user_input
                )
            }
        };

        format!("{}{}", confidence_prefix, response_content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory_graph::{MemoryGraph, SymbolicNode};

    #[test]
    fn test_kurisu_shell_initialization() {
        // Arrange
        let memory_graph = Arc::new(MemoryGraph::new(None));
        let personality = PersonalityType::Scientific;

        // Act
        let shell = KurisuShell::new(memory_graph, personality);

        // Assert
        assert_eq!(shell.personality, PersonalityType::Scientific);
        assert_eq!(shell.dialogue_history.len(), 0);
    }

    #[test]
    fn test_dialogue_interaction() {
        // Arrange
        let memory_graph = Arc::new(MemoryGraph::new(None));
        let personality = PersonalityType::Scientific;
        let mut shell = KurisuShell::new(memory_graph, personality);

        // Act
        let response = shell.process_input("Tell me about consciousness from the IIT perspective");

        // Assert
        assert!(!response.is_empty());
        assert_eq!(shell.dialogue_history.len(), 2); // User input + response
        assert_eq!(shell.dialogue_history[0].role, "user");
        assert_eq!(shell.dialogue_history[1].role, "system");
    }

    #[test]
    fn test_symbol_identification() {
        // First create the memory graph
        let mut memory_graph = MemoryGraph::new(None);

        // Add concepts directly to the memory graph
        memory_graph
            .add_concept(
                "consciousness_IIT".to_string(),
                SymbolicNode {
                    content: "integrated information above threshold Φ".to_string(),
                    confidence: 0.8,
                    framework: "Integrated Information Theory".to_string(),
                    last_updated: Utc::now().to_rfc3339(),
                },
            )
            .unwrap();

        // Now wrap it in an Arc
        let memory_graph_arc = Arc::new(memory_graph);

        // Continue with the rest of the test
        let personality = PersonalityType::Scientific;
        let shell = KurisuShell::new(memory_graph_arc, personality);

        // Act
        let symbols = shell.identify_symbols_in_text(
            "The IIT perspective on consciousness involves integrated information.",
        );

        // Assert
        assert!(symbols.contains(&"consciousness_IIT".to_string()));
    }

    // New tests to improve coverage

    #[test]
    fn test_process_input_with_question() {
        // Arrange
        let memory_graph = Arc::new(MemoryGraph::new(None));
        let personality = PersonalityType::Scientific;
        let mut shell = KurisuShell::new(memory_graph, personality);

        // Act
        let response = shell.process_input("What is consciousness?");

        // Assert
        assert!(response.contains("[Confidence: Medium]"));

        // Check dialogue history
        let history = shell.get_dialogue_history();
        assert_eq!(history.len(), 2);
        assert_eq!(history[0].role, "user");
        assert_eq!(history[0].content, "What is consciousness?");
        assert_eq!(history[1].role, "system");
        assert_eq!(history[1].content, response);
    }

    #[test]
    fn test_different_personality_types() {
        // Arrange
        let memory_graph = Arc::new(MemoryGraph::new(None));

        // Scientific personality
        let scientific_shell = KurisuShell::new(memory_graph.clone(), PersonalityType::Scientific);
        let scientific_response = scientific_shell
            .generate_response("Tell me about consciousness", ConfidenceLevel::High);

        // Philosophical personality
        let philosophical_shell =
            KurisuShell::new(memory_graph.clone(), PersonalityType::Philosophical);
        let philosophical_response = philosophical_shell
            .generate_response("Tell me about consciousness", ConfidenceLevel::High);

        // Balanced personality
        let balanced_shell = KurisuShell::new(memory_graph.clone(), PersonalityType::Balanced);
        let balanced_response =
            balanced_shell.generate_response("Tell me about consciousness", ConfidenceLevel::High);

        // Assert
        assert!(scientific_response.contains("scientific perspective"));
        assert!(philosophical_response.contains("Philosophically speaking"));
        assert!(balanced_response.contains("both empirical and philosophical"));
    }

    #[test]
    fn test_all_confidence_levels() {
        // Arrange
        let memory_graph = Arc::new(MemoryGraph::new(None));
        let shell = KurisuShell::new(memory_graph, PersonalityType::Scientific);

        // Test all confidence levels
        let low_response = shell.generate_response("consciousness", ConfidenceLevel::Low);
        let medium_response = shell.generate_response("consciousness", ConfidenceLevel::Medium);
        let high_response = shell.generate_response("consciousness", ConfidenceLevel::High);
        let certain_response = shell.generate_response("consciousness", ConfidenceLevel::Certain);

        // Assert
        assert!(low_response.contains("[Confidence: Low]"));
        assert!(medium_response.contains("[Confidence: Medium]"));
        assert!(high_response.contains("[Confidence: High]"));
        assert!(certain_response.contains("[Confidence: Certain]"));
    }

    #[test]
    fn test_empty_dialogue_history() {
        // Arrange
        let memory_graph = Arc::new(MemoryGraph::new(None));
        let shell = KurisuShell::new(memory_graph, PersonalityType::Scientific);

        // Act
        let history = shell.get_dialogue_history();

        // Assert
        assert!(history.is_empty());
    }

    #[test]
    fn test_symbol_identification_with_content_match() {
        // Create memory graph with a concept that has distinctive content
        let mut memory_graph = MemoryGraph::new(None);
        memory_graph
            .add_concept(
                "phi_value".to_string(),
                SymbolicNode {
                    content: "mathematical measure of integrated information".to_string(),
                    confidence: 0.9,
                    framework: "Integrated Information Theory".to_string(),
                    last_updated: Utc::now().to_rfc3339(),
                },
            )
            .unwrap();

        let memory_graph_arc = Arc::new(memory_graph);
        let shell = KurisuShell::new(memory_graph_arc, PersonalityType::Scientific);

        // Test finding the concept via content matching
        let symbols = shell.identify_symbols_in_text(
            "We need to calculate the mathematical measure for this system.",
        );

        // Should find the phi_value concept because "mathematical measure" appears in content
        assert!(symbols.contains(&"phi_value".to_string()));
    }

    #[test]
    fn test_symbol_identification_with_short_words() {
        // Create memory graph with a concept that has short words in content
        let mut memory_graph = MemoryGraph::new(None);
        memory_graph
            .add_concept(
                "test_concept".to_string(),
                SymbolicNode {
                    content: "a set of tiny text".to_string(), // Changed to avoid "words"
                    confidence: 0.7,
                    framework: "Test Framework".to_string(),
                    last_updated: Utc::now().to_rfc3339(),
                },
            )
            .unwrap();

        let memory_graph_arc = Arc::new(memory_graph);
        let shell = KurisuShell::new(memory_graph_arc, PersonalityType::Scientific);

        // Test shouldn't match on words <= 3 characters
        let symbols = shell.identify_symbols_in_text("This is a set of words");

        // Should not match because "a", "set", and "of" are all <= 3 characters
        assert!(symbols.is_empty());

        // Test should match on "tiny" since it's > 3 characters
        let symbols2 = shell.identify_symbols_in_text("This contains tiny text");
        assert!(symbols2.contains(&"test_concept".to_string()));
    }
}
