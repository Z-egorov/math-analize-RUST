pub mod function;
use function::Function;
use crate::ma::options::Limits;

/// Linear function structure
pub struct Linear {
    k: f64,
    b: f64,
    
    limits: Limits,
}

/// Quadratic function structure
pub struct Quadratic { 
    a: f64,
    b: f64,
    c: f64,
    
    limits: Limits,
}

/// Power function structure
pub struct Power {
    n: f64,

    limits: Limits,
}

/// Fractional-Linear function structure
pub struct FractionalLinear {
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    
    limits: Limits,
}

/// Exponential function structure
pub struct Exponential {
    a: f64,
    
    limits: Limits,
}
// implementations

// Linear Function

impl Function for Linear {
    fn newton(&self, limits: &Limits) -> Result<f64, &'static str> {
        let integral: f64 = (self.k*(limits.limit_b.powf(2.0)/2.0) + self.b*limits.limit_b) - (self.k * (limits.limit_a.powf(2.0)/2.0) - self.b*limits.limit_a);
        Ok(integral)
    }
    fn solve(&self) -> Option<Vec<f64>> {
        let mut solves: Vec<f64> = Vec::new(); 
        solves.push(self.k/self.b);
        Some(solves)
    }
    fn derivative_solve(&self) -> Option<Vec<f64>> {
        None
    }
    fn f(&self, x: &f64) -> f64 {
        self.k*x+self.b
    }
    fn derivative_f(&self, _x: &f64) -> f64 {
        self.k
    }
    fn limits(&self) -> Limits {
        self.limits
    }
}
impl Linear {
    /// To create an object of function you need to use "new" method. After passing all the coefficients of the function, you need to pass the limits on which this function will exist
    pub fn new(k: f64, b: f64, limit_a: f64, limit_b: f64) -> Linear {
        Linear {
            k,
            b,
            limits: Limits {
                limit_a,
                limit_b,
            },
        }
    }
    /// Function "set_limits" is used to change the limits on which your function will exist
    pub fn set_limits(&mut self, limit_a: f64, limit_b: f64) {
        self.limits = Limits {
            limit_a,
            limit_b,
        };
    }
}

// Quadratic Function

impl Function for Quadratic {
    fn newton(&self, limits: &Limits) -> Result<f64, &'static str> {
        let integral: f64 = (self.a * (limits.limit_b.powf(3.0) / 3.0) + self.b * (limits.limit_b.powf(2.0)/2.0) + self.c * limits.limit_b) - (self.a * (limits.limit_a.powf(3.0) / 3.0) + self.b * (limits.limit_a.powf(2.0)/2.0) + self.c * limits.limit_a);
        Ok(integral)
    }
    fn solve(&self) -> Option<Vec<f64>> {
        let mut solves: Vec<f64> = Vec::new(); 
        let d: f64 = self.b.powf(2.0) - 4.0*self.a*self.c;
        if d < 0.0 { None }
        else if d == 0.0 {
            solves.push((self.b*-1.0 + d.sqrt()) / 2.0*self.a);
            Some(solves.clone())
        }
        else {
            solves.push((self.b*-1.0 + d.sqrt()) / 2.0*self.a);
            solves.push((self.b*-1.0 - d.sqrt()) / 2.0*self.a);
            Some(solves.clone())
        }
    }
    fn derivative_solve(&self) -> Option<Vec<f64>> {
        let mut derivative_solve: Vec<f64> = Vec::new();
        derivative_solve.push(-self.b/2.0*self.a);
        Some(derivative_solve.clone())
    }
    fn f(&self, x: &f64) -> f64 {
        self.a * x.powf(2.0) + self.b*x + self.c
    }
    fn derivative_f(&self, _x: &f64) -> f64 {
        2.0*self.a + self.b
    }
    fn limits(&self) -> Limits {
        self.limits
    }
}
impl Quadratic {
    pub fn new(a: f64, b: f64, c: f64, limit_a: f64, limit_b: f64) -> Quadratic {
        Quadratic {
            a,
            b,
            c,
            limits: Limits {
                limit_a,
                limit_b,
            },
        }
    }
    pub fn set_limits(&mut self, limit_a: f64, limit_b: f64) {
        self.limits = Limits {
            limit_a,
            limit_b,
        };
    }
}

// Power Function

impl Function for Power {
    fn newton(&self, limits: &Limits) -> Result<f64, &'static str> {
        let integral: f64 = (limits.limit_b.powf(self.n+1.0)) / self.n + 1.0;
        Ok(integral)
    }
    fn solve(&self) -> Option<Vec<f64>> {
        Some(vec![0.0])
    }
    fn derivative_solve(&self) -> Option<Vec<f64>> {
        Some(vec![0.0])
    }
    fn f(&self, x: &f64) -> f64 {
        x.powf(self.n)
    }

    fn derivative_f(&self, x: &f64) -> f64 {
        self.n * x.powf(self.n-1.0)
    }
    fn limits(&self) -> Limits {
        self.limits
    }
}
impl Power {
    pub fn new(n: f64, limit_a: f64, limit_b: f64) -> Power {
        Power {
            n,
            limits: Limits {
                limit_a,
                limit_b,
            },
        }
    }
    pub fn set_limits(&mut self, limit_a: f64, limit_b: f64) {
        self.limits = Limits {
            limit_a,
            limit_b,
        };
    }
}

// Fractional-Linear Function

impl Function for FractionalLinear {
    fn newton(&self, _limits: &Limits) -> Result<f64, &'static str> {
        Err("Integral of Fractional-Linear function Can not be calculated with Newton's method. Try another one")
    }
    fn solve(&self) -> Option<Vec<f64>> {
        let mut solves: Vec<f64> = Vec::new();
        solves.push(self.b / self.a);
        Some(solves.clone())
    }
    fn derivative_solve(&self) -> Option<Vec<f64>> {
        Some(vec![0.0])
    }
    fn f(&self, x: &f64) -> f64 {
        (self.a * x + self.b) / (self.c *x + self.d)
    }
    fn derivative_f(&self, x: &f64) -> f64 {
        (self.a * self.d - self.b * self.c) / ((self.c * x).powf(2.0))
    }
    fn limits(&self) -> Limits {
        self.limits
    }
}
impl FractionalLinear {
    pub fn new(a: f64, b:f64, c:f64, d:f64, limit_a: f64, limit_b: f64) -> Result<FractionalLinear, &'static str> {
        if (c == 0.0) && (d == 0.0) { return Err("Error") }
        Ok(FractionalLinear {
            a,
            b,
            c,
            d,
            limits: Limits {
                limit_a,
                limit_b,
            },
        }) 
    }
    pub fn set_limits(&mut self, limit_a: f64, limit_b: f64) {
        self.limits = Limits {
            limit_a,
            limit_b,
        };
    }
}

// Exponential Function

impl Function for Exponential {
    fn newton(&self, limits: &Limits) -> Result<f64, &'static str> {
        let integral: f64 = (1.0/(self.a.ln()) * self.a.powf(limits.limit_b)) - (1.0/(self.a.ln()) * self.a.powf(limits.limit_a));
        Ok(integral) 
    }
    fn solve(&self) -> Option<Vec<f64>> {
        None
    }
    fn derivative_solve(&self) -> Option<Vec<f64>> {
        None
    }
    fn f(&self, x: &f64) -> f64 {
        self.a.powf(*x)
    }
    fn derivative_f(&self, x: &f64) -> f64 {
        self.a.powf(*x)
    }
    fn limits(&self) -> Limits {
        self.limits
    }
}
impl Exponential {
    pub fn new(a: f64, limit_a: f64, limit_b: f64) -> Exponential {
        Exponential {
            a,
            limits: Limits {
                limit_a,
                limit_b,
            },
        }
    }
    pub fn set_limits(&mut self, limit_a: f64, limit_b: f64) {
        self.limits = Limits {
            limit_a,
            limit_b,
        };
    }
}
