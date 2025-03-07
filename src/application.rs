//! # Ipopt Bindgen Application
//!
//! This module defines an idiomatic Rust emulation of the `Ipopt::Application` C++ type via the C
//! interface - supporting functionality like configuring the optimizer and optimizing problems.

use std::collections::HashMap;

/// The main application type for making calls to Ipopt.
#[derive(Debug, Clone, Default)]
pub struct Application {
    int_options: HashMap<String, i32>,
    numeric_options: HashMap<String, f64>,
    string_options: HashMap<String, String>,
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
    pub fn new() -> Self {
        Application {
            int_options: HashMap::<String, i32>::new(),
            numeric_options: HashMap::<String, f64>::new(),
            string_options: HashMap::<String, String>::new(),
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
    pub fn set_integer_option(&mut self, key: impl Into<String>, value: impl Into<i32>) -> &mut Self {
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
    pub fn set_numeric_option(&mut self, key: impl Into<String>, value: impl Into<f64>) -> &mut Self {
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
    pub fn set_string_option(&mut self, key: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.string_options.insert(key.into(), value.into());
        self
    }
}
