use std::{
    fmt::{self, Debug, Display},
    marker::PhantomData,
};

use crate::{
    error::FinalizeError,
    symbol::Symbol,
    term::Term,
    traits::{PolyTypes, PolyValues},
};

pub trait Factorable<Values, Types>
where
    Self: Substitutiable<Values, Types, Output = Factor<Values, Types>>,
    Self: Finalizable<Values, Types>,
    Types: PolyTypes<Types>,
    Values: PolyValues<Types, Values>,
{
    fn as_factor(self) -> Factor<Values, Types>;
}

pub trait Substitutiable<Values, Types> {
    type Output;

    fn substitute(self, to: Symbol<Types>, factor: Factor<Values, Types>) -> Self::Output;
}

pub trait Finalizable<Values, Types> {
    fn finalize_type(&self) -> Result<Types, FinalizeError>;
    fn finalize_value(self) -> Result<Values, FinalizeError>;
}

#[derive(Clone)]
pub enum Factor<Values, Types> {
    Value(Value<Values, Types>),
    Variable(Variable<Types>),
    SubPoly(SubPoly<Values, Types>),
}

impl<Values, Types> Factorable<Values, Types> for Factor<Values, Types>
where
    Types: PolyTypes<Types>,
    Values: PolyValues<Types, Values>,
{
    fn as_factor(self) -> Factor<Values, Types> {
        self
    }
}

impl<Values, Types> Substitutiable<Values, Types> for Factor<Values, Types>
where
    Self: Factorable<Values, Types>,
    Types: PolyTypes<Types>,
    Values: PolyValues<Types, Values>,
{
    type Output = Self;

    fn substitute(self, to: Symbol<Types>, factor: Factor<Values, Types>) -> Self::Output {
        match self {
            Factor::Value(value) => value.substitute(to, factor),
            Factor::Variable(variable) => variable.substitute(to, factor),
            Factor::SubPoly(sub_poly) => sub_poly.substitute(to, factor),
        }
    }
}

impl<Values, Types> Finalizable<Values, Types> for Factor<Values, Types>
where
    Self: Factorable<Values, Types>,
    Types: PolyTypes<Types>,
    Values: PolyValues<Types, Values>,
{
    fn finalize_type(&self) -> Result<Types, FinalizeError> {
        match self {
            Factor::Value(value) => value.finalize_type(),
            Factor::Variable(variable) => Finalizable::<Values, Types>::finalize_type(variable),
            Factor::SubPoly(sub_poly) => sub_poly.finalize_type(),
        }
    }

    fn finalize_value(self) -> Result<Values, FinalizeError> {
        match self {
            Factor::Value(value) => Finalizable::<Values, Types>::finalize_value(value),
            Factor::Variable(variable) => Finalizable::<Values, Types>::finalize_value(variable),
            Factor::SubPoly(sub_poly) => sub_poly.finalize_value(),
        }
    }
}

impl<Values, Types> Debug for Factor<Values, Types>
where
    Types: PolyTypes<Types>,
    Values: PolyValues<Types, Values>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Value(value) => write!(f, "{:?}", value),
            Self::Variable(variable) => write!(f, "{:?}", variable),
            Self::SubPoly(sub_poly) => write!(f, "{:?}", sub_poly),
        }
    }
}

impl<Values, Types> Display for Factor<Values, Types>
where
    Types: PolyTypes<Types>,
    Values: PolyValues<Types, Values>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Factor::Value(value) => write!(f, "{}", value),
            Factor::Variable(variable) => write!(f, "{}", variable),
            Factor::SubPoly(sub_poly) => write!(f, "( {} )", sub_poly),
        }
    }
}

#[derive(Clone)]
pub struct Value<Values, Types> {
    value: Values,
    _marker: PhantomData<Types>, // I guess this is only way to avoid 'unconstrained generic parameter'
}

impl<Values, Types> Value<Values, Types> {
    pub fn new(value: Values) -> Self {
        Value {
            value,
            _marker: PhantomData,
        }
    }
}

impl<Values, Types> Factorable<Values, Types> for Value<Values, Types>
where
    Types: PolyTypes<Types>,
    Values: PolyValues<Types, Values>,
{
    fn as_factor(self) -> Factor<Values, Types> {
        Factor::Value(self)
    }
}

impl<Values, Types> Substitutiable<Values, Types> for Value<Values, Types>
where
    Self: Factorable<Values, Types>,
    Types: PolyTypes<Types>,
    Values: PolyValues<Types, Values>,
{
    type Output = Factor<Values, Types>;

    fn substitute(self, _: Symbol<Types>, _: Factor<Values, Types>) -> Self::Output {
        self.as_factor()
    }
}

impl<Values, Types> Finalizable<Values, Types> for Value<Values, Types>
where
    Self: Factorable<Values, Types>,
    Types: PolyTypes<Types>,
    Values: PolyValues<Types, Values>,
{
    fn finalize_type(&self) -> Result<Types, FinalizeError> {
        Ok(self.value.as_type())
    }

    fn finalize_value(self) -> Result<Values, FinalizeError> {
        Ok(self.value)
    }
}

impl<Values, Types> Debug for Value<Values, Types>
where
    Types: PolyTypes<Types>,
    Values: PolyValues<Types, Values>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "( {:?} | {:?} )", self.value, self.value.as_type())
    }
}

impl<Values, Types> Display for Value<Values, Types>
where
    Types: PolyTypes<Types>,
    Values: PolyValues<Types, Values>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Clone)]
pub struct Variable<Types> {
    symbol: Symbol<Types>,
}

impl<Types> Variable<Types> {
    pub fn new(symbol: Symbol<Types>) -> Self {
        Variable { symbol }
    }
}

impl<Values, Types> Factorable<Values, Types> for Variable<Types>
where
    Types: PolyTypes<Types>,
    Values: PolyValues<Types, Values>,
{
    fn as_factor(self) -> Factor<Values, Types> {
        Factor::Variable(self)
    }
}

impl<Values, Types> Substitutiable<Values, Types> for Variable<Types>
where
    Self: Factorable<Values, Types>,
    Types: PolyTypes<Types>,
    Values: PolyValues<Types, Values>,
{
    type Output = Factor<Values, Types>;

    fn substitute(self, to: Symbol<Types>, factor: Factor<Values, Types>) -> Self::Output {
        if self.symbol == to {
            factor
        } else {
            self.as_factor()
        }
    }
}

impl<Values, Types> Finalizable<Values, Types> for Variable<Types>
where
    Self: Factorable<Values, Types>,
    Types: PolyTypes<Types>,
    Values: PolyValues<Types, Values>,
{
    fn finalize_type(&self) -> Result<Types, FinalizeError> {
        self.symbol
            .associated_type
            .clone()
            .ok_or(FinalizeError::NoTypeToFinalize)
    }

    fn finalize_value(self) -> Result<Values, FinalizeError> {
        Err(FinalizeError::NoValueToFinalize)
    }
}

impl<Types> Debug for Variable<Types>
where
    Types: PolyTypes<Types>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.symbol)
    }
}

impl<Types> Display for Variable<Types>
where
    Types: PolyTypes<Types>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol)
    }
}

#[derive(Clone)]
pub struct SubPoly<Values, Types> {
    parts: Vec<Term<Values, Types>>,
}

impl<Values, Types> SubPoly<Values, Types> {
    pub fn new(parts: Vec<Term<Values, Types>>) -> Self {
        SubPoly { parts }
    }
}

impl<Values, Types> Factorable<Values, Types> for SubPoly<Values, Types>
where
    Types: PolyTypes<Types>,
    Values: PolyValues<Types, Values>,
{
    fn as_factor(self) -> Factor<Values, Types> {
        Factor::SubPoly(self)
    }
}

impl<Values, Types> Substitutiable<Values, Types> for SubPoly<Values, Types>
where
    Self: Factorable<Values, Types>,
    Types: PolyTypes<Types>,
    Values: PolyValues<Types, Values>,
{
    type Output = Factor<Values, Types>;

    fn substitute(self, to: Symbol<Types>, factor: Factor<Values, Types>) -> Self::Output {
        SubPoly {
            parts: self
                .parts
                .into_iter()
                .map(|term| term.substitute(to.clone(), factor.clone()))
                .collect(),
        }
        .as_factor()
    }
}

impl<Values, Types> Finalizable<Values, Types> for SubPoly<Values, Types>
where
    Self: Factorable<Values, Types>,
    Types: PolyTypes<Types>,
    Values: PolyValues<Types, Values>,
{
    fn finalize_type(&self) -> Result<Types, FinalizeError> {
        let parts_type = self
            .parts
            .iter()
            .try_fold(None, |pref: Option<Types>, term| {
                let term_type = term.finalize_type()?;
                match pref {
                    None => Ok(Some(term_type)),
                    Some(pref_type) => (pref_type + term_type)
                        .map(|result_type| Some(result_type))
                        .ok_or(FinalizeError::NoTypeToFinalize),
                }
            });
        match parts_type {
            Ok(Some(parts_type)) => Ok(parts_type),
            Ok(None) => Err(FinalizeError::NoTypeToFinalize),
            Err(err) => Err(err),
        }
    }

    fn finalize_value(self) -> Result<Values, FinalizeError> {
        let parts_values = self
            .parts
            .into_iter()
            .try_fold(None, |pref: Option<Values>, term| {
                let term_value = term.finalize_value()?;
                match pref {
                    None => Ok(Some(term_value)),
                    Some(pref_value) => (pref_value + term_value)
                        .map(|result_values| Some(result_values))
                        .ok_or(FinalizeError::NoValueToFinalize),
                }
            });
        match parts_values {
            Ok(Some(parts_values)) => Ok(parts_values),
            Ok(None) => Err(FinalizeError::NoTypeToFinalize),
            Err(err) => Err(err),
        }
    }
}

impl<Values, Types> Debug for SubPoly<Values, Types>
where
    Types: PolyTypes<Types>,
    Values: PolyValues<Types, Values>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "( ")?;
        let len = self.parts.len();
        self.parts.iter().enumerate().try_for_each(|(i, term)| {
            write!(f, "{:?}", term)?;
            if i + 1 != len {
                write!(f, " + ")?;
            }
            Ok(())
        })?;
        write!(f, " | ")?;
        match self.finalize_type() {
            Ok(finalized_type) => write!(f, "{:?}", finalized_type)?,
            Err(_) => write!(f, "None")?,
        }
        write!(f, " )")?;
        Ok(())
    }
}

impl<Values, Types> Display for SubPoly<Values, Types>
where
    Types: PolyTypes<Types>,
    Values: PolyValues<Types, Values>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let len = self.parts.len();
        self.parts.iter().enumerate().try_for_each(|(i, term)| {
            write!(f, "{}", term)?;
            if i + 1 != len {
                write!(f, " + ")?;
            }
            Ok(())
        })?;
        Ok(())
    }
}
