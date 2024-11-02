use std::ops::{Add, Mul, Neg};

use poly_gnom::traits::{MulTraits, PolyTypes};

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Types {
    Scalar,
    Vector,
    Matrix,
}

impl Neg for Types {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self
    }
}

impl Add for Types {
    type Output = Option<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Types::Scalar, Types::Scalar) => Some(Types::Scalar),
            (Types::Vector, Types::Vector) => Some(Types::Vector),
            (Types::Matrix, Types::Matrix) => Some(Types::Matrix),
            _ => None,
        }
    }
}

impl Mul for Types {
    type Output = MulTraits<Self>;

    fn mul(self, rhs: Self) -> Self::Output {
        let (result, commutative) = match (self, rhs) {
            (Types::Scalar, Types::Scalar) => (Some(Types::Scalar), true),
            (Types::Scalar, Types::Vector) => (Some(Types::Vector), true),
            (Types::Scalar, Types::Matrix) => (Some(Types::Scalar), true),
            (Types::Vector, Types::Scalar) => (Some(Types::Vector), true),
            (Types::Vector, Types::Vector) => (None, false),
            (Types::Vector, Types::Matrix) => (None, false),
            (Types::Matrix, Types::Scalar) => (Some(Types::Matrix), true),
            (Types::Matrix, Types::Vector) => (Some(Types::Matrix), false),
            (Types::Matrix, Types::Matrix) => (Some(Types::Matrix), false),
        };
        MulTraits {
            result,
            commutative,
        }
    }
}

impl PolyTypes<Types> for Types {}
