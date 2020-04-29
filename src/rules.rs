use crate::{operators::Operators, token::Factoken, fact::Fact, operation::Operation};
use std::{collections::HashMap, iter::FromIterator};

#[derive(Default, Debug, PartialEq, Eq, Hash, Clone)]
pub struct Rule 
{
    left: Factoken,
    right: Factoken,
    middle: Operators,
}

impl Rule
{
	pub fn resolve(&self, rules: &Vec<Rule>, known: &mut HashMap<Fact, Option<bool>>, seen: &mut HashMap<Rule, Vec<Fact>>) -> HashMap<Fact, Option<bool>>
	{
		match self.left.resolve(rules, known, seen)
		{
			Some(res) => self.right.resolve_as_conclusion(res),
			None => HashMap::from_iter(self.right.get_facts().iter().map(|&f| (f, None)))
		}
	}

	pub fn has_been_seen_for_fact(&self, seen: &mut HashMap<Rule, Vec<Fact>>, fact: &Fact) -> bool
	{
		seen.get(self).is_some() && seen[self].iter().any(|f| f == fact)
	}

	pub fn contains_fact_as_conclusion(&self, fact: &Fact) -> bool
	{
		self.right.contains_fact(fact)
	}

	pub fn from_operator(input: &str, index: usize, operator: &str) -> Result<Self, String>
	{
		let left = &input[..index];
		let right = &input[index + operator.len()..];
		let left_p = Operation::get_operators_sorted_by_priority(&left)?;
		let right_p = Operation::get_operators_sorted_by_priority(&right)?;

		Ok(Rule
		{
			left: Factoken::new(left, left_p, 0)?,
			right: Factoken::new(right, right_p, 0)?,
			middle: Operators::Then
		})
	}

	pub fn new(input: &str) -> Result<Self, String>
	{
		// TODO: We should search here for duplicated fact
		// in both operation and conclusion side.
		// Or maybe not, is it so bad to have the same fact declared multiple times ?
		let mut op = input.match_indices("=>");
		// let mut op = op.chain(input.match_indices("=>"));
		let operator = op.next();

		if input.contains("<=>")
		{
			Err(format!("sorry, we don't support `<=>` operator: `{}`", input))
		}
		else if operator.is_none() || op.count() != 0
		{
			Err(format!("a rule MUST contains exactly one `=>` OR `<=>` operator: `{}`", input))
		}
		else
		{
			let (index, operator) = operator.unwrap();
			Self::from_operator(input, index, operator)
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