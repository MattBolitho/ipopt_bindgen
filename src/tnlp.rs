//! # Ipopt Bindgen TNLP
//!
//! Defines an idiomatic Rust type that emulates the `Ipopt::Tnlp` C++ type via the C interface - a
//! base class for all NLP's that use standard triplet matrix form and dense vectors.

/// The dimensions of a nonlinear problem.
///
/// Whilst values in this structure should not be negative, `i32` is used to represent the sizes
/// for Ipopt compatibility.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ProblemSize {
    /// The number of primal variables in the problem.
    pub n: i32,

    /// The number of constraint functions (both equality and inequality) in the problem. If an
    /// inequality constraint function has both lower and upper bounds, it is only counted once.
    pub m: i32,

    /// The number of non-zero elements in the Jacobian of the constraints.
    pub nnz_jac: i32,

    /// The number of non-zero elements in the Hessian of the Lagrangian.
    pub nnz_hess: i32,
}

/// An initial solution to a nonlinear problem.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct InitialSolution {
    /// The initial values of the primal variables.
    pub x: Vec<f64>,

    /// The initial values of the lower bound multipliers.
    ///
    /// This value can be `None` if there are no initial values.
    pub z_l: Option<Vec<f64>>,

    /// The initial values of the upper bound multipliers.
    ///
    /// This value can be `None` if there are no initial values.
    pub z_u: Option<Vec<f64>>,

    /// The initial values of the constraint multipliers.
    ///
    /// This value can be `None` if there are no initial values.
    pub lambda: Option<Vec<f64>>,
}

impl InitialSolution {
    /// Creates a new `InitialSolution` that only contains initial values for the variables.
    ///
    /// # Example
    ///
    /// ```
    /// use ipopt_bindgen::InitialSolution;
    ///
    /// let initial_solution = InitialSolution::from_variables(vec![1.0]);
    ///
    /// assert_eq!(1.0, initial_solution.x[0]);
    /// ```
    ///
    /// # Parameters
    /// - `x` - The initial values of the primal variables.
    #[must_use]
    pub fn from_variables(x: Vec<f64>) -> Self {
        InitialSolution {
            x,
            ..Default::default()
        }
    }
}

/// A trait for NLPs that use standard triplet matrix form and dense vectors.
pub trait Tnlp {
    /// Gets the dimensions of the problem.
    fn get_nlp_info(&self) -> ProblemSize;

    /// Gets the bounds on the variables and constraints.
    ///
    /// For inequality constraints, set the corresponding lower bound if the constraint function is
    /// bounded from below, and/or the corresponding upper bound if the constraint function is
    /// bounded from above. For equality constraints, set the corresponding lower and upper bounds
    /// to the same value.
    ///
    /// # Parameters
    ///
    /// - `x_l` - The variable lower bounds.
    /// - `x_u` - The variable upper bounds.
    /// - `g_l` - The constraint lower bounds.
    /// - `g_u` - The constraint upper bounds.
    fn get_bounds_info(&self, x_l: &mut [f64], x_u: &mut [f64], g_l: &mut [f64], g_u: &mut [f64]);

    /// Gets the problem scaling parameters.
    ///
    /// This is only called if the `nlp_scaling_method` option is set to "user-scaling".
    ///
    /// The default implementation sets all scaling factors to 1.0.
    ///
    /// # Parameters
    ///
    /// - `obj_scaling` - The objective scaling. Use a negative value to maximize.
    /// - `x_scaling` - The variable scaling factors.
    /// - `g_scaling` - The constraint scaling factors.
    fn get_scaling_parameters(
        &self,
        obj_scaling: &mut f64,
        x_scaling: &mut [f64],
        g_scaling: &mut [f64],
    ) {
        *obj_scaling = 1.0;
        x_scaling.fill(1.0);
        g_scaling.fill(1.0);
    }

    /// Gets the initial point for the problem.
    fn get_starting_point(&self) -> InitialSolution;

    /// Evaluates the objective function.
    ///
    /// # Parameters
    /// - `x` - The current variable values.
    /// - `obj_value` - The objective function value.
    ///
    /// # Returns
    /// `true` if the objective was successfully evaluated, `false` otherwise.
    fn eval_f(&mut self, x: &[f64], obj_value: &mut f64) -> bool;

    /// Evaluates the gradients of the objective function.
    ///
    /// # Parameters
    /// - `x` - The current variable values.
    /// - `grad_f` - The gradient of the objective function.
    ///
    /// # Returns
    /// `true` if the gradient was successfully evaluated, `false` otherwise.
    fn eval_grad_f(&mut self, x: &[f64], grad_f: &mut [f64]) -> bool;

    /// Evaluates the constraint functions.
    ///
    /// # Parameters
    /// - `x` - The current variable values.
    /// - `g` - The constraint function values.
    ///
    /// # Returns
    /// `true` if the constraints were successfully evaluated, `false` otherwise.
    fn eval_g(&mut self, x: &[f64], g: &mut [f64]) -> bool;

    /// Gets the non-zero indices for the Jacobian of the constraints.
    ///
    /// The `i_row` and `j_col` slices should be filled with the row column index pairs
    /// respectively. The size of the index slices will equal the number of non-zero elements
    /// in the Jacobian of the constraints.
    ///
    /// # Parameters
    /// - `n` - The number of variables in the problem.
    /// - `m` - The number of constraints in the problem.
    /// - `i_row` - The row indices of the non-zero elements.
    /// - `j_col` - The column indices of the non-zero elements.
    #[cold]
    fn get_jacobian_sparsity(&mut self, n: i32, m: i32, i_row: &mut [i32], j_col: &mut [i32]);

    /// Evaluates the Jacobian of the constraints.
    ///
    /// # Parameters
    /// - `x` - The current variable values.
    /// - `m` - The number of constraints in the problem.
    /// - `values` - The non-zero values of the Jacobian of the constraints.
    ///
    /// # Returns
    /// `true` if the Jacobian was successfully evaluated, `false` otherwise.
    fn eval_jac_g(&mut self, x: &[f64], m: i32, values: &mut [f64]) -> bool;

    /// Gets the non-zero indices for the Hessian of the Lagrangian.
    ///
    /// The `i_row` and `j_col` slices should be filled with the row column index pairs
    /// respectively. The size of the index slices will equal the number of non-zero elements
    /// in the  Hessian of the Lagrangian.
    ///
    /// # Parameters
    /// - `n` - The number of variables in the problem.
    /// - `m` - The number of constraints in the problem.
    /// - `i_row` - The row indices of the non-zero elements.
    /// - `j_col` - The column indices of the non-zero elements.
    #[cold]
    fn get_hessian_sparsity(&mut self, n: i32, m: i32, i_row: &mut [i32], j_col: &mut [i32]);

    /// Evaluates the Hessian of the Lagrangian.
    ///
    /// # Parameters
    /// - `x` - The current variable values.
    /// - `m` - The number of constraints in the problem.
    /// - `values` - The non-zero values of the Hessian of the Lagrangian.
    ///
    /// # Returns
    /// `true` if the Jacobian was successfully evaluated, `false` otherwise.
    fn eval_h(
        &mut self,
        x: &[f64],
        obj_factor: f64,
        lambda: &[f64],
        m: i32,
        values: &mut [f64],
    ) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_solution_new_from_variables_returns_expected_value() {
        let x = vec![1.0, 2.0, 3.0];

        let initial_solution = InitialSolution::from_variables(x.clone());

        assert_eq!(x, initial_solution.x)
    }
}
