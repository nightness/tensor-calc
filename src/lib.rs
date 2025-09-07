//! # tensor-calc
//!
//! A Rust library for symbolic tensor calculus and Einstein field equation solving.
//!
//! This library provides comprehensive tools for:
//! - Symbolic tensor calculus operations
//! - Einstein field equation solving
//! - Spacetime metric computations
//! - General relativity calculations
//!
//! ## Quick Start
//!
//! ```rust
//! use tensor_calc::{solve_vacuum_einstein_equations};
//!
//! // Solve for Schwarzschild spacetime
//! let coords = vec!["t".to_string(), "r".to_string(), "theta".to_string(), "phi".to_string()];
//! let solutions = solve_vacuum_einstein_equations(&coords, "spherical", &[]).unwrap();
//! assert!(!solutions.is_empty());
//! ```
//!
//! ## Examples
//!
//! ### Computing Einstein Solutions
//! ```rust
//! use tensor_calc::{solve_vacuum_einstein_equations};
//!
//! let coords = vec!["t".to_string(), "r".to_string(), "theta".to_string(), "phi".to_string()];
//! let solutions = solve_vacuum_einstein_equations(&coords, "spherical", &[]).unwrap();
//! 
//! // Should find Schwarzschild and Reissner-Nordström solutions
//! assert_eq!(solutions.len(), 2);
//! assert_eq!(solutions[0].solution_type, "exact");
//! ```

pub mod symbolic;
pub mod tensor;
pub mod einstein;

// Re-export commonly used types and functions
pub use symbolic::*;
pub use tensor::*;
pub use einstein::*;

// Re-export error type
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TensorError {
    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("Invalid metric tensor: {0}")]
    InvalidMetric(String),
    #[error("Computation error: {0}")]
    ComputationError(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TensorResult {
    pub result_type: String,
    pub data: serde_json::Value,
    pub coordinates: Vec<String>,
    pub success: bool,
    pub error: Option<String>,
}