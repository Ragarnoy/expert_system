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

    #[inline]
    pub fn is_operator(c: char) -> bool
    {
        "+|^".contains(c)
    }

    // Here `depth` should represent how deep we are inside parentheses
    pub fn get_priority(self, depth: isize) -> Option<usize>
    {
        let highest = Operators::get_highest_priority();
        let priority = self as usize + 1;

        if priority > highest || depth < 0
        {
            return None;
        }
        Some(highest * depth as usize + priority)
    }

    #[inline]
    fn get_highest_priority() -> usize
    {
        Operators::And as usize + 1
	}

	pub fn is_present(input: &str) -> bool
	{
		static operators: &[char] = &['+', '|', '^'];
		input.contains(operators)
	}
}

#[cfg(test)]
mod test_operators
{
	use super::*;

	#[test]
	fn test_new()
	{
		let op_and = Operators::new("+").unwrap();
		let op_or = Operators::new("|").unwrap();
		let op_xor = Operators::new("^").unwrap();
		let op_then = Operators::new("=>").unwrap();
		let op_if_only = Operators::new("<=>").unwrap();

		assert_eq!(Operators::And, op_and);
		assert_eq!(Operators::Or, op_or);
		assert_eq!(Operators::Xor, op_xor);
		assert_eq!(Operators::Then, op_then);
		assert_eq!(Operators::IfOnly, op_if_only);
	}
}