use crate::{token::Factoken, operators::Operators, rules::Rule, fact::Fact};
use std::{
	collections::HashMap,
	iter::FromIterator,
	fmt,
	hash::{Hash, Hasher}
};

#[derive(Debug, Default, Clone)]
pub struct Operation
{
    operator: Operators,
    facts: (Box<Factoken>, Box<Factoken>),
	raw: String
}

impl PartialEq for Operation
{
	fn eq(&self, other: &Self) -> bool
	{
		self.raw == other.raw
		// self.operator == other.operator
		// 	&& self.facts.0 == other.facts.0
		// 	&& self.facts.1 == other.facts.1
	}
}

impl Eq for Operation { }

impl Hash for Operation
{
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		self.raw.hash(state);
		// self.operator.hash(state);
		// self.facts.hash(state);
	}
}

impl fmt::Display for Operation
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		write!(f, "{}", self.raw)
	}
}

impl Operation
{
	// Here `depth` will helps us to know if we are
	// between parentheses or not and how deep we are:
	//	- `depth` > 0 => we are between parentheses at `depth` level
	// if we are a the end of the string this means `depth` closing parenthese is/are missing
	//	- `depth` = 0 => we aren't between parentheses
	//	- `depth` < 0 => there is a closing parenthese without a matching opening parenthese
	//
	// To create a new Operation the easiest seems to find the operator with the lowest priority
	// then call the `new()` method on Factoken on each part of the operation:
	// 	- the first part should be a substring from the beginning to operator excluded
	//	- the second part should be a substring from the first char after the operator to the end
	//
	// /!\ WE STILL DON'T KNOW HOW TO HANDLE PARENTHESES PROPERLY HERE /!\
	// pub fn new(input: &str) -> Result<Self, String>
	// {
	// 	let mut depth: i64 = 0;

	// 	match 
	// 	for c in input.chars()
	// 	{
	// 		match c
	// 		{
	// 			'(' => 
	// 		}
	// 	}
	// }

	pub fn resolve/*<T>*/(&self, /*intials: &I, */rules: &Vec<Rule>, known: &mut HashMap<Fact, Option<bool>>, seen: &mut HashMap<Rule, Vec<Fact>>) -> Option<bool>
	// where
	// 	T: Iterator<Item=Rule> + DoubleEndedIterator<Item=Rule>
	{
		match &self.facts
		{
			(Some(token0), Some(token1)) if self.operator.is_some() =>
			{
				self.operator.unwrap().resolve(token0.resolve(rules, known, seen), token1.resolve(rules, known, seen))
			},
			(Some(token), None) if self.operator == None => token.resolve(rules, known, seen),
			(None, Some(token)) if self.operator == None => token.resolve(rules, known, seen),
			_ =>
			{
				eprintln!("expert-system: unable to resolve the following operation due to an internal error: {}", self);
				std::process::exit(1);
			}
		}
	}

	pub fn resolve_as_conclusion_true(&self, /*intials: &I, */rules: &Vec<Rule>, known: &mut HashMap<Fact, Option<bool>>, seen: &mut HashMap<Rule, Vec<Fact>>) -> HashMap<Fact, Option<bool>>// Option<bool>
	{
		// TODO: Handle OR and XOR conclusions
		match self.operator
		{
			Some(op) => match op
			{
				Operators::And => HashMap::from_iter(self.get_facts().iter().map(|&f| (f, Some(true)))),
				_ =>
				{
					eprintln!("expert-system: OR and XOR in conclusion are not supported: {}", self);
					std::process::exit(1);
				}
			},
			_ => HashMap::from_iter(self.get_facts().iter().map(|&f| (f, Some(true))))
		}
	}

	pub fn resolve_as_conclusion_false(&self, /*intials: &I, */rules: &Vec<Rule>, known: &mut HashMap<Fact, Option<bool>>, seen: &mut HashMap<Rule, Vec<Fact>>) -> HashMap<Fact, Option<bool>>// Option<bool>
	{
		// TODO: Handle OR and XOR conclusions
		// Maybe a good idea to have a Vec<Fact> in the Operation struct
		// to be able to quickly get all the facts an operation contains.
		// Also, a FALSE conclusion even with only AND operators in it
		// seems to be a bit more tricky than expected:
		// from my understanding A + B => C + D means "if the expression `A AND B` is FALSE then the experession `C AND D` is FALSE"
		// so C can be TRUE or FALSE as long as D is FALSE and the opposite is true as well.
		// I really hope I'm wrong ! (Otherwise this method is wrong)
		// Actually I'm maybe wrong (that would be great !) and I just need a break because this would lead to an undefined result for one of the fact
		// but the subject of the project explicitly says that we can't have undefined value if we don't support OR and XOR conclusions.
		// If a fact is FALSE and a rule which implies this fact returns FALSE is the fact set to FALSE or its is just not changed.
		// (Read again if you don't see the difference)
		match self.operator
		{
			Some(op) => match op
			{
				Operators::And => HashMap::from_iter(self.get_facts().iter().map(|&f| (f, Some(false)))),
				_ => 
				{
					eprintln!("expert-system: OR and XOR in conclusion are not supported: {}", self);
					std::process::exit(1);
				}
			},
			_ => HashMap::from_iter(self.get_facts().iter().map(|&f| (f, Some(false))))
		}
	}

	pub fn get_facts(&self) -> Vec<Fact>
	{
		// TODO: Implement me
		Vec::new()
	}

	pub fn contains_fact(&self, fact: &Fact) -> bool
	{
		match (self.facts.0.as_ref(), self.facts.1.as_ref())
		{
			(Some(f0), Some(f1)) => f0.contains_fact(fact) || f1.contains_fact(fact),
			(_, _) => false
		}
	}
}

fn capture_parentheses(input: &str)
{

}