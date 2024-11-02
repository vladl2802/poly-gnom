use std::{
    fmt::{Debug, Display},
    ops::{Add, Mul, Neg},
};

pub trait Zero {
    fn zero() -> Self;
}

pub trait One {
    fn one() -> Self;
}

#[derive(PartialEq, Eq, Debug)]
pub struct MulTraits<Types> {
    pub result: Option<Types>,
    pub commutative: bool,
}

pub trait PolyTypes<Types: PolyTypes<Types>>
where
    Self: Sized + Display + Debug + Clone + Eq,
    Self: Mul<Output = MulTraits<Types>>,
    Self: Add<Output = Option<Self>>, // TODO: this is not checking commutative in any kind
    Self: Neg<Output = Self>,
{
}

pub trait PolyValues<Types: PolyTypes<Types>, Values: PolyValues<Types, Values>>
where
    Self: Sized + Display + Debug + Clone,
    Self: Mul<Output = Option<Self>>,
    Self: Add<Output = Option<Self>>,
    Self: Neg<Output = Self>,
{
    fn zero_with_type(expected_type: Types) -> Option<Self>;
    fn one_with_type(expected_type: Types) -> Option<Self>;

    fn as_type(&self) -> Types;
}
