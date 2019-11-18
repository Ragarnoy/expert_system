use crate::{fact::Fact, operation::Operation, operators::Operators};

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
		Ok(if Operators::is_present(input)
		{
			Factoken::Operation(Operation::new(input)?)
		}
		else
		{
			Factoken::Fact(Fact::new(input)?)
		})
	}
}