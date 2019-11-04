use crate::{token::Factoken, operators::Operators};

#[derive(Debug, Default)]
pub struct Operation
{
    operator: Operators,
    facts: (Box<Factoken>, Box<Factoken>),
	raw: String
}
