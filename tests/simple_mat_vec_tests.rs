mod simple_mat_vec;

mod tests {
    use poly_gnom::{polynomial::Polynomial, symbol::{SymbolInfo, SymbolsProvider}};
    use crate::simple_mat_vec::{types::Types, values::Values};

    fn basic_sumbols_provider() -> SymbolsProvider<Types> {
        let provider = SymbolsProvider::empty();

        provider.add(SymbolInfo::new_typed("x",  Types::Scalar));
        provider.add(SymbolInfo::new_typed("y",  Types::Scalar));
        provider.add(SymbolInfo::new_typed("z",  Types::Scalar));

        provider.add(SymbolInfo::new_typed("u",  Types::Vector));
        provider.add(SymbolInfo::new_typed("v",  Types::Vector));
        provider.add(SymbolInfo::new_typed("w",  Types::Vector));

        provider.add(SymbolInfo::new_typed("A",  Types::Matrix));
        provider.add(SymbolInfo::new_typed("B",  Types::Matrix));
        provider.add(SymbolInfo::new_typed("C",  Types::Matrix));

        provider
    }

    type MatVecPolynomial = Polynomial<Values, Types>;

    #[test]
    fn basic_test() {
        let provider = basic_sumbols_provider();
        let poly = MatVecPolynomial::one(Types::Scalar);
        
    }
}
