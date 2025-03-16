use ipopt_bindgen::{Application, InitialSolution, ProblemSize, Tnlp};

struct HS071;

const N: usize = 4;
const M: usize = 2;
const NNZ_JAC: usize = N * M;
const NNZ_HESS: usize = 10;

impl Tnlp for HS071 {
    fn get_nlp_info(&self) -> ProblemSize {
        ProblemSize {
            n: i32::try_from(N).unwrap(),
            m: i32::try_from(M).unwrap(),
            nnz_jac: i32::try_from(NNZ_JAC).unwrap(),
            nnz_hess: i32::try_from(NNZ_HESS).unwrap(),
        }
    }

    fn get_bounds_info(&self, x_l: &mut [f64], x_u: &mut [f64], g_l: &mut [f64], g_u: &mut [f64]) {
        x_l.fill(1.0);
        x_u.fill(5.0);
        g_l[0] = 25.0;
        g_u[0] = 2e19;
        g_l[1] = 40.0;
        g_u[1] = 40.0;
    }

    fn get_starting_point(&self) -> InitialSolution {
        InitialSolution::from_variables(vec![1.0, 5.0, 5.0, 1.0])
    }

    fn eval_f(&mut self, x: &[f64], obj_value: &mut f64) -> bool {
        *obj_value = x[0] * x[3] * (x[0] + x[1] + x[2]) + x[2];
        true
    }

    fn eval_grad_f(&mut self, x: &[f64], grad_f: &mut [f64]) -> bool {
        grad_f[0] = x[0] * x[3] + x[3] * (x[0] + x[1] + x[2]);
        grad_f[1] = x[0] * x[3];
        grad_f[2] = x[0] * x[3] + 1.0;
        grad_f[3] = x[0] * (x[0] + x[1] + x[2]);
        true
    }

    fn eval_g(&mut self, x: &[f64], g: &mut [f64]) -> bool {
        g[0] = x[0] * x[1] * x[2] * x[3];
        g[1] = x[0] * x[0] + x[1] * x[1] + x[2] * x[2] + x[3] * x[3];
        true
    }

    fn get_jacobian_sparsity(&mut self, _n: i32, _m: i32, i_row: &mut [i32], j_col: &mut [i32]) {
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
    }

    fn eval_jac_g(&mut self, x: &[f64], _m: i32, values: &mut [f64]) -> bool {
        values[0] = x[1] * x[2] * x[3];
        values[1] = x[0] * x[2] * x[3];
        values[2] = x[0] * x[1] * x[3];
        values[3] = x[0] * x[1] * x[2];
        values[4] = 2.0 * x[0];
        values[5] = 2.0 * x[1];
        values[6] = 2.0 * x[2];
        values[7] = 2.0 * x[3];
        true
    }

    fn get_hessian_sparsity(&mut self, n: i32, _m: i32, i_row: &mut [i32], j_col: &mut [i32]) {
        let mut idx = 0;
        for row in 0..n {
            for col in 0..=row {
                i_row[idx] = row;
                j_col[idx] = col;
                idx += 1;
            }
        }
    }

    fn eval_h(
        &mut self,
        x: &[f64],
        obj_factor: f64,
        lambda: &[f64],
        _m: i32,
        values: &mut [f64],
    ) -> bool {
        values[0] = obj_factor * (2.0 * x[3]);

        values[1] = obj_factor * (x[3]);
        values[2] = 0.;

        values[3] = obj_factor * (x[3]);
        values[4] = 0.;
        values[5] = 0.;

        values[6] = obj_factor * (2.0 * x[0] + x[1] + x[2]);
        values[7] = obj_factor * (x[0]);
        values[8] = obj_factor * (x[0]);
        values[9] = 0.;

        // add the portion for the first constraint
        values[1] += lambda[0] * (x[2] * x[3]);

        values[3] += lambda[0] * (x[1] * x[3]);
        values[4] += lambda[0] * (x[0] * x[3]);

        values[6] += lambda[0] * (x[1] * x[2]);
        values[7] += lambda[0] * (x[0] * x[2]);
        values[8] += lambda[0] * (x[0] * x[1]);

        // add the portion for the second constraint
        values[0] += lambda[1] * 2.0;

        values[2] += lambda[1] * 2.0;

        values[5] += lambda[1] * 2.0;

        values[9] += lambda[1] * 2.0;

        true
    }
}

fn main() {
    let mut application = Application::new();
    application
        .set_string_option("linear_solver", "mumps")
        .set_string_option("mu_strategy", "adaptive")
        .set_numeric_option("tol", 3.82e-6);
    let hs071 = HS071;
    let _results = application.optimize_tnlp(hs071);
}
