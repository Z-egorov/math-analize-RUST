use crate::ma::options::{ Limits, Points };

/// Funtion's Trait. You can use it for creating your own function structure 
pub trait Function {
    fn f(&self, x: &f64) -> f64;
    fn derivative_f(&self, x: &f64) -> f64;
    fn solve(&self) -> Option<Vec<f64>>;
    fn derivative_solve(&self) -> Option<Vec<f64>>;
    fn newton(&self, limits: &Limits) -> Result<f64, &'static str>;
    fn limits(&self) -> Limits;
    fn trapezoid(&self, limits: &Limits) -> f64 {
        let mut points = Points { x: Vec::new(), y: Vec::new() };
        let a = limits.limit_a;
        let b = limits.limit_b;

        // if you want to change the step of calculating - change "step" variable. More accuracy on increasing. But you'll lose performance
        
        let step = 10;
        let h = (b-a) / step as f64;
        for i in 0..step+1 as usize {
            points.x.push(a + i as f64 * h);
            points.y.push(self.f(points.x.get(i).expect("Failed to get value")));
        }

        let mut y_sum: f64 = 0.0;
        for i in 0..(step as usize)-1 {
            y_sum += points.y.get(i+1).expect("Failed to get element");
        }

        let integral: f64 =
            h * (
            (points.y[0] + points.y[step as usize])
            / 2.0 + y_sum
        );

        points.x.clear();
        points.y.clear();

        integral
    }
    fn simpson(&self, limits: &Limits) -> f64 {
        let mut points = Points { x: Vec::new(), y: Vec::new() };
        let a = limits.limit_a;
        let b = limits.limit_b;
        
        // if you want to change the step of calculating - change "step" variable. More accuracy on increasing, but you'll lose performance

        let step = 10;
        let h = (b-a) / (2.0*step as f64);
        for i in 0..step*2+1 as usize {
            for j in (0..step*2+1).map(|value| value as f64 * h) {
                points.x.push(j);
            }
            points.y.push(self.f(points.x.get(i).expect("Failed to get value")));
        }

        let mut y_even_sum: f64 = 0.0;
        let mut y_odd_sum: f64 = 0.0;

        for i in (2..2*step-1).step_by(2) {
            y_even_sum += points.y.get(i).expect("Failed to get vaulue");
        }
        for i in (1..2*step).step_by(2) {
            y_odd_sum += points.y.get(i).expect("Failed to get value");
        }


        let integral: f64 = (h/3.0) * ((points.y[0] + points.y[2*step]) + 2.0 * y_even_sum + 4.0 * y_odd_sum);
        
        points.x.clear();
        points.y.clear();

        integral
    }
    fn derivative_points(&self, limits: &Limits) -> Vec<f64> {    
        let mut points = Points { x: Vec::new(), y: Vec::new() };
        let dx: f64 = 0.1;
        
        let a = limits.limit_a as usize;
        let b = limits.limit_b as usize;



        for i in a..b+1 {
            let i = f64::from(i as i32) * dx;
            points.x.push(i);
        } 
        for i in 0..b+1 {
            points.y.push((self.f(&(points.x[i]+dx)) - self.f(&points.x[i])) / dx);
            // (f(X[i]+dx)-f(X[i]))/dx
        }

        let y_points = points.y.clone();
        points.x.clear();
        points.y.clear();

        y_points
    }
    fn function_points(&self, limits: &Limits) -> Vec<f64> {
        let mut points = Points { x: Vec::new(), y: Vec::new() };
        let a = limits.limit_a as usize;
        let b = limits.limit_b as usize;
        
        for i in a..b+1 {
            points.x.push(i as f64);
            points.y.push(self.f(&points.x[i]));
        }

        let y_points = points.y.clone();
        points.x.clear();
        points.y.clear();

        y_points
    }    
    fn evenness(&self) -> &str {
        let mut _result: &str = "";

        if self.f(&(-1.0)) as i64 == self.f(&1.0) as i64 { _result = "even"; }
        else if self.f(&(-1.0)) as i64 == -(self.f(&1.0)) as i64 { _result = "odd"; }
        else { _result = "neither even nor odd"; }
    
        _result
    }
    fn calculate_point(&self, x: &f64) -> f64 {
        self.f(x)
    }
    fn calculate_derivative_point(&self, x: &f64) -> f64 {
        self.derivative_f(x)
    }
}