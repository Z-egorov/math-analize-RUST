use crate::ma::functions::function::Function;
use crate::ma::options::CalculatingMethod;
use std::collections::HashMap;

/// Function for calculating integral. You need an enum "Calculating Method" in options module for using. You can calculate integral with 3 methods: Newton's, Trapezoid method and Simpson's method.
pub fn calculate_integral(method: CalculatingMethod, f: &impl Function) -> f64 {
    match method{
        CalculatingMethod::NEWTON => f.newton(&f.limits()).unwrap(),
        CalculatingMethod::TRAPEZOID => f.trapezoid(&f.limits()),
        CalculatingMethod::SIMPSON => f.simpson(&f.limits()),
    }
}
/// Function that returns Vector of f64. Vector containing derivative points of function 
pub fn calculate_derivative_points(f: &impl Function) -> Vec<f64> {
    f.derivative_points(&f.limits())
}
/// Function for calculating extremum points.
pub fn calculate_extremum_points(f: &impl Function) -> HashMap<&str, f64> {
    dbg!("DEBUG FUNCTION ENTRY");
    let mut extremum_points: HashMap<&str, f64> = HashMap::new();
    let mut solves: Vec<f64> = f.derivative_solve().expect("Can't calculate extremum points here");

    for i in 0..solves.len() {
        if f.calculate_derivative_point(&(solves[i] - 0.1)) < 0.0 && f.calculate_derivative_point(&(solves[i] + 0.1)) > 0.0 {
            extremum_points.insert("MINIMUM POINT", solves[i]);
        }
        else if f.calculate_derivative_point(&(solves[i] - 0.1)) > 0.0 && f.calculate_derivative_point(&(solves[i] + 0.1)) < 0.0{
            extremum_points.insert("MAXIMUM POINT", solves[i]);
        }
        else if (f.calculate_point(&(solves[i] - 0.1)) > f.calculate_point(&(solves[i]))) && (f.calculate_point(&(solves[i] + 0.1)) > f.calculate_point(&(solves[i]))) {
            extremum_points.insert("MINIMUM POINT", solves[i]);
        }
        else if (f.calculate_point(&(solves[i] - 0.1)) < f.calculate_point(&(solves[i]))) && (f.calculate_point(&(solves[i] + 0.1)) < f.calculate_point(&(solves[i]))) {
            extremum_points.insert("MAXIMUM POINT", solves[i]);
        }

    }
    solves.clear();
    extremum_points
}
/// Function for calculating extremums of a function
pub fn calculate_extremums(f: &impl Function) -> HashMap<&str, f64> {
    let extremum_points = calculate_extremum_points(f);
    let mut extremums: HashMap<&str, f64> = HashMap::new();

    let extremum_points_iter: (&&str, &f64) = extremum_points.iter().next().unwrap();

    extremums.insert(extremum_points_iter.0, f.calculate_point(extremum_points_iter.1));

    extremums
}
/// Function for calculating evennes of a function. Returns &str that says "even", "odd" or "neither even nor odd".
pub fn evenness(f: &impl Function) -> &str {
    f.evenness()
}