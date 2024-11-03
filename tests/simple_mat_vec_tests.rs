mod simple_mat_vec;

mod tests {
    use crate::simple_mat_vec::{objects::Scalar, types::Types, values::Values};
    use poly_gnom::{
        polynomial::Polynomial,
        symbol::{SymbolInfo, SymbolsProvider},
        traits::{One, PolyValues},
    };

    fn basic_sumbols_provider() -> SymbolsProvider<Types> {
        let provider = SymbolsProvider::empty();

        provider.add(SymbolInfo::new_typed("x", Types::Scalar));
        provider.add(SymbolInfo::new_typed("y", Types::Scalar));
        provider.add(SymbolInfo::new_typed("z", Types::Scalar));

        provider.add(SymbolInfo::new_typed("u", Types::Vector));
        provider.add(SymbolInfo::new_typed("v", Types::Vector));
        provider.add(SymbolInfo::new_typed("w", Types::Vector));

        provider.add(SymbolInfo::new_typed("A", Types::Matrix));
        provider.add(SymbolInfo::new_typed("B", Types::Matrix));
        provider.add(SymbolInfo::new_typed("C", Types::Matrix));

        provider.add(SymbolInfo::new("r", None));
        provider.add(SymbolInfo::new("p", None));
        provider.add(SymbolInfo::new("q", None));

        provider
    }

    type MatVecPolynomial = Polynomial<Values, Types>;

    // Current implementation do not allow simple Add and Mul for Polynomial
    // I know how to fix that (written in polynomial.rs near commented Mul implementation)
    // but that requires time that I do not have much now

    #[test]
    fn basic_scalar_sum_test() {
        let provider = basic_sumbols_provider();

        let poly1 = MatVecPolynomial::one(Types::Scalar).unwrap();
        let poly2 = poly1.clone();

        let poly = MatVecPolynomial::builder()
            .term_builder(Values::new_scalar(1))
            .variable(provider.get("x").unwrap(), 1)
            .build()
            .term_builder(Values::new_scalar(1))
            .variable(provider.get("y").unwrap(), 1)
            .build()
            .build()
            .unwrap();

        let poly = poly
            .substitute_polynomial(provider.get("x").unwrap(), poly1)
            .substitute_polynomial(provider.get("y").unwrap(), poly2);

        let value = poly.as_value().unwrap();

        assert_eq!(value, Values::new_scalar(2));
    }

    #[test]
    fn basic_scalar_mul_test() {
        let provider = basic_sumbols_provider();

        let poly1 = MatVecPolynomial::builder()
            .term_builder(Values::new_scalar(2))
            .build()
            .build()
            .unwrap();
        let poly2 = MatVecPolynomial::builder()
            .term_builder(Values::new_scalar(3))
            .build()
            .build()
            .unwrap();

        let poly = MatVecPolynomial::builder()
            .term_builder(Values::new_scalar(1))
            .variable(provider.get("x").unwrap(), 1)
            .variable(provider.get("y").unwrap(), 1)
            .build()
            .build()
            .unwrap();

        let poly = poly
            .substitute_polynomial(provider.get("x").unwrap(), poly1)
            .substitute_polynomial(provider.get("y").unwrap(), poly2);

        let value = poly.as_value().unwrap();

        assert_eq!(value, Values::new_scalar(6));
    }

    #[test]
    fn basic_sum_test() {
        let provider = basic_sumbols_provider();

        let operands_and_result = vec![
            (
                Values::new_scalar(2),
                Values::new_scalar(4),
                Values::new_scalar(6),
            ),
            (
                Values::new_vector(vec![1, 2]),
                Values::new_vector(vec![6, 3]),
                Values::new_vector(vec![7, 5]),
            ),
            (
                Values::new_matrix(vec![vec![0, 1], vec![2, 3]]),
                Values::new_matrix(vec![vec![8, 7], vec![6, 5]]),
                Values::new_matrix(vec![vec![8, 8], vec![8, 8]]),
            ),
        ];

        let adder = MatVecPolynomial::builder()
            .term_builder(Values::new_scalar(1))
            .variable(provider.get("p").unwrap(), 1)
            .build()
            .term_builder(Values::new_scalar(1))
            .variable(provider.get("q").unwrap(), 1)
            .build()
            .build()
            .unwrap();

        for (lhs, rhs, res) in operands_and_result {
            let sum = adder
                .clone()
                .substitute_value(provider.get("p").unwrap(), lhs)
                .substitute_value(provider.get("q").unwrap(), rhs)
                .as_value()
                .unwrap();
            assert_eq!(res, sum);
        }
    }

    #[test]
    fn basic_mul_test() {
        let provider = basic_sumbols_provider();

        let operands_and_result = vec![
            (
                Values::new_scalar(2),
                Values::new_scalar(4),
                Some(Values::new_scalar(8)),
            ),
            (
                Values::new_vector(vec![1, 2]),
                Values::new_vector(vec![6, 3]),
                None,
            ),
            (
                Values::new_matrix(vec![vec![0, 1], vec![2, 3]]),
                Values::new_matrix(vec![vec![8, 7], vec![6, 5]]),
                Some(Values::new_matrix(vec![vec![6, 5], vec![34, 36]])),
            ),
        ];

        let multiplyer = MatVecPolynomial::builder()
            .term_builder(Values::new_scalar(1))
            .variable(provider.get("p").unwrap(), 1)
            .variable(provider.get("q").unwrap(), 1)
            .build()
            .build()
            .unwrap();

        for (lhs, rhs, res) in operands_and_result {
            let mul = multiplyer
                .clone()
                .substitute_value(provider.get("p").unwrap(), lhs)
                .substitute_value(provider.get("q").unwrap(), rhs)
                .as_value();
            assert_eq!(res, mul.ok());
        }
    }

    #[test]
    fn basic_polynomial_test_1() {
        let provider = basic_sumbols_provider();

        let permutation_matrix = vec![
            vec![0, 1, 0, 0],
            vec![0, 0, 0, 1],
            vec![0, 0, 1, 0],
            vec![1, 0, 0, 0],
        ];

        let poly = MatVecPolynomial::builder()
            .term_builder(Values::new_scalar(5))
            .value(Values::new_matrix(permutation_matrix), 1)
            .variable(provider.get("v").unwrap(), 1)
            .build()
            .build()
            .unwrap();

        let vectors = vec![
            (Values::new_vector(vec![1, 2, 3]), None),
            (
                Values::new_vector(vec![1, 2, 3, 4]),
                Some(Values::new_vector(vec![10, 20, 15, 5])),
            ),
            (
                Values::new_vector(vec![2, 2, 4, 4]),
                Some(Values::new_vector(vec![10, 20, 20, 10])),
            ),
        ];

        assert_eq!(poly.as_type().unwrap(), Types::Vector);

        for (v, res) in vectors {
            let result = poly
                .clone()
                .substitute_value(provider.get("v").unwrap(), v)
                .as_value();
            assert_eq!(res, result.ok());
        }
    }
}
