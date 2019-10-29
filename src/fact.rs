use crate::rules::Rule;
use std::{
	fmt,
	collections::HashMap, 
	hash::{Hash, Hasher}
};

#[derive(Default, Debug, Eq, Copy, Clone)]
pub struct Fact
{
	name: char,
	not: bool,
}

impl PartialEq for Fact
{
	fn eq(&self, other: &Self) -> bool
	{
		self.name == other.name
	}
}

impl Hash for Fact
{
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		self.name.hash(state);
	}
}

impl fmt::Display for Fact
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		write!(f, "{}", self.name)
	}
}

impl Fact
{
	pub fn new(input: &str) -> Result<Self, String>
	{
		if input.len() == 2
		{
			let not = input.chars().nth(0).unwrap();
			let fact = input.chars().nth(1).unwrap();
			if not == '!' && fact.is_ascii_uppercase()
			{
				Ok(Fact{
					name: fact,
					not: true,
				})
			}
			else
			{
				Err(format!("`{}`: this cannot be a fact", input))
			}
		}
		else
		{
			let fact = input.chars().nth(0).unwrap();
			if fact.is_ascii_uppercase()
			{
				Ok(Fact{
					name: fact,
					not: false,
				})
			}
			else
			{
				Err(format!("`{}`: this cannot be a fact", input))
			}
		}
	}

	// pub fn resolve(&self, intials: &I, rules: &T) -> Option<bool>
	// where
	// 	I: Iterator<Item=Fact>,
	// 	T: Iterator<Item=Rule> + DoubleEndedIterator<Item=Rule>
	// {
	// 	// We probably should have a list of the Fact which are already known as TRUE
	// 	// and verify in this list if we have a value for the Fact we're trying to resolve

	// }

	pub fn is_not(&self) -> bool
	{
		self.not
	}

	pub fn resolve/*<T>*/(&self, /*intials: &I, */rules: &Vec<Rule>, known: &mut HashMap<Fact, Option<bool>>, seen: &mut HashMap<Rule, Vec<Fact>>) -> Option<bool>
	// where
	// 	T: Iterator<Item=Rule> + DoubleEndedIterator<Item=Rule>
	{
		// We probably should have a list of the Fact which are already known as TRUE
		// and verify in this list if we have a value for the Fact we're trying to resolve
		//
		// This is probably here we should check for infinite loop using a list of rules that we
		// already seen. If we try to resolve again a rule which we have already seen before when trying to resolve
		// the same Fact we should just skip it. At this point if there is non more rules that implies this Fact
		// and none of the previous ones make it TRUE then the Fact is false
		let mut result: Option<bool> = Some(false);
		let mut is_result_assigned = false;
		// if intials.any(|ref f| f == self)
		// {
		// 	return Some(true);
		// }
		if let Some(&value) = known.get(self)
		{
			return value;
		}
		for rule in rules.iter().filter(|r| r.contains_fact_as_conclusion(self)).rev()
		{
			if rule.has_been_seen_for_fact(seen, self)
			{
				continue;
			}
			seen.entry(rule.clone()).or_insert(Vec::new()).push(*self);
			for (fact, value) in rule.resolve(rules, known, seen)
			{
				if fact == *self && (!is_result_assigned || result.is_none() || (result == Some(false) && value.is_some()))
				{
					result = value;
					is_result_assigned = true;
				}
				let entry = known.entry(fact).or_insert(value);
				if entry.is_none() || (*entry == Some(false) && value.is_some())
				{
					*entry = value;
				}
			}
			if result == Some(true)
			{
				break;
			}
			// let value = rule.resolve(rules, known, seen);
			// if let Some(true) = value
			// {
			// 	result = value;
			// 	*known.entry(*self).or_insert(true) = true;
			// 	break;
			// }
			// else if result.is_none()
			// {
			// 	result = value;
			// }
		}
		result
	}
}

#[cfg(test)]
mod test_fact
{
	use crate::fact::Fact;

	#[test]
	fn test_new()
	{
		let a = Fact::new("!");
		let b = Fact::new("AB");
		let c = Fact::new("a");
		let d = Fact::new("aB");
		let e = Fact::new("!a");
		let f = Fact::new("A");
		let g = Fact::new("!A");

		assert_eq!(Err("`!`: this cannot be a fact".into()), a);
		assert_eq!(Err("`AB`: this cannot be a fact".into()), b);
		assert_eq!(Err("`a`: this cannot be a fact".into()), c);
		assert_eq!(Err("`aB`: this cannot be a fact".into()), d);
		assert_eq!(Err("`!a`: this cannot be a fact".into()), e);
		assert!(f.is_ok());
		let f = f.unwrap();
		assert!(f.name == 'A' && !f.not);
		assert!(g.is_ok());
		let g = g.unwrap();
		assert!(g.name == 'A' && g.not);
	}

	#[test]
	fn test_eq()
	{
		let a = Fact{
			name: 'A',
			not: true
		};
		let b = Fact{
			name: 'A',
			not: false
		};
		let c = Fact{
			name: 'B',
			not: false
		};

		assert!(a == b);
		assert!(b != c);
	}

	#[test]
	fn test_resolve()
	{

	}
}