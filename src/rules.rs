use crate::{operators::Operators, token::Factoken, fact::Fact, operation::Operation};
use std::{collections::HashMap, iter::FromIterator};

#[derive(Default, Debug)]
pub struct Rule 
{
    pub left: Factoken,
    pub right: Factoken,
    pub middle: Operators,
}

impl Rule
{
	pub fn from_operator(input: &str, index: usize, operator: &str) -> Result<Self, String>
	{
		let left = &input[..index];
		let right = &input[index + operator.len()..];
		let left_p = Operation::get_operators_sorted_by_priority(&left)?;
		let right_p = Operation::get_operators_sorted_by_priority(&right)?;

		Ok(Rule
		{
			left: Factoken::new(left, left_p)?,
			right: Factoken::new(right, right_p)?,
			middle: Operators::Then
		})
	}

	pub fn new(input: &str) -> Result<Self, String>
	{
		// TODO: We should search here for duplicated fact
		// in both operation and conclusion side.
		// Or maybe not, is it so bad to have the same fact declared multiple times ?
		let mut then = input.match_indices(Operators::then());
		let mut if_only = input.match_indices(Operators::if_only());

		if then.clone().count() == 1 && if_only.clone().count() == 0
		{
			let (index, then) = then.next().unwrap();
			Self::from_operator(input, index, then)
		}
		else if if_only.clone().count() == 1 && then.clone().count() == 0
		{
			let (index, if_only) = if_only.next().unwrap();
			Self::from_operator(input, index, if_only)
		}
		else
		{
			Err(format!("a rule MUST contains exactly one `=>` OR `<=>` operator: {}", input))
		}
	}
}

#[cfg(test)]
mod rule_tests
{
	use super::*;

	#[test]
	fn test_constructeur()
	{
		let rule = Rule::new("A | (B + C) => D");
		println!("Rule: {:#?}", rule)
	}
}