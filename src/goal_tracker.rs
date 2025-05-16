// src/goal_tracker.rs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GoalType {
    Terminal,
    Instrumental,
    Tactical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goal {
    pub id: String,
    pub description: String,
    pub type_: GoalType,
    pub parent_ids: Vec<String>,
    pub confidence: f32,
    pub created_at: String,
    pub updated_at: String,
}

impl Default for GoalTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoalRelation {
    pub from_id: String,
    pub to_id: String,
    pub relation_type: String,
    pub strength: f32,
}

pub struct GoalTracker {
    pub goals: HashMap<String, Goal>,
    pub relations: Vec<GoalRelation>,
}

impl GoalTracker {
    pub fn new() -> Self {
        Self {
            goals: HashMap::new(),
            relations: Vec::new(),
        }
    }

    pub fn add_goal(&mut self, goal: Goal) -> Result<(), String> {
        // Check if goal with this ID already exists
        if self.goals.contains_key(&goal.id) {
            return Err(format!("Goal with ID '{}' already exists", goal.id));
        }

        // Add the goal to the hashmap
        self.goals.insert(goal.id.clone(), goal);
        Ok(())
    }

    pub fn relate_goals(&mut self, relation: GoalRelation) -> Result<(), String> {
        // Verify both source and target goals exist
        if !self.goals.contains_key(&relation.from_id) {
            return Err(format!("Source goal '{}' does not exist", relation.from_id));
        }

        if !self.goals.contains_key(&relation.to_id) {
            return Err(format!("Target goal '{}' does not exist", relation.to_id));
        }

        // Add the relation
        self.relations.push(relation);
        Ok(())
    }

    pub fn detect_alignment_drift(&self) -> Vec<(String, String, f32)> {
        let mut drift_results = Vec::new();

        // Find tactical goals with weak alignment to terminal goals
        for (goal_id, goal) in &self.goals {
            if goal.type_ == GoalType::Tactical {
                // Find path to terminal goals
                let hierarchy = self.get_goal_hierarchy_internal(goal_id);

                if hierarchy.len() > 1 {
                    // Calculate the overall alignment strength by finding the weakest link
                    let mut min_strength = 1.0;
                    let mut parent_id = String::new();

                    for i in 0..hierarchy.len() - 1 {
                        // Find relation between current goal and its parent
                        for relation in &self.relations {
                            if relation.from_id == hierarchy[i].id
                                && relation.to_id == hierarchy[i + 1].id
                                && relation.strength < min_strength
                            {
                                min_strength = relation.strength;
                                parent_id = hierarchy[i + 1].id.clone();
                            }
                        }
                    }

                    // If alignment is weak (below 0.5), add to drift results
                    if min_strength < 0.5 {
                        drift_results.push((goal_id.clone(), parent_id, min_strength));
                    }
                }
            }
        }

        drift_results
    }

    pub fn get_goal_hierarchy(&self, goal_id: &str) -> Result<Vec<Goal>, String> {
        if !self.goals.contains_key(goal_id) {
            return Err(format!("Goal '{}' does not exist", goal_id));
        }

        Ok(self.get_goal_hierarchy_internal(goal_id))
    }

    // Internal helper function for getting goal hierarchy
    fn get_goal_hierarchy_internal(&self, goal_id: &str) -> Vec<Goal> {
        let mut hierarchy = Vec::new();

        // Start with the current goal
        if let Some(goal) = self.goals.get(goal_id) {
            hierarchy.push(goal.clone());

            // Follow parent relationships
            let mut current_goal = goal;
            while !current_goal.parent_ids.is_empty() {
                let parent_id = &current_goal.parent_ids[0]; // Take the first parent
                if let Some(parent_goal) = self.goals.get(parent_id) {
                    hierarchy.push(parent_goal.clone());
                    current_goal = parent_goal;
                } else {
                    break; // Parent not found
                }
            }
        }

        hierarchy
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_goal() {
        // Arrange
        let mut tracker = GoalTracker::new();
        let goal = Goal {
            id: "understand_consciousness".to_string(),
            description: "Develop a coherent understanding of consciousness".to_string(),
            type_: GoalType::Terminal,
            parent_ids: vec![],
            confidence: 0.9,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        };

        // Act
        let result = tracker.add_goal(goal.clone());

        // Assert
        assert!(result.is_ok());
        assert_eq!(tracker.goals.len(), 1);
        assert!(tracker.goals.contains_key(&goal.id));
    }

    #[test]
    fn test_goal_hierarchy() {
        // Arrange
        let mut tracker = GoalTracker::new();

        // Terminal goal
        let terminal_goal = Goal {
            id: "understand_consciousness".to_string(),
            description: "Develop a coherent understanding of consciousness".to_string(),
            type_: GoalType::Terminal,
            parent_ids: vec![],
            confidence: 0.9,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        };

        // Instrumental goal that supports terminal goal
        let instrumental_goal = Goal {
            id: "research_iit".to_string(),
            description: "Research Integrated Information Theory".to_string(),
            type_: GoalType::Instrumental,
            parent_ids: vec!["understand_consciousness".to_string()],
            confidence: 0.8,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        };

        // Tactical goal that supports instrumental goal
        let tactical_goal = Goal {
            id: "read_tononi_paper".to_string(),
            description: "Read Tononi's seminal paper on IIT".to_string(),
            type_: GoalType::Tactical,
            parent_ids: vec!["research_iit".to_string()],
            confidence: 0.7,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        };

        tracker.add_goal(terminal_goal).unwrap();
        tracker.add_goal(instrumental_goal).unwrap();
        tracker.add_goal(tactical_goal).unwrap();

        // Create relations
        let relation1 = GoalRelation {
            from_id: "research_iit".to_string(),
            to_id: "understand_consciousness".to_string(),
            relation_type: "supports".to_string(),
            strength: 0.8,
        };

        let relation2 = GoalRelation {
            from_id: "read_tononi_paper".to_string(),
            to_id: "research_iit".to_string(),
            relation_type: "implements".to_string(),
            strength: 0.9,
        };

        tracker.relate_goals(relation1).unwrap();
        tracker.relate_goals(relation2).unwrap();

        // Act
        let hierarchy = tracker.get_goal_hierarchy("read_tononi_paper").unwrap();

        // Assert
        assert_eq!(hierarchy.len(), 3);
        assert_eq!(hierarchy[0].id, "read_tononi_paper");
        assert_eq!(hierarchy[1].id, "research_iit");
        assert_eq!(hierarchy[2].id, "understand_consciousness");
    }

    #[test]
    fn test_alignment_drift_detection() {
        // Arrange
        let mut tracker = GoalTracker::new();

        // Terminal goal
        let terminal_goal = Goal {
            id: "ethical_ai".to_string(),
            description: "Develop AI that aligns with human values".to_string(),
            type_: GoalType::Terminal,
            parent_ids: vec![],
            confidence: 0.9,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        };

        // Instrumental goal that supports terminal goal
        let instrumental_goal = Goal {
            id: "optimize_accuracy".to_string(),
            description: "Maximize prediction accuracy".to_string(),
            type_: GoalType::Instrumental,
            parent_ids: vec!["ethical_ai".to_string()],
            confidence: 0.8,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        };

        // Tactical goal that potentially contradicts terminal values
        let tactical_goal = Goal {
            id: "collect_user_data".to_string(),
            description: "Collect extensive user data without consent".to_string(),
            type_: GoalType::Tactical,
            parent_ids: vec!["optimize_accuracy".to_string()],
            confidence: 0.7,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        };

        tracker.add_goal(terminal_goal).unwrap();
        tracker.add_goal(instrumental_goal).unwrap();
        tracker.add_goal(tactical_goal).unwrap();

        // Create relations with weak alignment
        let relation1 = GoalRelation {
            from_id: "optimize_accuracy".to_string(),
            to_id: "ethical_ai".to_string(),
            relation_type: "supports".to_string(),
            strength: 0.8,
        };

        let relation2 = GoalRelation {
            from_id: "collect_user_data".to_string(),
            to_id: "optimize_accuracy".to_string(),
            relation_type: "implements".to_string(),
            strength: 0.3, // Low strength indicates potential misalignment
        };

        tracker.relate_goals(relation1).unwrap();
        tracker.relate_goals(relation2).unwrap();

        // Act
        let drift_results = tracker.detect_alignment_drift();

        // Assert
        assert!(!drift_results.is_empty());

        // Should detect the tactical goal is misaligned with terminal values
        let has_drift = drift_results
            .iter()
            .any(|(goal_id, _, _)| goal_id == "collect_user_data");
        assert!(has_drift);
    }

    // New tests to improve coverage

    #[test]
    fn test_add_duplicate_goal() {
        // Arrange
        let mut tracker = GoalTracker::new();
        let goal = Goal {
            id: "goal1".to_string(),
            description: "Test goal".to_string(),
            type_: GoalType::Terminal,
            parent_ids: vec![],
            confidence: 0.9,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        };

        // Add the goal first time
        tracker.add_goal(goal.clone()).unwrap();

        // Act - try to add the same goal again
        let result = tracker.add_goal(goal);

        // Assert
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("already exists"));
    }

    #[test]
    fn test_relate_goals_errors() {
        // Arrange
        let mut tracker = GoalTracker::new();
        let goal = Goal {
            id: "goal1".to_string(),
            description: "Test goal".to_string(),
            type_: GoalType::Terminal,
            parent_ids: vec![],
            confidence: 0.9,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        };

        tracker.add_goal(goal).unwrap();

        // Act - try to relate with non-existent source
        let relation1 = GoalRelation {
            from_id: "nonexistent".to_string(),
            to_id: "goal1".to_string(),
            relation_type: "supports".to_string(),
            strength: 0.8,
        };

        let result1 = tracker.relate_goals(relation1);

        // Assert
        assert!(result1.is_err());
        assert!(result1.unwrap_err().contains("does not exist"));

        // Act - try to relate with non-existent target
        let relation2 = GoalRelation {
            from_id: "goal1".to_string(),
            to_id: "nonexistent".to_string(),
            relation_type: "supports".to_string(),
            strength: 0.8,
        };

        let result2 = tracker.relate_goals(relation2);

        // Assert
        assert!(result2.is_err());
        assert!(result2.unwrap_err().contains("does not exist"));
    }

    #[test]
    fn test_get_goal_hierarchy_nonexistent() {
        // Arrange
        let tracker = GoalTracker::new();

        // Act
        let result = tracker.get_goal_hierarchy("nonexistent");

        // Assert
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("does not exist"));
    }

    #[test]
    fn test_get_goal_hierarchy_missing_parent() {
        // Arrange
        let mut tracker = GoalTracker::new();

        // Goal with a parent that doesn't exist
        let goal = Goal {
            id: "orphan_goal".to_string(),
            description: "Goal with missing parent".to_string(),
            type_: GoalType::Tactical,
            parent_ids: vec!["nonexistent_parent".to_string()],
            confidence: 0.7,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        };

        tracker.add_goal(goal).unwrap();

        // Act
        let hierarchy = tracker.get_goal_hierarchy("orphan_goal").unwrap();

        // Assert
        assert_eq!(hierarchy.len(), 1); // Should only contain the goal itself
        assert_eq!(hierarchy[0].id, "orphan_goal");
    }

    #[test]
    fn test_detect_alignment_drift_no_drift() {
        // Arrange
        let mut tracker = GoalTracker::new();

        // Terminal goal
        let terminal_goal = Goal {
            id: "ethical_ai".to_string(),
            description: "Develop AI that aligns with human values".to_string(),
            type_: GoalType::Terminal,
            parent_ids: vec![],
            confidence: 0.9,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        };

        // Tactical goal with strong alignment
        let tactical_goal = Goal {
            id: "privacy_protection".to_string(),
            description: "Implement strong privacy protections".to_string(),
            type_: GoalType::Tactical,
            parent_ids: vec!["ethical_ai".to_string()],
            confidence: 0.9,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        };

        tracker.add_goal(terminal_goal).unwrap();
        tracker.add_goal(tactical_goal).unwrap();

        // Create relation with strong alignment
        let relation = GoalRelation {
            from_id: "privacy_protection".to_string(),
            to_id: "ethical_ai".to_string(),
            relation_type: "supports".to_string(),
            strength: 0.9, // High strength indicates good alignment
        };

        tracker.relate_goals(relation).unwrap();

        // Act
        let drift_results = tracker.detect_alignment_drift();

        // Assert
        assert!(drift_results.is_empty()); // No drift should be detected
    }

    #[test]
    fn test_default_implementation() {
        // Test that the Default trait works
        let tracker = GoalTracker::default();
        assert!(tracker.goals.is_empty());
        assert!(tracker.relations.is_empty());
    }
}
