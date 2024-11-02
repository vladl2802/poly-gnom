use std::ops::{Add, Mul, Neg};

use poly_gnom::traits::{One, PolyValues, Zero};

use super::{
    objects::{Matrix, Scalar, Vector},
    types::Types,
};

#[derive(Clone, Debug)]
pub enum Values {
    Scalar(Scalar),
    Vector(Vector),
    Matrix(Matrix),
}

impl Neg for Values {
    type Output = Values;

    fn neg(self) -> Self::Output {
        match self {
            Values::Scalar(scalar) => Values::Scalar(-scalar),
            Values::Vector(vector) => Values::Vector(-vector),
            Values::Matrix(matrix) => Values::Matrix(-matrix),
        }
    }
}

impl Add for Values {
    type Output = Option<Values>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Values::Scalar(l_val), Values::Scalar(r_val)) => Some(Values::Scalar(l_val + r_val)),
            (Values::Scalar(_), Values::Vector(_)) => None,
            (Values::Scalar(_), Values::Matrix(_)) => None,
            (Values::Vector(_), Values::Scalar(_)) => None,
            (Values::Vector(l_vec), Values::Vector(r_vec)) => (l_vec + r_vec).map(Values::Vector),
            (Values::Vector(_), Values::Matrix(_)) => None,
            (Values::Matrix(_), Values::Scalar(_)) => None,
            (Values::Matrix(_), Values::Vector(_)) => None,
            (Values::Matrix(l_mat), Values::Matrix(r_mat)) => (l_mat + r_mat).map(Values::Matrix),
        }
    }
}

impl Mul for Values {
    type Output = Option<Values>;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Values::Scalar(l_val), Values::Scalar(r_val)) => Some(Values::Scalar(l_val * r_val)),
            (Values::Scalar(l_val), Values::Vector(r_vec)) => Some(Values::Vector(l_val * r_vec)),
            (Values::Scalar(l_val), Values::Matrix(r_mat)) => Some(Values::Matrix(l_val * r_mat)),
            (Values::Vector(l_vec), Values::Scalar(r_val)) => Some(Values::Vector(l_vec * r_val)),
            (Values::Vector(_), Values::Vector(_)) => None,
            (Values::Vector(_), Values::Matrix(_)) => None,
            (Values::Matrix(l_mat), Values::Scalar(r_val)) => Some(Values::Matrix(l_mat * r_val)),
            (Values::Matrix(l_mat), Values::Vector(r_vec)) => (l_mat * r_vec).map(Values::Vector),
            (Values::Matrix(l_mat), Values::Matrix(r_mat)) => (l_mat * r_mat).map(Values::Matrix),
        }
    }
}

impl PolyValues<Types, Values> for Values {
    fn zero_with_type(expected_type: Types) -> Option<Self> {
        match expected_type {
            Types::Scalar => Some(Values::Scalar(Scalar::zero())),
            Types::Vector => None,
            Types::Matrix => None,
        }
    }

    fn one_with_type(expected_type: Types) -> Option<Self> {
        match expected_type {
            Types::Scalar => Some(Values::Scalar(Scalar::one())),
            Types::Vector => None,
            Types::Matrix => None,
        }
    }

    fn as_type(&self) -> Types {
        match self {
            Values::Scalar(_) => Types::Scalar,
            Values::Vector(_) => Types::Vector,
            Values::Matrix(_) => Types::Matrix,
        }
    }
}
