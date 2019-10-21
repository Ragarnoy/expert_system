use crate::{token::Factoken, operators::Operators};
use std::io::Error;

#[derive(Debug)]
pub struct Operation
{
    outcome: Result<Option<bool>, Error>,
    operator: Option<Operators>,
    facts: (Option<Box<Factoken>>, Option<Box<Factoken>>),
}

impl Default for Operation
{
    fn default() -> Self
    {
        Operation
        {
            outcome: Ok(None),
            operator: None,
            facts: (None, None),
        }
    }
}
