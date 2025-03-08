//! HS071 example per the [Ipopt documentation](https://coin-or.github.io/Ipopt/INTERFACES.html).
//!
//! This example shows how it is possible to directly use the C API bindings. It is recommended to
//! ise the `Tnlp` trait and `Application` struct for a more idiomatic Rust interface.

use ipopt_bindgen::c_interface::*;

extern "C" fn objective_callback(
    n: ipindex,
    x: *mut ipnumber,
    _new_x: bool,
    obj_value: *mut ipnumber,
    _user_data: UserDataPtr,
) -> bool {
    let x_slice = unsafe { std::slice::from_raw_parts(x, n as usize) };
    unsafe {
        *obj_value = x_slice[0] * x_slice[3] * (x_slice[0] + x_slice[1] + x_slice[2]) + x_slice[2];
    }
    true
}

extern "C" fn constraints_callback(
    n: ipindex,
    x: *mut ipnumber,
    _new_x: bool,
    m: ipindex,
    g: *mut ipnumber,
    _user_data: UserDataPtr,
) -> bool {
    let x_slice = unsafe { std::slice::from_raw_parts(x, n as usize) };
    let g_slice = unsafe { std::slice::from_raw_parts_mut(g, m as usize) };
    g_slice[0] = x_slice[0] * x_slice[1] * x_slice[2] * x_slice[3];
    g_slice[1] = x_slice[0] * x_slice[0]
        + x_slice[1] * x_slice[1]
        + x_slice[2] * x_slice[2]
        + x_slice[3] * x_slice[3];
    true
}

extern "C" fn gradient_callback(
    n: ipindex,
    x: *mut ipnumber,
    _new_x: bool,
    grad_f: *mut ipnumber,
    _user_data: UserDataPtr,
) -> bool {
    let x_slice = unsafe { std::slice::from_raw_parts(x, n as usize) };
    let grad_slice = unsafe { std::slice::from_raw_parts_mut(grad_f, n as usize) };
    grad_slice[0] = x_slice[0] * x_slice[3] + x_slice[3] * (x_slice[0] + x_slice[1] + x_slice[2]);
    grad_slice[1] = x_slice[0] * x_slice[3];
    grad_slice[2] = x_slice[0] * x_slice[3] + 1.0;
    grad_slice[3] = x_slice[0] * (x_slice[0] + x_slice[1] + x_slice[2]);
    true
}

extern "C" fn jacobian_callback(
    n: ipindex,
    x: *mut ipnumber,
    _new_x: bool,
    _m: ipindex,
    nele_jac: ipindex,
    i_row: *mut ipindex,
    j_col: *mut ipindex,
    values: *mut ipnumber,
    _user_data: UserDataPtr,
) -> bool {
    if x.is_null() {
        // Set Jacobian sparsity structure
        let i_row = unsafe { std::slice::from_raw_parts_mut(i_row, nele_jac as usize) };
        let j_col = unsafe { std::slice::from_raw_parts_mut(j_col, nele_jac as usize) };
        // this particular Jacobian is dense
        i_row[0] = 0;
        j_col[0] = 0;
        i_row[1] = 0;
        j_col[1] = 1;
        i_row[2] = 0;
        j_col[2] = 2;
        i_row[3] = 0;
        j_col[3] = 3;
        i_row[4] = 1;
        j_col[4] = 0;
        i_row[5] = 1;
        j_col[5] = 1;
        i_row[6] = 1;
        j_col[6] = 2;
        i_row[7] = 1;
        j_col[7] = 3;
    } else {
        // Set Jacobian values
        let x_slice = unsafe { std::slice::from_raw_parts(x, n as usize) };
        let values = unsafe { std::slice::from_raw_parts_mut(values, nele_jac as usize) };
        values[0] = x_slice[1] * x_slice[2] * x_slice[3]; // 0,0
        values[1] = x_slice[0] * x_slice[2] * x_slice[3]; // 0,1
        values[2] = x_slice[0] * x_slice[1] * x_slice[3]; // 0,2
        values[3] = x_slice[0] * x_slice[1] * x_slice[2]; // 0,3

        values[4] = 2.0 * x_slice[0]; // 1,0
        values[5] = 2.0 * x_slice[1]; // 1,1
        values[6] = 2.0 * x_slice[2]; // 1,2
        values[7] = 2.0 * x_slice[3]; // 1,3
    }
    true
}

extern "C" fn hessian_callback(
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
    _user_data: UserDataPtr,
) -> bool {
    if x.is_null() {
        // Set the Hessian sparsity structure
        let i_row_slice = unsafe { std::slice::from_raw_parts_mut(i_row, nele_hess as usize) };
        let j_col_slice = unsafe { std::slice::from_raw_parts_mut(j_col, nele_hess as usize) };
        let mut idx: ipindex = 0;
        for row in 0..n {
            for col in 0..=row {
                i_row_slice[idx as usize] = row;
                j_col_slice[idx as usize] = col;
                idx += 1;
            }
        }
    } else {
        // Set the Hessian values
        let x_slice = unsafe { std::slice::from_raw_parts(x, n as usize) };
        let lambda_slice = unsafe { std::slice::from_raw_parts(lambda, m as usize) };
        let hessian = unsafe { std::slice::from_raw_parts_mut(values, nele_hess as usize) };

        // return the values. This is a symmetric matrix, fill the lower left
        // triangle only

        // fill the objective portion
        hessian[0] = obj_factor * (2.0 * x_slice[3]); // 0,0

        hessian[1] = obj_factor * (x_slice[3]); // 1,0
        hessian[2] = 0.; // 1,1

        hessian[3] = obj_factor * (x_slice[3]); // 2,0
        hessian[4] = 0.; // 2,1
        hessian[5] = 0.; // 2,2

        hessian[6] = obj_factor * (2.0 * x_slice[0] + x_slice[1] + x_slice[2]); // 3,0
        hessian[7] = obj_factor * (x_slice[0]); // 3,1
        hessian[8] = obj_factor * (x_slice[0]); // 3,2
        hessian[9] = 0.; // 3,3

        // add the portion for the first constraint
        hessian[1] += lambda_slice[0] * (x_slice[2] * x_slice[3]); // 1,0
        hessian[3] += lambda_slice[0] * (x_slice[1] * x_slice[3]); // 2,0
        hessian[4] += lambda_slice[0] * (x_slice[0] * x_slice[3]); // 2,1
        hessian[6] += lambda_slice[0] * (x_slice[1] * x_slice[2]); // 3,0
        hessian[7] += lambda_slice[0] * (x_slice[0] * x_slice[2]); // 3,1
        hessian[8] += lambda_slice[0] * (x_slice[0] * x_slice[1]); // 3,2

        // add the portion for the second constraint
        hessian[0] += lambda_slice[1] * 2.0; // 0,0
        hessian[2] += lambda_slice[1] * 2.0; // 1,1
        hessian[5] += lambda_slice[1] * 2.0; // 2,2
        hessian[9] += lambda_slice[1] * 2.0; // 3,3
    }
    true
}

fn main() {
    let n = 4;
    let m = 2;
    let mut x_l = vec![1.0, 1.0, 1.0, 1.0];
    let mut x_u = vec![5.0, 5.0, 5.0, 5.0];
    let mut g_l = vec![25.0, 40.0];
    let mut g_u = vec![2.0e19, 40.0];
    let nnz_jacobian = 8;
    let nnz_hessian = 10;
    const C_STYLE_INDEXING: i32 = 0;

    let objective: Eval_F_CB = Some(objective_callback);
    let constraints: Eval_G_CB = Some(constraints_callback);
    let gradients: Eval_Grad_F_CB = Some(gradient_callback);
    let jacobian: Eval_Jac_G_CB = Some(jacobian_callback);
    let hessian: Eval_H_CB = Some(hessian_callback);

    // Helper to make dealing with raw C string less miserable.
    let cstr = |s: &str| std::ffi::CString::new(s).unwrap().into_raw();

    unsafe {
        let problem = CreateIpoptProblem(
            n,
            x_l.as_mut_ptr(),
            x_u.as_mut_ptr(),
            m,
            g_l.as_mut_ptr(),
            g_u.as_mut_ptr(),
            nnz_jacobian,
            nnz_hessian,
            C_STYLE_INDEXING,
            objective,
            constraints,
            gradients,
            jacobian,
            hessian,
        );

        AddIpoptNumOption(problem, cstr("tol"), 3.82e-6);
        AddIpoptStrOption(problem, cstr("mu_strategy"), cstr("adaptive"));
        AddIpoptStrOption(problem, cstr("output_file"), cstr("ipopt.out"));

        let mut variables = vec![1.0, 5.0, 5.0, 1.0];
        let mut constraint_multipliers = vec![0.0, 0.0];
        let mut lower_bound_multipliers = vec![0.0, 0.0, 0.0, 0.0];
        let mut upper_bound_multipliers = vec![0.0, 0.0, 0.0, 0.0];

        let mut objective_value: f64 = 0.0;
        let _status = IpoptSolve(
            problem,
            variables.as_mut_ptr(),
            std::ptr::null_mut(),
            std::ptr::addr_of_mut!(objective_value),
            constraint_multipliers.as_mut_ptr(),
            lower_bound_multipliers.as_mut_ptr(),
            upper_bound_multipliers.as_mut_ptr(),
            std::ptr::null_mut(),
        );
    }
}
