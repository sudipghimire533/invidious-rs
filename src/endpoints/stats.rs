use std::future::Future;
use std::pin::Pin;

use super::error::Error;
use super::InstanceUrl;
use crate::types;
use crate::types::common::SimpleError;
use crate::utils;
use crate::utils::GetOwned;
use serde::de::DeserializeOwned;

pub type ExpectedOk = Vec<u8>;
pub type ExpectedRes<Cbe> = Result<ExpectedOk, Error<Cbe>>;
