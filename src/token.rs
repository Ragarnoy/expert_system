use crate::{fact::Fact, operation::Operation};

#[derive(Debug)]
pub enum Factoken
{
    Fact(Fact),
    Operation(Operation),
}

impl Default for Factoken
{
	fn default() -> Self
	{
		Self::Fact(Fact::default())
	}
}

impl Factoken
{
	pub fn new(input: &str) -> Result<Self, String>
	{
		// TODO: Implement me
		Ok(Self::default())
	}
}