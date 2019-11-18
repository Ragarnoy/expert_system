use crate::{fact::Fact, operation::Operation, rules::Rule};
use std::{fmt, collections::HashMap};

#[derive(Debug, Clone, Eq, Hash)]
pub enum Factoken
{
    Fact(Fact),
    Operation(Operation),
}

impl Default for Factoken
{
	fn default() -> Self
	{
		Self::Fact(Fact::default())
	}
}

impl PartialEq for Factoken
{
	fn eq(&self, other: &Self) -> bool
	{
		match (self, other)
		{
			(Factoken::Fact(f0), Factoken::Fact(f1)) => f0 == f1,
			(Factoken::Operation(o0), Factoken::Operation(o1)) => o0 == o1,
			(_, _) => false
		}
	}
}

impl fmt::Display for Factoken
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		match self
		{
			Factoken::Fact(fact) => write!(f, "{}", fact),
			Factoken::Operation(op) => write!(f, "{}", op)
		}
	}
}

impl Factoken
{
	pub fn resolve/*<T>*/(&self, /*intials: &I, */rules: &Vec<Rule>, known: &mut HashMap<Fact, Option<bool>>, seen: &mut HashMap<Rule, Vec<Fact>>) -> Option<bool>
	// where
	// 	T: Iterator<Item=Rule> + DoubleEndedIterator<Item=Rule>
	{
		match self
		{
			Factoken::Fact(f) if f.is_not() => f.resolve(rules, known, seen).map(|v| !v),
			Factoken::Fact(f) => f.resolve(rules, known, seen),
			Factoken::Operation(o) => o.resolve(rules, known, seen)
		}
	}

	pub fn resolve_as_conclusion(&self, /*intials: &I, */rules: &Vec<Rule>, known: &mut HashMap<Fact, Option<bool>>, seen: &mut HashMap<Rule, Vec<Fact>>, result: bool) -> HashMap<Fact, Option<bool>>// Option<bool>
	{
		match self
		{
			Self::Fact(f) => {
				let mut ret = HashMap::new();
				ret.insert(*f, Some(if f.is_not() { !result } else { result }));
				ret
			},
			Self::Operation(o) => o.resolve_as_conclusion(rules, known, seen, result)
		}
	}

	pub fn contains_fact(&self, fact: &Fact) -> bool
	{
		match self
		{
			Factoken::Fact(f) => f == fact,
			Factoken::Operation(o) => o.contains_fact(fact)
		}
	}

	pub fn get_facts(&self) -> Vec<Fact>
	{
		match self
		{
			Self::Fact(f) => vec!(*f),
			Self::Operation(o) => o.get_facts()
		}
	}
}