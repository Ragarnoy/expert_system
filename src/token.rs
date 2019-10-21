use crate::{fact::Fact, operation::Operation};

#[derive(Debug)]
pub enum Factoken
{
    Fact(Fact),
    Operation(Operation),
}
