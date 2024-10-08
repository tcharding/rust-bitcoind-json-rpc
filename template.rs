
impl Baz {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> Result<model::Baz, BazError> {
        use BazError as E;

        let foo = self.foo.parse::<Amount>().map_err(E::Foo)?;
        let bar = self.bar.map(|s| s.parse::<hash160::Hash>()).transpose().map_err(E::Bar)?;

        model::Baz { foo, bar }
    }
}

/// Error when converting a `Baz` type into the model type.
#[derive(Debug)]
pub enum BazError {
    /// Conversion of the `foo` field failed.
    Foo(ParseAmountError),
    /// Conversion of the `bar` field failed.
    Bar(hex::HexToArrayError),
}

impl fmt::Display for BazError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use BazError::*;

        match *self {
            Foo(ref e) => write_err!(f, "conversion of the `foo` field failed"; e),
            Bar(ref e) => write_err!(f, "conversion of the `bar` field failed"; e),
        }
    }
}

impl std::error::Error for BazError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use BazError::*;

        match *self {
            Foo(ref e) => Some(e),
            Bar(ref e) => Some(e),
        }
    }
}
        
