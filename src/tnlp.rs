//! # Ipopt Bindgen TNLP
//!
//! Defines an idiomatic Rust type that emulates the `Ipopt::Tnlp` C++ type via the C interface - a
//! base class for all NLP's that use standard triplet matrix form and dense vectors.

/// A trait for NLPs that use standard triplet matrix form and dense vectors.
pub trait Tnlp {
    /// Gets the initial information about the problem.
    ///
    /// # Parameters
    ///
    /// - `n` - The number of variables in the problem.
    /// - `m` - The number of constraints in the problem.
    /// - `nnz_jac_g` - The number of non-zero elements in the Jacobian of the constraints.
    /// - `nnz_h_lag` - The number of non-zero elements in the Hessian of the Lagrangian.
    fn get_nlp_info(n: &mut i32, m: &mut i32, nnz_jac_g: &mut i32, nnz_h_lag: &mut i32);

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
    fn get_bounds_info(x_l: &mut [f64], x_u: &mut [f64], g_l: &mut [f64], g_u: &mut [f64]);

    /// Gets the problem scaling parameters.
    ///
    /// This is only called if the nlp_scaling_method is set to "user-scaling".
    ///
    /// The default implementation sets all scaling factors to 1.0.
    ///
    /// # Parameters
    ///
    /// - `obj_scaling` - The objective scaling. Use a negative value to maximize.
    /// - `x_scaling` - The variable scaling factors.
    /// - `g_scaling` - The constraint scaling factors.
    fn get_scaling_parameters(obj_scaling: &mut f64, x_scaling: &mut [f64], g_scaling: &mut [f64]) {
        *obj_scaling = 1.0;
        x_scaling.fill(1.0);
        g_scaling.fill(1.0);
    }

    /// Gets the initial point for the problem.
    ///
    /// # Parameters
    /// - `x` - The initial variable values.
    /// - `init_x` - Whether to initialize `x`.
    /// - `z_l` - The initial lower bound multipliers.
    /// - `z_u` - The initial upper bound multipliers.
    /// - `init_z` - Whether to initialize `z_l` and `z_u`.
    /// - `lambda` - The initial constraint multipliers.
    /// - `init_lambda` - Whether to initialize `lambda`.
    fn get_starting_point(
        x: &mut [f64],
        init_x: bool,
        z_l: &mut [f64],
        z_u: &mut [f64],
        init_z: bool,
        lambda: &mut [f64],
        init_lambda: bool,
    );

    /// Evaluates the objective function.
    ///
    /// # Parameters
    /// - `x` - The current variable values.
    /// - `obj_value` - The objective function value.
    ///
    /// # Returns
    /// `true` if the objective was successfully evaluated, `false` otherwise.
    fn eval_f(x: &[f64], obj_value: &mut f64) -> bool;

    /// Evaluates the gradients of the objective function.
    ///
    /// # Parameters
    /// - `x` - The current variable values.
    /// - `grad_f` - The gradient of the objective function.
    ///
    /// # Returns
    /// `true` if the gradient was successfully evaluated, `false` otherwise.
    fn eval_grad_f(x: &[f64], grad_f: &mut [f64]) -> bool;

    /// Evaluates the constraint functions.
    ///
    /// # Parameters
    /// - `x` - The current variable values.
    /// - `g` - The constraint function values.
    ///
    /// # Returns
    /// `true` if the constraints were successfully evaluated, `false` otherwise.
    fn eval_g(x: &[f64], g: &mut [f64]) -> bool;

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
    fn get_jacobian_sparsity(n: i32, m: i32, i_row: &mut [i32], j_col: &mut [i32]);

    /// Evaluates the Jacobian of the constraints.
    ///
    /// # Parameters
    /// - `x` - The current variable values.
    /// - `m` - The number of constraints in the problem.
    /// - `values` - The non-zero values of the Jacobian of the constraints.
    ///
    /// # Returns
    /// `true` if the Jacobian was successfully evaluated, `false` otherwise.
    fn eval_jac_g(x: &[f64], m: i32, values: &mut [f64]) -> bool;

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
    fn get_hessian_sparsity(n: i32, m: i32, i_row: &mut [i32], j_col: &mut [i32]);

    /// Evaluates the Hessian of the Lagrangian.
    ///
    /// # Parameters
    /// - `x` - The current variable values.
    /// - `m` - The number of constraints in the problem.
    /// - `values` - The non-zero values of the Hessian of the Lagrangian.
    ///
    /// # Returns
    /// `true` if the Jacobian was successfully evaluated, `false` otherwise.
    fn eval_h(x: &[f64], obj_factor: f64, lambda: &[f64], m: i32, values: &mut [f64]) -> bool;
}
