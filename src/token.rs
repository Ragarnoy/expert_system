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
	pub fn new(input: &str, priorities: Vec<(usize, usize, Operators)>) -> Result<Self, String>
	{
		Ok(if Operators::is_present(input)
		{
			Factoken::Operation(Operation::new(input, priorities)?)
		}
		else
		{
			Factoken::Fact(Fact::new(input)?)
		})
	}
}