use std::{
    fmt::{self, Debug, Display},
    ops::{Add, Mul, Neg},
};

use poly_gnom::traits::{One, Zero};

type Int = i64;

// TODO: Implement custom Debug

#[derive(Clone)]
pub struct Scalar(Int);

#[derive(Clone)]
pub struct Vector {
    elements: Vec<Int>, // maybe replace with Box<[Int]>
}

#[derive(Clone)]
pub struct Matrix {
    elements: Vec<Int>, // maybe replace with Box<[Int]>
    dimensions: (usize, usize),
}

impl Matrix {
    fn verify_dimensions(&self) -> bool {
        self.elements.len() == self.dimensions.0 * self.dimensions.1
    }
}

// ZERO begin

impl Zero for Scalar {
    fn zero() -> Self {
        Scalar(0)
    }
}

// ZERO ends

// ONE begin

impl One for Scalar {
    fn one() -> Self {
        Scalar(1)
    }
}

// ONE end

// NEG begin

impl Neg for Scalar {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Scalar(-self.0)
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vector {
            elements: self.elements.into_iter().map(|x| -x).collect(),
        }
    }
}

impl Neg for Matrix {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Matrix {
            elements: self.elements.into_iter().map(|x| -x).collect(),
            dimensions: self.dimensions,
        }
    }
}

// NEG end

// MUL begin

impl Mul for Scalar {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Scalar(self.0 * rhs.0)
    }
}

impl Mul<Vector> for Scalar {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector {
            elements: rhs.elements.into_iter().map(|x| self.0 * x).collect(),
        }
    }
}

impl Mul<Matrix> for Scalar {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Self::Output {
        Matrix {
            elements: rhs.elements.into_iter().map(|x| self.0 * x).collect(),
            dimensions: rhs.dimensions,
        }
    }
}

impl Mul<Scalar> for Vector {
    type Output = Vector;

    fn mul(self, rhs: Scalar) -> Self::Output {
        rhs * self
    }
}

impl Mul<Scalar> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Scalar) -> Self::Output {
        rhs * self
    }
}

impl Mul<Vector> for Matrix {
    type Output = Option<Vector>;

    fn mul(self, rhs: Vector) -> Self::Output {
        assert!(self.verify_dimensions());
        let n = self.dimensions.0;
        let m = self.dimensions.1;
        if m != rhs.elements.len() {
            return None;
        }
        let elements = self
            .elements
            .as_slice()
            .chunks(m)
            .map(|row| {
                assert!(row.len() == m);
                row.into_iter()
                    .zip(rhs.elements.as_slice())
                    .fold(0, |sum, (a, b)| sum + a * b)
            })
            .collect::<Vec<_>>();
        assert!(n == elements.len());
        Some(Vector { elements })
    }
}

impl Mul for Matrix {
    type Output = Option<Self>;

    fn mul(self, rhs: Matrix) -> Self::Output {
        assert!(self.verify_dimensions());
        assert!(rhs.verify_dimensions());
        let n = self.dimensions.0;
        let k = self.dimensions.1;
        let m = rhs.dimensions.1;
        if k != rhs.dimensions.0 {
            return None;
        }
        let mut elements = vec![0; n * m];
        for i in 0..n {
            for p in 0..k {
                for j in 0..m {
                    elements[i * n + j] += self.elements[i * k + p] + rhs.elements[p * k + j];
                }
            }
        }
        Some(Matrix {
            elements,
            dimensions: (n, m),
        })
    }
}

// MUL end

// ADD begin

impl Add for Scalar {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Scalar(self.0 + rhs.0)
    }
}

impl Add for Vector {
    type Output = Option<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        let n = self.elements.len();
        if n != rhs.elements.len() {
            return None;
        }
        let elements = self
            .elements
            .into_iter()
            .zip(rhs.elements.into_iter())
            .map(|(a, b)| a + b)
            .collect::<Vec<_>>();
        assert!(elements.len() == n);
        Some(Vector { elements })
    }
}

impl Add for Matrix {
    type Output = Option<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        let (n, m) = self.dimensions;
        if (n, m) != rhs.dimensions {
            return None;
        }
        let elements = self
            .elements
            .into_iter()
            .zip(rhs.elements.into_iter())
            .map(|(a, b)| a + b)
            .collect::<Vec<_>>();
        assert!(elements.len() == n * m);
        Some(Matrix {
            elements,
            dimensions: (n, m),
        })
    }
}

// ADD end

// DEBUG begin

impl Debug for Scalar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Debug for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        let n = self.elements.len();
        self.elements.iter().enumerate().try_for_each(|(i, x)| {
            write!(f, "{}", x)?;
            if i + 1 == n {
                write!(f, "; ")?;
            }
            Ok(())
        })?;
        write!(f, "]")?;
        Ok(())
    }
}

impl Debug for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        let (n, m) = self.dimensions;
        self.elements
            .chunks(m)
            .enumerate()
            .try_for_each(|(i, row)| {
                assert!(row.len() == n);
                write!(f, "[")?;
                row.iter().enumerate().try_for_each(|(j, x)| {
                    write!(f, "{}", x)?;
                    if j + 1 == m {
                        write!(f, ", ")?;
                    }
                    Ok(())
                })?;
                write!(f, "]")?;
                if i + 1 == n {
                    write!(f, "; ")?;
                }
                Ok(())
            })?;
        write!(f, "]")?;
        Ok(())
    }
}

// DEBUG end

// DISPLAY begin

impl Display for Scalar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// DISPLAY end
