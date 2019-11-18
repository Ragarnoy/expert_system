use crate::{operators::Operators, token::Factoken, fact::Fact};
use std::{collections::HashMap, iter::FromIterator};

#[derive(Default, Debug, PartialEq, Eq, Hash, Clone)]
pub struct Rule 
{
    pub left: Factoken,
    pub right: Factoken,
    pub middle: Operators,
}

impl Rule
{
	pub fn resolve(&self, rules: &Vec<Rule>, known: &mut HashMap<Fact, Option<bool>>, seen: &mut HashMap<Rule, Vec<Fact>>) -> HashMap<Fact, Option<bool>>
	{
		match self.left.resolve(rules, known, seen)
		{
			Some(true) => self.right.resolve_as_conclusion(rules, known, seen, true),
			Some(false) => self.right.resolve_as_conclusion(rules, known, seen, false),
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

	pub fn new(input: &str) -> Result<Self, String>
	{
		// TODO: We should search here for duplicated fact
		// in both operation and conclusion side.
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
