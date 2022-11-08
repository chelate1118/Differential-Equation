use std::error::Error;
use std::io::Error as IOError;
use std::io::ErrorKind;

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

    fn from_str(from: &str) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized,
    {
        if let Some((split_x1, split_x2)) = from.split_once('x') {
            if let Some((split_y1, split_y2)) = split_x2.split_once('y') {
                Ok(Monomial {
                    coeff: split_x1.parse()?,
                    x_exp: split_y1.parse()?,
                    y_exp: split_y2.parse()?,
                })
            } else {
                Err(Box::new(IOError::new(ErrorKind::InvalidInput, "no y")))
            }
        } else {
            Err(Box::new(IOError::new(ErrorKind::InvalidInput, "no x")))
        }
    }
}

impl From<(f64, f64, f64)> for Monomial {
    fn from(tuple: (f64, f64, f64)) -> Self {
        Monomial {
            coeff: tuple.0,
            x_exp: tuple.1,
            y_exp: tuple.2,
        }
    }
}

pub(super) struct Polynomial {
    pub(super) formula: Vec<Box<dyn Equation>>,
}

impl Equation for Polynomial {
    fn value(&self, x: f64, y: f64) -> f64 {
        self.formula.iter().map(|mono| mono.value(x, y)).sum()
    }

    fn from_str(from: &str) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized,
    {
        todo!()
    }
}
