use std::{
    fmt::Display,
    ops::{Add, Mul, Neg},
};

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

impl Display for Types {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Types::Scalar => write!(f, "Scalar"),
            Types::Vector => write!(f, "Vector"),
            Types::Matrix => write!(f, "Matrix"),
        }
    }
}

impl PolyTypes<Types> for Types {}
