#![allow(clippy::integer_arithmetic, clippy::suspicious_arithmetic_impl)]

use crate::*;

use rand::Rng;
use std::ops::{Add, Div, Mul, Sub};

/// A modular number.
#[derive(Deref, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Mod<const N: Num>(pub Num);

impl<const N: Num> From<Num> for Mod<N> {
    fn from(value: Num) -> Self {
        Self(value % N)
    }
}

impl<const N: Num> Mod<N> {
    /// Generate a random number with this modulus.
    pub fn random() -> Self {
        Self(rand::thread_rng().gen_range(0..N))
    }
}

macro_rules! mod_impl {
    ($trait:ident, $fn:ident, $op:expr, $assign_trait:ident, $assign_fn:ident) => {
        impl<const N: Num> std::ops::$trait for Mod<N> {
            type Output = Self;

            fn $fn(self, rhs: Self) -> Self::Output {
                Self(($op(self.0, rhs.0, N)) % N)
            }
        }

        impl<const N: Num> std::ops::$trait<Num> for Mod<N> {
            type Output = Self;

            fn $fn(self, rhs: Num) -> Self::Output {
                self.$fn(Self(rhs))
            }
        }

        impl<const N: Num> std::ops::$assign_trait for Mod<N> {
            fn $assign_fn(&mut self, rhs: Self) {
                *self = self.$fn(rhs);
            }
        }

        impl<const N: Num> std::ops::$assign_trait<Num> for Mod<N> {
            fn $assign_fn(&mut self, rhs: Num) {
                self.$assign_fn(Self(rhs));
            }
        }
    };
}

mod_impl!(Add, add, |a, b, _| a + b, AddAssign, add_assign);
// Have to add n before subtracting to avoid potential overflow.
mod_impl!(Sub, sub, |a, b, n| a + n - b, SubAssign, sub_assign);
mod_impl!(Mul, mul, |a, b, _| a * b, MulAssign, mul_assign);
mod_impl!(Div, div, |a, b, _| a / b, DivAssign, div_assign);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plus_works() {
        let n = Mod::<12>(10);
        assert_eq!(n + 3, Mod::<12>(1));
    }

    #[test]
    fn sub_works() {
        let n = Mod::<12>(3);
        assert_eq!(n - 10, Mod::<12>(5));
    }

    #[test]
    fn times_works() {
        let n = Mod::<12>(4);
        let m = Mod::<12>(8);
        assert_eq!(n * m, Mod::<12>(8));
    }
}
