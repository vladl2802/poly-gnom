// this file is most of all boiler plate and therefore most likely can be geenrated with some macroses
// but this also requires some time to implement

use std::{
    fmt::{self, Debug, Display},
    ops::{Add, Mul, Neg},
};

use poly_gnom::traits::{One, PolyValues, Zero};

use super::{
    objects::{Int, Matrix, Scalar, Vector},
    types::Types,
};

#[derive(Clone, PartialEq, Eq)]
pub enum Values {
    Scalar(Scalar),
    Vector(Vector),
    Matrix(Matrix),
}

impl Values {
    pub fn new_scalar(value: Int) -> Self {
        Values::Scalar(Scalar::new(value))
    }

    pub fn new_vector(elements: Vec<Int>) -> Self {
        Values::Vector(Vector::new(elements))
    }

    pub fn new_matrix(elements: Vec<Vec<Int>>) -> Self {
        Values::Matrix(Matrix::new(elements))
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

impl Debug for Values {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Values::Scalar(scalar) => write!(f, "scalar {{ {} }}", scalar),
            Values::Vector(vector) => write!(f, "vector {{ {} }}", vector),
            Values::Matrix(matrix) => write!(f, "matrix {{ {} }}", matrix),
        }
    }
}

impl Display for Values {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
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
