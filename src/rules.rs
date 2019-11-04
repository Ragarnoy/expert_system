use crate::{operators::Operators, token::Factoken, fact::Fact};
use std::{collections::HashMap, iter::FromIterator};

#[derive(Default, Debug, PartialEq, Eq, Hash, Clone)]
pub struct Rule 
{
    pub left: Factoken,
    pub right: Factoken,
    pub middle: Operators,
}

// impl PartialEq for Rule
// {
// 	fn eq(&self, other: &Self) -> bool
// 	{
// 		self.left == other.left
// 			&& self.right == other.right
// 			&& self.middle == other.middle
// 	}
// }

impl Rule
{
	pub fn resolve/*<T>*/(&self, /*intials: &I, */rules: &Vec<Rule>, known: &mut HashMap<Fact, Option<bool>>, seen: &mut HashMap<Rule, Vec<Fact>>) -> HashMap<Fact, Option<bool>>// Option<bool>
	// where
	// 	T: Iterator<Item=Rule> + DoubleEndedIterator<Item=Rule>
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
		/*if */seen.get(self).is_some() && seen[self].iter().any(|f| f == fact)
		// {
		// 	return true;
		// }
		// seen.entry(self.clone()).or_insert(Vec::new()).push(*fact);
		// false
	}

	// pub fn resolve_if_not_seen_for_fact(&self, /*intials: &I, */rules: &Vec<Rule>, known: &mut HashMap<Fact, bool>, &mut seen: &mut HashMap<Rule, Vec<Fact>>, fact: &Fact) -> Option<bool>
	// {
	// 	if seen.get(self).is_some() && seen[self].iter().any(|f| f == fact)
	// 	{
	// 		return true;
	// 	}
	// 	seen.entry(self.clone()).or_insert(Vec::new()).push(*fact);
	// 	false
	// }

	pub fn contains_fact_as_conclusion(&self, fact: &Fact) -> bool
	{
		self.right.contains_fact(fact)
	}
}