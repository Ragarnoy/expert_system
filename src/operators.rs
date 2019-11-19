#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Operators
{
    And,
    Or,
    Xor,
    Then,
    IfOnly,
}

impl Default for Operators
{
	fn default() -> Self
	{
		Self::Then
	}
}

impl Operators
{
	pub fn new(input: &str) -> Result<Self, String>
	{
		match input.len()
		{
			1 => match input.chars().nth(0).unwrap()
			{
				'+' => Ok(Operators::And),
				'|' => Ok(Operators::Or),
				'^' => Ok(Operators::Xor),
				_ => Err(format!("`{}`: this cannot be an operators", input))
			},
			2 if input == "=>" => Ok(Operators::Then),
			3 if input == "<=>" => Ok(Operators::IfOnly),
			_ => Err(format!("`{}`: this cannot be an operators", input))
		}
	}

	pub fn resolve(&self, value0: Option<bool>, value1: Option<bool>) -> Option<bool>
	{
		match (value0, value1)
		{
			(Some(v0), Some(v1)) => match self
			{
				Operators::And => Some(v0 && v1),
				Operators::Or => Some(v0 || v1),
				Operators::Xor => Some(v0 ^ v1),
				_ => None
			},
			(Some(v), None) if *self == Operators::Or && v == true => Some(true),
			(None, Some(v)) if *self == Operators::Or && v == true => Some(true),
			(Some(v), None) if *self == Operators::And && v == false => Some(false),
			(None, Some(v)) if *self == Operators::And && v == false => Some(false),
			_ => None
		}
	}
}