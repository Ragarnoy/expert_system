#[derive(Debug)]
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