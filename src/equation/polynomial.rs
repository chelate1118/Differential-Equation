use super::Equation;

pub(super) struct Monomial {
    pub(super) coeff: f64,
    pub(super) x_exp: f64,
    pub(super) y_exp: f64,
}

impl Equation for Monomial {
    fn value(&self, x: f64, y: f64) -> f64 {
        self.coeff * x.powf(self.x_exp) * y.powf(self.y_exp)
    }
}

impl From<(f64, f64, f64)> for Monomial {
    fn from(tuple: (f64, f64, f64)) -> Self {
        Monomial { coeff: tuple.0, x_exp: tuple.1, y_exp: tuple.2 }
    }
}

pub(super) struct Polynomial {
    pub(super) formula: Vec<Monomial>
}

impl Equation for Polynomial {
    fn value(&self, x: f64, y: f64) -> f64 {
        self.formula
            .iter()
            .map(|mono| mono.value(x, y))
            .sum()
    }
}