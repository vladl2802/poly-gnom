use crate::{
    error::{BuilderError, FinalizeError},
    factor::{Factor, Finalizable, SubPoly, Substitutiable, Value, Variable},
    symbol::Symbol,
    term::{Term, TermBuilder},
    traits::{PolyTypes, PolyValues},
};

pub struct Polynomial<Values, Types> {
    poly: SubPoly<Values, Types>,
}

impl<Values, Types> Into<SubPoly<Values, Types>> for Polynomial<Values, Types> {
    fn into(self) -> SubPoly<Values, Types> {
        self.poly
    }
}

impl<Values, Types> Polynomial<Values, Types>
where
    Types: PolyTypes<Types>,
    Values: PolyValues<Types, Values>,
{
    pub fn builder() -> PolynomialBuilder<Values, Types> {
        PolynomialBuilder::new()
    }

    pub fn zero(coefficient_type: Types) -> Result<Self, BuilderError> {
        Self::builder()
            .term_builder(
                Values::one_with_type(coefficient_type).ok_or(BuilderError::CoefficientError)?,
            )
            .build()
            .build()
    }

    pub fn one(coefficient_type: Types) -> Result<Self, BuilderError> {
        Self::builder()
            .term_builder(
                Values::zero_with_type(coefficient_type).ok_or(BuilderError::CoefficientError)?,
            )
            .build()
            .build()
    }

    pub fn substitute_value(self, to: Symbol<Types>, value: Values) -> Self {
        self.substitute(to, Factor::Value(Value::new(value)))
    }

    pub fn substitute_variable(self, to: Symbol<Types>, symbol: Symbol<Types>) -> Self {
        self.substitute(to, Factor::Variable(Variable::new(symbol)))
    }

    pub fn substitute_polynomial(
        self,
        to: Symbol<Types>,
        polynomial: Polynomial<Values, Types>,
    ) -> Self {
        self.substitute(to, Factor::SubPoly(polynomial.into()))
    }
}

impl<Values, Types> Substitutiable<Values, Types> for Polynomial<Values, Types>
where
    Types: PolyTypes<Types>,
    Values: PolyValues<Types, Values>,
{
    type Output = Self;

    fn substitute(self, to: Symbol<Types>, factor: Factor<Values, Types>) -> Self::Output {
        match self.poly.substitute(to, factor) {
            Factor::Value(_) => panic!(),
            Factor::Variable(_) => panic!(),
            Factor::SubPoly(poly) => Polynomial::<Values, Types> { poly },
        }
    }
}

impl<Values, Types> Finalizable<Values, Types> for Polynomial<Values, Types>
where
    Types: PolyTypes<Types>,
    Values: PolyValues<Types, Values>,
{
    fn finalize_type(&self) -> Result<Types, FinalizeError> {
        self.poly.finalize_type()
    }

    fn finalize_value(self) -> Result<Values, FinalizeError> {
        self.poly.finalize_value()
    }
}

pub struct PolynomialBuilder<Values, Types> {
    parts: Vec<Result<Term<Values, Types>, BuilderError>>,
}

impl<Values, Types> PolynomialBuilder<Values, Types> {
    pub fn new() -> Self {
        PolynomialBuilder { parts: Vec::new() }
    }

    pub fn term_builder(self, coefficient: Values) -> TermBuilder<Values, Types> {
        TermBuilder::new(self, coefficient)
    }

    pub fn maybe_term(
        mut self,
        term: Result<Term<Values, Types>, BuilderError>,
    ) -> PolynomialBuilder<Values, Types> {
        self.parts.push(term);
        self
    }

    pub fn term(self, term: Term<Values, Types>) -> PolynomialBuilder<Values, Types> {
        self.maybe_term(Ok(term))
    }

    pub fn build(self) -> Result<Polynomial<Values, Types>, BuilderError> {
        let parts = self
            .parts
            .into_iter()
            .collect::<Result<Vec<_>, BuilderError>>()?;
        Ok(Polynomial {
            poly: SubPoly::new(parts),
        })
    }
}
