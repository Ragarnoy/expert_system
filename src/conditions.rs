use crate::*;


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


#[derive(Default, Debug)]
pub struct Fact
{
    name: char,
    not: bool,
}

#[derive(Debug)]
pub enum Factoken
{
    Fact(Fact),
    Operation(Operation),
}

impl Fact
{
    fn new(input: &str) -> Self
    {
        if input.len() == 2
        {
            Fact{
                name: input.chars().nth(1).unwrap(),
                not: true,
            }
        }
        else
        {
            Fact{
                name: input.chars().nth(0).unwrap(),
                not: false,
            }
        }
    }
}