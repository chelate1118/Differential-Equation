use std::error::Error;

mod polynomial;

pub(super) trait Equation {
    fn value(&self, x: f64, y: f64) -> f64;
    fn from_str(from: &str) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
}

#[test]
fn test_monomial() {
    use crate::assert_feq;

    let mono1: Box<dyn Equation> = Box::new(
        polynomial::Monomial {
            coeff: 0.8,
            x_exp: 0.5,
            y_exp: 2.0
        }
    );

    assert_feq!(mono1.value(4.0, -1.0), 1.6);
    assert_feq!(mono1.value(4.0, 0.0), 0.0);

    let mono2 = polynomial::Monomial::from_str("0.8");
    let mono3 = polynomial::Monomial::from_str("0.8x");
    let mono4 = polynomial::Monomial::from_str("3x2y-3");

    assert!(matches!(mono2, Err(_)));
    assert!(matches!(mono3, Err(_)));
    assert!(matches!(mono4, Ok(_)));

    let mono4 = mono4.unwrap();

    assert_feq!(mono4.value(1.0, 1.0), 3.0);
    assert_feq!(mono4.value(2.0, -1.0), -12.0);
}

#[test]
fn test_polynomial() {
    use crate::assert_feq;

    let poly1: Box<dyn Equation> = Box::new(
        polynomial::Polynomial {
            formula: vec!(
                Box::new(polynomial::Monomial::from((0.8, 0.5, 2.0))),
                Box::new(polynomial::Monomial::from((-1.0, 1.0, 1.0))),
                Box::new(polynomial::Monomial::from((0.0, -5.0, 2.0))),
                Box::new(polynomial::Monomial::from((4.0, 0.0, 0.0))),
            )
        }
    );

    assert_feq!(poly1.value(4.0, -1.0), 9.6);
    assert_feq!(poly1.value(4.0, 0.0), 4.0);
}