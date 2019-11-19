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
	pub fn new(input: &str) -> Result<Self, String>
	{
		// TODO: We should search here for duplicated fact
		// in both operation and conclusion side.

        // TODO: We could (should ?) use match_indices() instead of matches here
        // then pass use string slice instead of call split().
        // This way is probably faster than the current one.
		if input.matches("=>").count() == 1
		{
			let mut parts = input.split("=>");
			let left = parts.next().unwrap_or("");
			let right = parts.next().unwrap_or("");
            let left_p = Operation::get_operators_sorted_by_priority(left)?;
            let right_p = Operation::get_operators_sorted_by_priority(right)?;
			Ok(Rule
			{
				left: Factoken::new(left, left_p)?,
				right: Factoken::new(right, right_p)?,
				middle: Operators::Then
			})
		}
		else if input.matches("<=>").count() == 1
		{
			let mut parts = input.split("<=>");
			let left = parts.next().unwrap_or("");
			let right = parts.next().unwrap_or("");
            let left_p = Operation::get_operators_sorted_by_priority(left)?;
            let right_p = Operation::get_operators_sorted_by_priority(right)?;
			Ok(Rule
			{
				left: Factoken::new(left, left_p)?,
				right: Factoken::new(right, right_p)?,
				middle: Operators::IfOnly
			})
		}
		else
		{
			Err(format!("a rule MUST contains exactly one `=>` OR `<=>` operator: {}", input))
		}
	}
}
