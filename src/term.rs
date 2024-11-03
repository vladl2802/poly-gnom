use std::{fmt::{self, Debug, Display}, ops::Neg};

use crate::{
    error::{BuilderError, FinalizeError},
    factor::{Factor, Finalizable, Substitutiable, Value, Variable},
    polynomial::{Polynomial, PolynomialBuilder},
    symbol::Symbol,
    traits::{PolyTypes, PolyValues},
};

#[derive(Clone)]
pub struct Term<Values, Types> {
    coefficient: Values,
    monomial: Vec<MonomialFactor<Values, Types>>,
}

// Seems like finalize of type and value are vere similar operations,
// so maybe it make sence to rethink finalizing in terms of common properties of values and types (such as result of multiplication etc)
// But only in next iteration because I'm at third iteration and still do not commit anything

impl<Values, Types> Substitutiable<Values, Types> for Term<Values, Types>
where
    Types: PolyTypes<Types>,
    Values: PolyValues<Types, Values>,
{
    type Output = Term<Values, Types>;

    fn substitute(self, to: Symbol<Types>, factor: Factor<Values, Types>) -> Self::Output {
        Term {
            coefficient: self.coefficient,
            monomial: self
                .monomial
                .into_iter()
                .map(|monomial_factor| monomial_factor.substitute(to.clone(), factor.clone()))
                .collect(),
        }
    }
}

impl<Values, Types> Finalizable<Values, Types> for Term<Values, Types>
where
    Types: PolyTypes<Types>,
    Values: PolyValues<Types, Values>,
{
    fn finalize_type(&self) -> Result<Types, FinalizeError> {
        let coef_type = self.coefficient.as_type();
        let monomial_type =
            self.monomial
                .iter()
                .try_fold(None, |pref: Option<Types>, factor| {
                    let factor_type = factor.finalize_type()?;
                    Ok(Some(match pref {
                        None => factor_type,
                        Some(pref_type) => (pref_type * factor_type)
                            .result
                            .ok_or(FinalizeError::NoTypeToFinalize)?,
                    }))
                })?;
        match monomial_type {
            Some(monomial_type) => (coef_type * monomial_type)
                .result
                .ok_or(FinalizeError::NoTypeToFinalize),
            None => Ok(coef_type),
        }
    }

    fn finalize_value(self) -> Result<Values, FinalizeError> {
        let finalized_type = self.finalize_type()?;
        let monomial_value =
            self.monomial
                .into_iter()
                .try_fold(None, |pref: Option<Values>, factor| {
                    let factor_value = factor.finalize_value()?;
                    Ok(Some(match pref {
                        None => factor_value,
                        Some(pref_value) => (pref_value * factor_value).ok_or(FinalizeError::NoValueToFinalize)?,
                    }))
                })?;
        let result = match monomial_value {
            Some(monomial_value) => {
                (self.coefficient * monomial_value).expect("type verified operation failed")
            }
            None => self.coefficient.into(),
        };
        assert!(result.as_type() == finalized_type);
        Ok(result)
    }
}

impl<Values, Types> Neg for Term<Values, Types>
where
    Types: PolyTypes<Types>,
    Values: PolyValues<Types, Values>,
{
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.coefficient = -self.coefficient;
        self
    }
}

impl<Values, Types> Debug for Term<Values, Types>
where
    Types: PolyTypes<Types>,
    Values: PolyValues<Types, Values>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "( ( {:?} | {:?} ) ",
            self.coefficient,
            self.coefficient.as_type()
        )?;
        self.monomial
            .iter()
            .try_for_each(|monomial_factor| write!(f, "{:?} ", monomial_factor))?;
        write!(f, "| ")?;
        match self.finalize_type() {
            Ok(finalized_type) => write!(f, "{:?}", finalized_type)?,
            Err(_) => write!(f, "None")?,
        }
        write!(f, " )")?;
        Ok(())
    }
}

impl<Values, Types> Display for Term<Values, Types>
where
    Types: PolyTypes<Types>,
    Values: PolyValues<Types, Values>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.coefficient)?;
        self.monomial
            .iter()
            .try_for_each(|monomial_factor| write!(f, " {}", monomial_factor))?;
        Ok(())
    }
}

#[derive(Clone)]
struct MonomialFactor<Values, Types> {
    factor: Factor<Values, Types>,
    power: u64,
}

impl<Values, Types> Substitutiable<Values, Types> for MonomialFactor<Values, Types>
where
    Types: PolyTypes<Types>,
    Values: PolyValues<Types, Values>,
{
    type Output = MonomialFactor<Values, Types>;

    fn substitute(self, to: Symbol<Types>, factor: Factor<Values, Types>) -> Self::Output {
        MonomialFactor {
            factor: self.factor.substitute(to.clone(), factor.clone()),
            power: self.power,
        }
    }
}

impl<Values, Types> Finalizable<Values, Types> for MonomialFactor<Values, Types>
where
    Types: PolyTypes<Types>,
    Values: PolyValues<Types, Values>,
{
    fn finalize_type(&self) -> Result<Types, FinalizeError> {
        let factor_type = self.factor.finalize_type();
        if self.power == 1 {
            return factor_type;
        }
        let factor_type = factor_type?;
        let mul_result = (factor_type.clone() * factor_type.clone())
            .result
            .ok_or(FinalizeError::NoTypeToFinalize)?;
        if mul_result == factor_type {
            Ok(mul_result)
        } else {
            Err(FinalizeError::NoTypeToFinalize)
        }
    }

    fn finalize_value(self) -> Result<Values, FinalizeError> {
        let finalized_type = self.finalize_type()?;
        let factor_value = self.factor.finalize_value();

        if self.power == 1 {
            return factor_value;
        }

        if self.power == 0 {
            return Values::one_with_type(finalized_type).ok_or(FinalizeError::NoValueToFinalize);
        }
        let mut factor_value = factor_value?;

        // move this to separate place
        let mut power = self.power;
        while power & 2 == 0 {
            factor_value =
                (factor_value.clone() * factor_value).expect("type verified operation failed");
            power /= 2;
        }
        if power == 1 {
            assert!(factor_value.as_type() == finalized_type);
            return Ok(factor_value);
        }
        let mut result = factor_value.clone();
        while power > 1 {
            power /= 2;
            factor_value =
                (factor_value.clone() * factor_value).expect("type verified operation failed");
            if power & 2 == 1 {
                result = (result * factor_value.clone()).expect("type verified operation failed");
            }
        }
        assert!(factor_value.as_type() == finalized_type);
        Ok(factor_value)
    }
}

impl<Values, Types> Debug for MonomialFactor<Values, Types>
where
    Types: PolyTypes<Types>,
    Values: PolyValues<Types, Values>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.power == 1 {
            write!(f, "{:?}", self.factor)?;
        } else {
            write!(f, "( {:?}^{} | ", self.factor, self.power)?;
            match self.finalize_type() {
                Ok(finalized_type) => write!(f, "{:?}", finalized_type)?,
                Err(_) => write!(f, "None")?,
            }
            write!(f, " )")?;
        }
        Ok(())
    }
}

impl<Values, Types> Display for MonomialFactor<Values, Types>
where
    Types: PolyTypes<Types>,
    Values: PolyValues<Types, Values>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.factor)?;
        if self.power != 1 {
            write!(f, "^{}", self.power)?;
        }
        Ok(())
    }
}
pub struct TermBuilder<Values, Types> {
    parent: PolynomialBuilder<Values, Types>,
    coefficient: Values,
    factors: Vec<(Result<Factor<Values, Types>, BuilderError>, u64)>,
}

impl<Values, Types> TermBuilder<Values, Types> {
    pub fn new(parent: PolynomialBuilder<Values, Types>, coefficient: Values) -> Self {
        TermBuilder {
            parent,
            coefficient,
            factors: vec![],
        }
    }

    pub fn maybe_value(mut self, value: Result<Values, BuilderError>, power: u64) -> Self {
        self.factors
            .push((value.map(|value| Factor::Value(Value::new(value))), power));
        self
    }

    pub fn value(self, value: Values, power: u64) -> Self {
        self.maybe_value(Ok(value), power)
    }

    pub fn maybe_variable(
        mut self,
        symbol: Result<Symbol<Types>, BuilderError>,
        power: u64,
    ) -> Self {
        self.factors.push((
            symbol.map(|symbol| Factor::Variable(Variable::new(symbol))),
            power,
        ));
        self
    }

    pub fn variable(self, symbol: Symbol<Types>, power: u64) -> Self {
        self.maybe_variable(Ok(symbol), power)
    }

    pub fn maybe_polynomial(
        mut self,
        polynomial: Result<Polynomial<Values, Types>, BuilderError>,
        power: u64,
    ) -> Self {
        self.factors.push((
            polynomial.map(|polynomial| Factor::SubPoly(polynomial.into())),
            power,
        ));
        self
    }

    pub fn polynomial(self, polynomial: Polynomial<Values, Types>, power: u64) -> Self {
        self.maybe_polynomial(Ok(polynomial), power)
    }

    pub fn build(self) -> PolynomialBuilder<Values, Types> {
        self.parent.maybe_term(
            self.factors
                .into_iter()
                .map(|(maybe_factor, power)| {
                    maybe_factor.map(|factor| MonomialFactor { factor, power })
                })
                .collect::<Result<Vec<_>, BuilderError>>()
                .map(|monomial| Term {
                    coefficient: self.coefficient,
                    monomial,
                }),
        )
    }
}
