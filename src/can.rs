use lazy_static::lazy_static;
use num_rational::Rational;

lazy_static! {
    pub static ref EMPTY: Rational = Rational::new(0, 1);
    pub static ref FULL: Rational = Rational::new(1, 1);
}

pub fn is_empty(can: &Rational) -> bool {
    *can == *EMPTY
}
