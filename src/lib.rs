// src/lib.rs

mod goal_tracker;
mod kurisu_shell;
mod memory_graph;
mod proof_engine;

pub use goal_tracker::{Goal, GoalRelation, GoalTracker, GoalType};
pub use kurisu_shell::{ConfidenceLevel, DialogueEntry, KurisuShell, PersonalityType};
pub use memory_graph::{MemoryGraph, SymbolicNode, SymbolicRelation};
pub use proof_engine::{ProofEngine, ProofResult, ProofStatus, TheoremProver};
