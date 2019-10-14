use crate::*;

pub struct Operation
{
    outcome: Result<Outcome, Error>,
    operator: Operators,
    facts: (Fact, Fact),
}

pub struct Fact
{
    name: char,
    not: bool,
}