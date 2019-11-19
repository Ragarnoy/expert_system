#[derive(Debug, Copy, Clone)]
// Please don't touch to the ordering.
// The order of the 3 first operators is used to determine
// the priority of an operator by casting the enum into a u32.
pub enum Operators
{
    Xor,
    Or,
    And,
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