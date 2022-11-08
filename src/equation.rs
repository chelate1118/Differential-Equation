mod polynomial;

pub(super) trait Equation {
    fn value(&self, x: f64, y: f64) -> f64;
}

#[test]
fn test_monomial() {
    use crate::assert_feq;

    let mono: Box<dyn Equation> = Box::new(
        polynomial::Monomial {
            coeff: 0.8,
            x_exp: 0.5,
            y_exp: 2.0
        }
    );

    assert_feq!(mono.value(4.0, -1.0), 1.6);
    assert_feq!(mono.value(4.0, 0.0), 0.0);
}

#[test]
fn test_polynomial() {
    use crate::assert_feq;

    let poly: Box<dyn Equation> = Box::new(
        polynomial::Polynomial {
            formula: vec!(
                polynomial::Monomial::from((0.8, 0.5, 2.0)),
                polynomial::Monomial::from((-1.0, 1.0, 1.0)),
                polynomial::Monomial::from((0.0, -5.0, 2.0)),
                polynomial::Monomial::from((4.0, 0.0, 0.0)),
            )
        }
    );

    assert_feq!(poly.value(4.0, -1.0), 9.6);
    assert_feq!(poly.value(4.0, 0.0), 4.0);
}