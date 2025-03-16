//! # Ipopt Bindgen Application
//!
//! This module defines an idiomatic Rust emulation of the `Ipopt::Application` C++ type via the C
//! interface - supporting functionality like configuring the optimizer and optimizing problems.

#[allow(clippy::wildcard_imports)]

use crate::{c_interface::*, results::OptimizationResult, tnlp::Tnlp};
use std::{collections::HashMap, error::Error, ffi::CString, os::raw::c_void, slice};

/// The main application type for making calls to Ipopt.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Application {
    int_options: HashMap<String, i32>,
    numeric_options: HashMap<String, f64>,
    string_options: HashMap<String, String>,
}

/// Helper type for passing Ipopt Bingen problem and results structures to Ipopt callbacks.
#[derive(Debug)]
struct IpoptBindgenUserData<'a, P: Tnlp> {
    pub problem: &'a mut P,
    pub results: &'a mut OptimizationResult,
}

impl<'a, P: Tnlp> IpoptBindgenUserData<'a, P> {
    pub fn new(problem: &'a mut P, results: &'a mut OptimizationResult) -> Self {
        Self { problem, results }
    }

    pub fn reify_from_void_ptr(value: *mut c_void) -> &'a mut Self {
        debug_assert!(!value.is_null());
        unsafe { &mut *value.cast::<Self>() }
    }
}

impl Application {
    /// Creates a new instance of an `Application`.
    ///
    /// # Example
    ///
    /// ```
    /// use ipopt_bindgen::Application;
    ///
    /// let application = Application::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Application {
            ..Default::default()
        }
    }

    /// Sets an integer option.
    ///
    /// # Parameters
    ///
    /// - `key` - The name of the option (as per the Ipopt documentation).
    /// - `value` - The value to set the option to.
    ///
    /// # Example
    ///
    /// ```
    /// use ipopt_bindgen::Application;
    ///
    /// let mut application = Application::new();
    ///
    /// application.set_integer_option("max_iter", 500);
    /// ```
    pub fn set_integer_option(
        &mut self,
        key: impl Into<String>,
        value: impl Into<i32>,
    ) -> &mut Self {
        self.int_options.insert(key.into(), value.into());
        self
    }

    /// Sets a numeric option.
    ///
    /// # Parameters
    ///
    /// - `key` - The name of the option (as per the Ipopt documentation).
    /// - `value` - The value to set the option to.
    ///
    /// # Example
    ///
    /// ```
    /// use ipopt_bindgen::Application;
    ///
    /// let mut application = Application::new();
    ///
    /// application.set_numeric_option("mu_init", 1e-2);
    /// ```
    pub fn set_numeric_option(
        &mut self,
        key: impl Into<String>,
        value: impl Into<f64>,
    ) -> &mut Self {
        self.numeric_options.insert(key.into(), value.into());
        self
    }

    /// Sets a string option.
    ///
    /// # Parameters
    ///
    /// - `key` - The name of the option (as per the Ipopt documentation).
    /// - `value` - The value to set the option to.
    ///
    /// # Example
    ///
    /// ```
    /// use ipopt_bindgen::Application;
    ///
    /// let mut application = Application::new();
    ///
    /// application.set_string_option("linear_solver", "mumps");
    /// ```
    pub fn set_string_option(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> &mut Self {
        self.string_options.insert(key.into(), value.into());
        self
    }

    extern "C" fn objective_callback<P: Tnlp>(
        n: ipindex,
        x: *mut ipnumber,
        _new_x: bool,
        obj_value: *mut ipnumber,
        user_data_ptr: UserDataPtr,
    ) -> bool {
        let n_var = usize::try_from(n).unwrap();

        debug_assert!(!x.is_null());
        let x_slice = unsafe { slice::from_raw_parts(x, n_var) };

        debug_assert!(!user_data_ptr.is_null());
        let user_data: &mut IpoptBindgenUserData<'_, P> =
            IpoptBindgenUserData::reify_from_void_ptr(user_data_ptr);

        user_data
            .results
            .performance
            .number_of_objective_evaluations += 1;
        user_data
            .problem
            .eval_f(x_slice, unsafe { &mut *obj_value })
    }

    extern "C" fn gradient_callback<P: Tnlp>(
        n: ipindex,
        x: *mut ipnumber,
        _new_x: bool,
        grad_f: *mut ipnumber,
        user_data_ptr: UserDataPtr,
    ) -> bool {
        let n_var = usize::try_from(n).unwrap();

        debug_assert!(!x.is_null());
        let x_slice = unsafe { slice::from_raw_parts(x, n_var) };

        debug_assert!(!grad_f.is_null());
        let grad_slice = unsafe { slice::from_raw_parts_mut(grad_f, n_var) };

        debug_assert!(!user_data_ptr.is_null());
        let user_data: &mut IpoptBindgenUserData<'_, P> =
            IpoptBindgenUserData::reify_from_void_ptr(user_data_ptr);

        user_data
            .results
            .performance
            .number_of_objective_gradient_evaluations += 1;
        user_data.problem.eval_grad_f(x_slice, grad_slice)
    }

    extern "C" fn constraints_callback<P: Tnlp>(
        n: ipindex,
        x: *mut ipnumber,
        _new_x: bool,
        m: ipindex,
        g: *mut ipnumber,
        user_data_ptr: UserDataPtr,
    ) -> bool {
        let n_var = usize::try_from(n).unwrap();
        let n_cons = usize::try_from(m).unwrap();

        debug_assert!(!x.is_null());
        let x_slice = unsafe { slice::from_raw_parts(x, n_var) };

        debug_assert!(!g.is_null());
        let g_slice = unsafe { slice::from_raw_parts_mut(g, n_cons) };

        debug_assert!(!user_data_ptr.is_null());
        let user_data: &mut IpoptBindgenUserData<'_, P> =
            IpoptBindgenUserData::reify_from_void_ptr(user_data_ptr);

        user_data
            .results
            .performance
            .number_of_constraint_evaluations += 1;
        user_data.problem.eval_g(x_slice, g_slice)
    }

    extern "C" fn jacobian_callback<P: Tnlp>(
        n: ipindex,
        x: *mut ipnumber,
        _new_x: bool,
        m: ipindex,
        nele_jac: ipindex,
        i_row: *mut ipindex,
        j_col: *mut ipindex,
        values: *mut ipnumber,
        user_data_ptr: UserDataPtr,
    ) -> bool {
        let n_var = usize::try_from(n).unwrap();
        let nnz = usize::try_from(nele_jac).unwrap();

        debug_assert!(!user_data_ptr.is_null());
        let user_data: &mut IpoptBindgenUserData<'_, P> =
            IpoptBindgenUserData::reify_from_void_ptr(user_data_ptr);

        if x.is_null() {
            debug_assert!(!i_row.is_null());
            let row_slice = unsafe { slice::from_raw_parts_mut(i_row, nnz) };

            debug_assert!(!j_col.is_null());
            let col_slice = unsafe { slice::from_raw_parts_mut(j_col, nnz) };

            user_data
                .problem
                .get_jacobian_sparsity(n, m, row_slice, col_slice);
            true
        } else {
            debug_assert!(!x.is_null());
            let x_slice = unsafe { slice::from_raw_parts(x, n_var) };

            debug_assert!(!values.is_null());
            let jac_slice = unsafe { slice::from_raw_parts_mut(values, nnz) };

            user_data.results.performance.number_of_jacobian_evaluations += 1;
            user_data.problem.eval_jac_g(x_slice, m, jac_slice)
        }
    }

    extern "C" fn hessian_callback<P: Tnlp>(
        n: ipindex,
        x: *mut ipnumber,
        _new_x: bool,
        obj_factor: ipnumber,
        m: ipindex,
        lambda: *mut ipnumber,
        _new_lambda: bool,
        nele_hess: ipindex,
        i_row: *mut ipindex,
        j_col: *mut ipindex,
        values: *mut ipnumber,
        user_data_ptr: UserDataPtr,
    ) -> bool {
        let n_var = usize::try_from(n).unwrap();
        let n_cons = usize::try_from(m).unwrap();
        let nnz = usize::try_from(nele_hess).unwrap();

        debug_assert!(!user_data_ptr.is_null());
        let user_data: &mut IpoptBindgenUserData<'_, P> =
            IpoptBindgenUserData::reify_from_void_ptr(user_data_ptr);

        if x.is_null() {
            debug_assert!(!i_row.is_null());
            let row_slice = unsafe { slice::from_raw_parts_mut(i_row, nnz) };

            debug_assert!(!j_col.is_null());
            let col_slice = unsafe { slice::from_raw_parts_mut(j_col, nnz) };

            user_data
                .problem
                .get_hessian_sparsity(n, m, row_slice, col_slice);
            true
        } else {
            debug_assert!(!x.is_null());
            let x_slice = unsafe { slice::from_raw_parts(x, n_var) };

            debug_assert!(!lambda.is_null());
            let lambda_slice = unsafe { slice::from_raw_parts(lambda, n_cons) };

            debug_assert!(!values.is_null());
            let hessian = unsafe { slice::from_raw_parts_mut(values, nnz) };

            user_data.results.performance.number_of_hessian_evaluations += 1;
            user_data
                .problem
                .eval_h(x_slice, obj_factor, lambda_slice, m, hessian)
        }
    }

    /// Optimizes the problem.
    ///
    /// # Parameters
    ///
    /// - `problem` - The problem to optimize.
    ///
    /// # Errors
    ///
    /// - The problem dimensions are not representable as a usize.
    /// - A C compatible string cannot be created for an option.
    ///
    /// # Returns
    ///
    /// A result containing the optimization results.
    pub fn optimize_tnlp<P: Tnlp>(
        &self,
        mut problem: P,
    ) -> Result<OptimizationResult, Box<dyn Error>> {
        const C_STYLE_INDEXING: i32 = 0;

        let problem_size = problem.get_nlp_info();
        let n = usize::try_from(problem_size.n)?;
        let m = usize::try_from(problem_size.m)?;
        usize::try_from(problem_size.nnz_jac)?;
        usize::try_from(problem_size.nnz_hess)?;

        let mut x_l = vec![0.0; n];
        let mut x_u = vec![0.0; n];
        let mut g_l = vec![0.0; m];
        let mut g_u = vec![0.0; m];
        problem.get_bounds_info(
            x_l.as_mut_slice(),
            x_u.as_mut_slice(),
            g_l.as_mut_slice(),
            g_u.as_mut_slice(),
        );

        let initial_solution = problem.get_starting_point();
        let mut results = OptimizationResult::default();
        let mut user_data = IpoptBindgenUserData::new(&mut problem, &mut results);
        let user_data_ptr = &raw mut user_data as UserDataPtr;

        let problem = unsafe {
            CreateIpoptProblem(
                problem_size.n,
                x_l.as_mut_ptr(),
                x_u.as_mut_ptr(),
                problem_size.m,
                g_l.as_mut_ptr(),
                g_u.as_mut_ptr(),
                problem_size.nnz_jac,
                problem_size.nnz_hess,
                C_STYLE_INDEXING,
                Some(Self::objective_callback::<P>),
                Some(Self::constraints_callback::<P>),
                Some(Self::gradient_callback::<P>),
                Some(Self::jacobian_callback::<P>),
                Some(Self::hessian_callback::<P>),
            )
        };

        // Add the options to the C problem. Todo - see if we can avoid copying the strings.
        for (option, value) in &self.int_options {
            let name = CString::new(option.as_bytes())?;
            unsafe {
                AddIpoptIntOption(problem, name.into_raw(), *value);
            }
        }
        for (option, value) in &self.string_options {
            let name = CString::new(option.as_bytes())?;
            let string = CString::new(value.as_bytes())?;
            unsafe {
                AddIpoptStrOption(problem, name.into_raw(), string.into_raw());
            }
        }
        for (option, value) in &self.numeric_options {
            let option_name = CString::new(option.as_bytes())?;
            unsafe {
                AddIpoptNumOption(problem, option_name.into_raw(), *value);
            }
        }

        let mut variables = initial_solution.x;
        let z_l = initial_solution.z_l.unwrap_or(vec![1.0; n]);
        let z_u = initial_solution.z_u.unwrap_or(vec![1.0; n]);
        let mut g = vec![0.0; m];
        let mut lambda = initial_solution.lambda.unwrap_or(vec![1.0; m]);

        results.status = unsafe {
            IpoptSolve(
                problem,
                variables.as_mut_ptr(),
                g.as_mut_ptr(),
                &raw mut results.solution.objective,
                lambda.as_mut_ptr(),
                x_l.as_mut_ptr(),
                x_u.as_mut_ptr(),
                user_data_ptr,
            )
        };

        results.solution.x = variables;
        results.solution.constraints = g;
        results.solution.lambda = lambda;
        results.solution.z_l = z_l;
        results.solution.z_u = z_u;

        Ok(results)
    }
}
