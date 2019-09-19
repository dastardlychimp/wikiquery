use serde;
use serde::{Deserialize};

#[derive(Deserialize, Debug, PartialEq)]
pub struct WikiError
{
    pub error: WikiErrorInner,
    #[serde(rename="servedby")]
    pub served_by: String
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct WikiErrorInner
{
    pub code: String,
    pub info: String,
    pub docref: String,
}
