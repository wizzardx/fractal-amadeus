// src/memory_graph.rs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// The MemoryGraph stores symbolic relationships across sessions.
/// It maintains concept definitions, project trajectories, and verification histories.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MemoryGraph {
    // Each node contains content and meta-information about symbolic significance
    concepts: HashMap<String, SymbolicNode>,
    // Tracks relationships between concepts
    relationships: Vec<SymbolicRelation>,
    // Configuration for persistence
    persistence_path: Option<String>,
}

/// Represents a node in the Memory Graph containing conceptual information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SymbolicNode {
    // The concept's definition or content
    pub content: String,
    // Confidence level in the concept (0.0 to 1.0)
    pub confidence: f32,
    // Domain or framework this concept belongs to
    pub framework: String,
    // When this concept was last updated
    pub last_updated: String,
}

/// Represents a relationship between two symbolic nodes
#[derive(Debug, Serialize, Deserialize)]
pub struct SymbolicRelation {
    // Source concept identifier
    pub from: String,
    // Target concept identifier
    pub to: String,
    // Type of relationship (e.g., "is_a", "part_of", "contradicts")
    pub relation_type: String,
    // Confidence in this relationship (0.0 to 1.0)
    pub confidence: f32,
}

impl MemoryGraph {
    /// Create a new memory graph with optional persistence path
    pub fn new(persistence_path: Option<String>) -> Self {
        Self {
            concepts: HashMap::new(),
            relationships: Vec::new(),
            persistence_path,
        }
    }

    /// Add a concept to the memory graph
    pub fn add_concept(&mut self, key: String, concept: SymbolicNode) -> Result<(), String> {
        self.concepts.insert(key, concept);
        Ok(())
    }

    /// Retrieve a concept from the memory graph
    pub fn get_concept(&self, key: &str) -> Option<&SymbolicNode> {
        self.concepts.get(key)
    }

    /// Add a relationship between concepts
    pub fn add_relationship(&mut self, relation: SymbolicRelation) -> Result<(), String> {
        // Verify both source and target concepts exist
        if !self.concepts.contains_key(&relation.from) {
            return Err(format!("Source concept '{}' does not exist", relation.from));
        }

        if !self.concepts.contains_key(&relation.to) {
            return Err(format!("Target concept '{}' does not exist", relation.to));
        }

        // Add the relationship
        self.relationships.push(relation);
        Ok(())
    }

    /// Save the memory graph to a file
    pub fn save(&self) -> Result<(), String> {
        // Check if we have a path to save to
        let path = match &self.persistence_path {
            Some(path) => path,
            None => return Err("No persistence path specified".to_string()),
        };

        // Serialize the memory graph to YAML
        let yaml = match serde_yaml::to_string(self) {
            Ok(yaml) => yaml,
            Err(e) => return Err(format!("Failed to serialize memory graph: {}", e)),
        };

        // Write to file
        match std::fs::write(path, yaml) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to write to file {}: {}", path, e)),
        }
    }

    /// Load the memory graph from a file
    pub fn load(path: &Path) -> Result<Self, String> {
        // Check if the file exists
        if !path.exists() {
            return Err(format!("File {} does not exist", path.display()));
        }

        // Read the file
        let contents = match std::fs::read_to_string(path) {
            Ok(contents) => contents,
            Err(e) => return Err(format!("Failed to read file {}: {}", path.display(), e)),
        };

        // Deserialize the YAML
        let mut memory_graph: MemoryGraph = match serde_yaml::from_str(&contents) {
            Ok(graph) => graph,
            Err(e) => return Err(format!("Failed to deserialize memory graph: {}", e)),
        };

        // Store the path for future saves
        memory_graph.persistence_path = Some(path.to_string_lossy().to_string());

        Ok(memory_graph)
    }

    /// Update an existing concept in the memory graph
    pub fn update_concept(&mut self, key: String, concept: SymbolicNode) -> Result<(), String> {
        if !self.concepts.contains_key(&key) {
            return Err(format!(
                "Concept '{}' does not exist in the memory graph",
                key
            ));
        }
        self.concepts.insert(key, concept);
        Ok(())
    }

    /// Get all relationships for a specific concept
    pub fn get_relationships_for_concept(&self, concept_key: &str) -> Vec<&SymbolicRelation> {
        self.relationships
            .iter()
            .filter(|rel| rel.from == concept_key || rel.to == concept_key)
            .collect()
    }

    /// Additional helper for tests (needed for KurisuShell::identify_symbols_in_text)
    pub fn get_all_concepts(&self) -> impl Iterator<Item = (&String, &SymbolicNode)> {
        self.concepts.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_add_and_retrieve_concept() {
        // Arrange
        let mut memory_graph = MemoryGraph::new(None);
        let concept_key = "consciousness_IIT".to_string();
        let concept = SymbolicNode {
            content: "integrated information above threshold Φ".to_string(),
            confidence: 0.8,
            framework: "Integrated Information Theory".to_string(),
            last_updated: Utc::now().to_rfc3339(),
        };

        // Act
        let add_result = memory_graph.add_concept(concept_key.clone(), concept.clone());
        let retrieved_concept = memory_graph.get_concept(&concept_key);

        // Assert
        assert!(add_result.is_ok());
        assert!(retrieved_concept.is_some());
        let retrieved = retrieved_concept.unwrap();
        assert_eq!(retrieved.content, concept.content);
        assert_eq!(retrieved.confidence, concept.confidence);
        assert_eq!(retrieved.framework, concept.framework);
    }

    #[test]
    fn test_add_relationship() {
        // Arrange
        let mut memory_graph = MemoryGraph::new(None);

        // Add two concepts
        let concept1_key = "consciousness_IIT".to_string();
        let concept1 = SymbolicNode {
            content: "integrated information above threshold Φ".to_string(),
            confidence: 0.8,
            framework: "Integrated Information Theory".to_string(),
            last_updated: chrono::Utc::now().to_rfc3339(),
        };

        let concept2_key = "phi_value".to_string();
        let concept2 = SymbolicNode {
            content: "mathematical measure of integrated information".to_string(),
            confidence: 0.9,
            framework: "Integrated Information Theory".to_string(),
            last_updated: chrono::Utc::now().to_rfc3339(),
        };

        memory_graph
            .add_concept(concept1_key.clone(), concept1)
            .unwrap();
        memory_graph
            .add_concept(concept2_key.clone(), concept2)
            .unwrap();

        // Create a relationship
        let relation = SymbolicRelation {
            from: concept1_key.clone(),
            to: concept2_key.clone(),
            relation_type: "depends_on".to_string(),
            confidence: 0.7,
        };

        // Act
        let add_result = memory_graph.add_relationship(relation);

        // Assert
        assert!(add_result.is_ok());

        // Get relationships
        let relationships = memory_graph.get_relationships_for_concept(&concept1_key);
        assert_eq!(relationships.len(), 1);
        assert_eq!(relationships[0].relation_type, "depends_on");
        assert_eq!(relationships[0].to, concept2_key);
    }

    #[test]
    fn test_persistence_save_and_load() {
        use std::fs;
        use std::path::PathBuf;

        // Arrange
        let temp_file = "test_memory_graph.yaml";
        let temp_path = PathBuf::from(temp_file);

        // Create a memory graph with some data
        let mut original_graph = MemoryGraph::new(Some(temp_file.to_string()));

        let concept_key = "consciousness_IIT".to_string();
        let concept = SymbolicNode {
            content: "integrated information above threshold Φ".to_string(),
            confidence: 0.8,
            framework: "Integrated Information Theory".to_string(),
            last_updated: chrono::Utc::now().to_rfc3339(),
        };

        original_graph
            .add_concept(concept_key.clone(), concept)
            .unwrap();

        // Act
        // Save the memory graph
        let save_result = original_graph.save();
        assert!(save_result.is_ok());

        // Load it back
        let loaded_result = MemoryGraph::load(&temp_path);
        assert!(loaded_result.is_ok());
        let loaded_graph = loaded_result.unwrap();

        // Assert
        let loaded_concept = loaded_graph.get_concept(&concept_key);
        assert!(loaded_concept.is_some());
        assert_eq!(
            loaded_concept.unwrap().content,
            "integrated information above threshold Φ"
        );

        // Cleanup
        fs::remove_file(temp_path).unwrap();
    }

    #[test]
    fn test_update_concept() {
        // Arrange
        let mut memory_graph = MemoryGraph::new(None);
        let concept_key = "consciousness_IIT".to_string();
        let concept = SymbolicNode {
            content: "integrated information above threshold Φ".to_string(),
            confidence: 0.8,
            framework: "Integrated Information Theory".to_string(),
            last_updated: chrono::Utc::now().to_rfc3339(),
        };

        memory_graph
            .add_concept(concept_key.clone(), concept)
            .unwrap();

        // Create updated concept with higher confidence
        let updated_concept = SymbolicNode {
            content: "integrated information above threshold Φ, requires causal power".to_string(),
            confidence: 0.9,
            framework: "Integrated Information Theory".to_string(),
            last_updated: chrono::Utc::now().to_rfc3339(),
        };

        // Act
        let update_result =
            memory_graph.update_concept(concept_key.clone(), updated_concept.clone());

        // Assert
        assert!(update_result.is_ok());
        let retrieved = memory_graph.get_concept(&concept_key).unwrap();
        assert_eq!(retrieved.confidence, 0.9);
        assert_eq!(
            retrieved.content,
            "integrated information above threshold Φ, requires causal power"
        );
    }

    #[test]
    fn test_error_when_adding_relationship_to_nonexistent_concept() {
        // Arrange
        let mut memory_graph = MemoryGraph::new(None);

        // Add one concept
        let concept1_key = "consciousness_IIT".to_string();
        let concept1 = SymbolicNode {
            content: "integrated information above threshold Φ".to_string(),
            confidence: 0.8,
            framework: "Integrated Information Theory".to_string(),
            last_updated: chrono::Utc::now().to_rfc3339(),
        };

        memory_graph
            .add_concept(concept1_key.clone(), concept1)
            .unwrap();

        // Try to create a relationship with non-existent concept
        let relation = SymbolicRelation {
            from: concept1_key.clone(),
            to: "non_existent_concept".to_string(),
            relation_type: "depends_on".to_string(),
            confidence: 0.7,
        };

        // Act
        let add_result = memory_graph.add_relationship(relation);

        // Assert
        assert!(add_result.is_err());
        assert!(add_result.unwrap_err().contains("does not exist"));
    }

    // New tests to improve coverage

    #[test]
    fn test_save_error_handling() {
        // Arrange - create graph with no persistence path
        let memory_graph = MemoryGraph::new(None);

        // Act - try to save
        let result = memory_graph.save();

        // Assert
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("No persistence path specified"));
    }

    #[test]
    fn test_load_nonexistent_file() {
        // Act - try to load from nonexistent path
        let result = MemoryGraph::load(Path::new("/nonexistent/path/memory.yaml"));

        // Assert
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("does not exist"));
    }

    #[test]
    fn test_load_invalid_yaml() {
        // Arrange - create temp file with invalid YAML
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("invalid.yaml");
        fs::write(&file_path, "invalid: - yaml: content").unwrap();

        // Act - try to load
        let result = MemoryGraph::load(&file_path);

        // Assert
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Failed to deserialize"));
    }

    #[test]
    fn test_update_nonexistent_concept() {
        // Arrange
        let mut memory_graph = MemoryGraph::new(None);
        let concept = SymbolicNode {
            content: "test content".to_string(),
            confidence: 0.8,
            framework: "test framework".to_string(),
            last_updated: Utc::now().to_rfc3339(),
        };

        // Act - try to update nonexistent concept
        let result = memory_graph.update_concept("nonexistent".to_string(), concept);

        // Assert
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("does not exist"));
    }

    #[test]
    fn test_file_read_error() {
        // Test case for when file exists but can't be read
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            // Create a temp file with no read permissions
            let temp_dir = tempdir().unwrap();
            let file_path = temp_dir.path().join("unreadable.yaml");
            fs::write(&file_path, "test: data").unwrap();

            // Set no-read permissions
            let mut perms = fs::metadata(&file_path).unwrap().permissions();
            perms.set_mode(0o000);
            fs::set_permissions(&file_path, perms).unwrap();

            // Try to load
            let result = MemoryGraph::load(&file_path);

            // Should get a permission error
            assert!(result.is_err());
            assert!(result.unwrap_err().contains("Failed to read file"));

            // Restore permissions for cleanup
            let mut perms = fs::metadata(&file_path).unwrap().permissions();
            perms.set_mode(0o644);
            fs::set_permissions(&file_path, perms).unwrap();
        }
    }

    #[test]
    fn test_write_error() {
        // Test case for when file path exists but can't be written
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            // Create a temp directory with no write permissions
            let temp_dir = tempdir().unwrap();
            let dir_path = temp_dir.path();

            // Create a memory graph with path in read-only directory
            let path_str = dir_path.join("test.yaml").to_string_lossy().to_string();
            let mut memory_graph = MemoryGraph::new(Some(path_str));

            // Add a concept
            let concept = SymbolicNode {
                content: "test".to_string(),
                confidence: 0.8,
                framework: "test".to_string(),
                last_updated: Utc::now().to_rfc3339(),
            };
            memory_graph
                .add_concept("test".to_string(), concept)
                .unwrap();

            // Set no-write permissions on directory
            let mut perms = fs::metadata(dir_path).unwrap().permissions();
            perms.set_mode(0o500); // r-x permissions, no write
            fs::set_permissions(dir_path, perms).unwrap();

            // Try to save
            let result = memory_graph.save();

            // Should get a permission error
            assert!(result.is_err());
            assert!(result.unwrap_err().contains("Failed to write to file"));

            // Restore permissions for cleanup
            let mut perms = fs::metadata(dir_path).unwrap().permissions();
            perms.set_mode(0o755);
            fs::set_permissions(dir_path, perms).unwrap();
        }
    }

    #[test]
    fn test_serialization_error() {
        // This test is difficult to create in practice because serde_yaml can serialize
        // pretty much any valid Rust struct. Instead, we'll test the success case.

        // Create a memory graph
        let memory_graph = MemoryGraph::new(Some("test.yaml".to_string()));

        // Serialize it
        let yaml_result = serde_yaml::to_string(&memory_graph);

        // Should succeed
        assert!(yaml_result.is_ok());
    }

    #[test]
    fn test_get_relationships_empty() {
        // Create a memory graph with a concept but no relationships
        let mut memory_graph = MemoryGraph::new(None);

        // Add a concept
        let concept = SymbolicNode {
            content: "test".to_string(),
            confidence: 0.8,
            framework: "test".to_string(),
            last_updated: Utc::now().to_rfc3339(),
        };
        memory_graph
            .add_concept("test".to_string(), concept)
            .unwrap();

        // Get relationships for concept
        let relationships = memory_graph.get_relationships_for_concept("test");

        // Should be empty
        assert!(relationships.is_empty());
    }
}
