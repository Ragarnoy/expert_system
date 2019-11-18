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

impl Operators
{
	pub fn is_present(input: &str) -> bool
	{
		static operators: &[char] = &['+', '|', '^'];
		input.contains(operators)
	}
}