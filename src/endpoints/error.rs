use crate::types::common::SimpleError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Error<Cbe> {
    SerilizationError(Option<String>),
    DeserilizationError(Option<String>),
    CallbackError(Cbe),
    SimpleError(SimpleError),
}

impl<Cbe> From<Cbe> for Error<Cbe> {
    fn from(e: Cbe) -> Self {
        Self::CallbackError(e)
    }
}
