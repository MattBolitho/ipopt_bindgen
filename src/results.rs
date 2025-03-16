//! # Ipopt Bindgen Results
//!
//! This module defines Rust types for Ipopt results structures.

/// Contains results related for the performance of the optimization.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct PerformanceResults {
    /// The number of evaluations of the objective function.
    pub number_of_objective_evaluations: u32,

    /// The number of evaluations of the objective gradients.
    pub number_of_objective_gradient_evaluations: u32,

    /// The number of evaluations of the constraint functions.
    pub number_of_constraint_evaluations: u32,

    /// The number of evaluations of the Jacobian of the constraints.
    pub number_of_jacobian_evaluations: u32,

    /// The number of evaluations of the Hessian of the Lagrangian.
    pub number_of_hessian_evaluations: u32,
}

/// Contains the numeric solution to the problem.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Solution {
    /// The final values of the optimization variables.
    pub x: Vec<f64>,

    /// The final values of the constraint functions.
    pub constraints: Vec<f64>,

    /// The final values of the constraint multipliers.
    pub lambda: Vec<f64>,

    /// The final values of the lower bound multipliers.
    pub z_l: Vec<f64>,

    /// The final values of the upper bound multipliers.
    pub z_u: Vec<f64>,

    /// The final value of the objective function.
    pub objective: f64,
}

/// An initial solution to a nonlinear problem.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct OptimizationResult {
    /// The numeric solution to the problem.
    pub solution: Solution,

    /// Results relating to the performance of the solve.
    pub performance: PerformanceResults,

    /// The raw status return code from Ipopt.
    pub status: i32,
}
