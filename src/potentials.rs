#![allow(non_snake_case)]

pub trait Potential{
	fn getEnergy(&self, r: f64) -> f64;
}

pub struct LJ {
    sigma: f64,
    epsilon: f64,
}

impl LJ {
    pub fn new(sigma: f64, epsilon: f64) -> LJ {
        LJ {
            sigma: sigma,
            epsilon: epsilon,
        }
    }
}

impl Potential for LJ{
    fn getEnergy(&self, r: f64) -> f64 {
        assert!(r >= 1e-6);
        4. * self.epsilon * ((self.sigma / r).powf(12.) - (self.sigma / r).powf(6.))
    }
}

pub struct Exp6 {
    alpha: f64,
    A: f64,
    C: f64,
}

impl Exp6 {
    pub fn new(alpha: f64, A: f64, C: f64) -> Exp6 {
        Exp6 {
            alpha: alpha,
            A: A,
            C: C,
        }
    }
}

impl Potential for Exp6{
	fn getEnergy(&self, r:f64)->f64{
		assert!(r >= 1e-6);
		self.A*(-self.alpha*r).exp() - self.C/r.powf(6.)
	}
}
