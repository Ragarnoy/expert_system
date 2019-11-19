use crate::{token::Factoken, operators::Operators};

#[derive(Debug, Default)]
pub struct Operation
{
    operator: Operators,
    facts: (Box<Factoken>, Box<Factoken>),
	raw: String
}

impl Operation
{
	pub fn new(input: &str, priorities: Vec<(usize, usize, Operators)>) -> Result<Self, String>
	{
		// TODO: Implement me
		Ok(Self::default())
	}
}