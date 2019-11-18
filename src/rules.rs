use crate::{operators::Operators, token::Factoken};

#[derive(Default, Debug)]
pub struct Rule 
{
    pub left: Factoken,
    pub right: Factoken,
    pub middle: Operators,
}

impl Rule
{
	pub fn new(input: &str) -> Result<Self, String>
	{
		if input.matches("=>").count() == 1
		{
			let mut parts = input.split("=>");
			let left = parts.next().unwrap_or("");
			let right  = parts.next().unwrap_or("");
			Ok(Rule
			{
				left: Factoken::new(left)?,
				right: Factoken::new(right)?,
				middle: Operators::Then
			})
		}
		else if input.matches("<=>").count() == 1
		{
			let mut parts = input.split("<=>");
			let left = parts.next().unwrap_or("");
			let right  = parts.next().unwrap_or("");
			Ok(Rule
			{
				left: Factoken::new(left)?,
				right: Factoken::new(right)?,
				middle: Operators::Then
			})
		}
		else
		{
			Err(format!("a rule MUST contains exactly one `=>` OR `<=>` operator: {}", input))
		}
	}
}
