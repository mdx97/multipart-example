use arma_rs::{IntoArma, Value as ArmaValue};

pub type ArmaResult = Result<ArmaValue, ArmaValue>;

pub trait IntoArmaResult {
    fn into_arma_result(self) -> ArmaResult;
}

pub trait IntoArmaError<T> {
    fn into_arma_error(self) -> Result<T, ArmaValue>;
}

impl<T> IntoArmaResult for anyhow::Result<T>
where
    T: IntoArma,
{
    fn into_arma_result(self) -> ArmaResult {
        match self {
            Ok(value) => Ok(value.to_arma()),
            Err(error) => Err(error.to_string().to_arma()),
        }
    }
}

impl<T> IntoArmaError<T> for anyhow::Result<T> {
    fn into_arma_error(self) -> Result<T, ArmaValue> {
        match self {
            Ok(value) => Ok(value),
            Err(error) => Err(error.to_string().to_arma()),
        }
    }
}
