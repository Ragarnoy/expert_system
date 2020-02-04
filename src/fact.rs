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
		let input_no_ws = input.replace(|c: char| c.is_ascii_whitespace(), "");
		let chars = input_no_ws.char_indices();
		let mut depth: isize = 0;
		let mut not: Option<(usize, bool)> = None;
		let mut fact: Option<char> = None;

		for (i, c) in chars
		{
			match c
			{
				'(' => depth += 1,
				')' => depth -= 1,
				'!' => match not {
					Some((_, prev_v)) if fact.is_none()/*if prev_i + 1 == i */=> not = Some((i, !prev_v)),
					None if fact.is_none() => not = Some((i, true)),
					_ if fact.is_some() => return Err(format!("`{}`: a fact cannot be declared with a NOT (`!`) after its name", input)),
					_ => return Err(format!("`{}`: NOT (`!`) is allowed only next to (excluding whitespaces) another NOT, a fact's name or an enparenthesed operation", input))
				},
				c if c.is_ascii_uppercase() => match not {
					// Some((index, _)) /*if index + 1 != i*/ => return Err(format!("`{}`: NOT (`!`) is allowed only next to (excluding whitespaces) another NOT, a fact's name or an enparenthesed operation", input)),
					_ if fact.is_some() => return Err(format!("`{}`: declare more than 1 fact here is forbidden", input)),
					_ => fact = Some(c)
				},
				_ => return Err(format!("`{}` in `{}`: illegal token", c, input))
			}
		}
		if depth != 0
		{
			return Err(format!("`{}`: mismatching parentheses", input));
		}
		if let Some(name) = fact
		{
			Ok(Self
			{
				name,
				not: not.unwrap_or((0, false)).1
			})
		}
		else
		{
			Err(format!("`{}`: we should have a fact around here", input))
		}
	}

	pub fn is_not(&self) -> bool
	{
		self.not
	}

	pub fn resolve(&self, rules: &Vec<Rule>, known: &mut HashMap<Fact, Option<bool>>, seen: &mut HashMap<Rule, Vec<Fact>>) -> Option<bool>
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
			for (fact, mut value) in rule.resolve(rules, known, seen)
			{
				if fact == *self && (!is_result_assigned || result.is_none() || (result == Some(false) && value.is_some()))
				{
					if self.is_not() && value.is_some()
					{
						value = Some(!value.unwrap());
					}
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
		}
		result
	}
}

#[cfg(test)]
mod test_fact
{
	use crate::{fact::Fact, token::Factoken, operation::Operation, operators::Operators, rules::Rule};
	use std::collections::HashMap;

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

		assert_eq!(Err("`!`: we should have a fact around here".into()), a);
		assert_eq!(Err("`AB`: declare more than 1 fact here is forbidden".into()), b);
		assert_eq!(Err("`a` in `a`: illegal token".into()), c);
		assert_eq!(Err("`a` in `aB`: illegal token".into()), d);
		assert_eq!(Err("`a` in `!a`: illegal token".into()), e);
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
		// A | B + C => E
		// (F | G) + H => E
		let rules = vec!(Rule::new("A | B + C => E").unwrap(), Rule::new("(F | G) + H => E").unwrap());

		// ?E
		let query = Fact {name: 'E', not: false};

		// =A -> E should be TRUE
		let mut known: HashMap<Fact, Option<bool>> = HashMap::new();
		known.insert(Fact{name: 'A', not: false}, Some(true));
		let result = query.resolve(&rules, &mut known, &mut HashMap::new());
		assert!(result.is_some());
		assert!(result.unwrap());

		// =B -> E should be FALSE
		let mut known: HashMap<Fact, Option<bool>> = HashMap::new();
		known.insert(Fact{name: 'B', not: false}, Some(true));
		let result = query.resolve(&rules, &mut known, &mut HashMap::new());
		assert!(result.is_some());
		assert!(!result.unwrap());

		// =C -> E should be FALSE
		let mut known: HashMap<Fact, Option<bool>> = HashMap::new();
		known.insert(Fact{name: 'C', not: false}, Some(true));
		let result = query.resolve(&rules, &mut known, &mut HashMap::new());
		assert!(result.is_some());
		assert!(!result.unwrap());

		// =AC -> E should be TRUE
		let mut known: HashMap<Fact, Option<bool>> = HashMap::new();
		known.insert(Fact{name: 'A', not: false}, Some(true));
		known.insert(Fact{name: 'C', not: false}, Some(true));
		let result = query.resolve(&rules, &mut known, &mut HashMap::new());
		assert!(result.is_some());
		assert!(result.unwrap());

		// =BC -> E should be TRUE
		let mut known: HashMap<Fact, Option<bool>> = HashMap::new();
		known.insert(Fact{name: 'B', not: false}, Some(true));
		known.insert(Fact{name: 'C', not: false}, Some(true));
		let result = query.resolve(&rules, &mut known, &mut HashMap::new());
		assert!(result.is_some());
		assert!(result.unwrap());


		// =F -> E should be FALSE
		let mut known: HashMap<Fact, Option<bool>> = HashMap::new();
		known.insert(Fact{name: 'F', not: false}, Some(true));
		let result = query.resolve(&rules, &mut known, &mut HashMap::new());
		assert!(result.is_some());
		assert!(!result.unwrap());

		// =G -> E should be FALSE
		let mut known: HashMap<Fact, Option<bool>> = HashMap::new();
		known.insert(Fact{name: 'G', not: false}, Some(true));
		let result = query.resolve(&rules, &mut known, &mut HashMap::new());
		assert!(result.is_some());
		assert!(!result.unwrap());

		// =H -> E should be FALSE
		let mut known: HashMap<Fact, Option<bool>> = HashMap::new();
		known.insert(Fact{name: 'H', not: false}, Some(true));
		let result = query.resolve(&rules, &mut known, &mut HashMap::new());
		assert!(result.is_some());
		assert!(!result.unwrap());

		// =FH -> E should be TRUE
		let mut known: HashMap<Fact, Option<bool>> = HashMap::new();
		known.insert(Fact{name: 'F', not: false}, Some(true));
		known.insert(Fact{name: 'H', not: false}, Some(true));
		let result = query.resolve(&rules, &mut known, &mut HashMap::new());
		assert!(result.is_some());
		assert!(result.unwrap());

		// =GH -> E should be TRUE
		let mut known: HashMap<Fact, Option<bool>> = HashMap::new();
		known.insert(Fact{name: 'G', not: false}, Some(true));
		known.insert(Fact{name: 'H', not: false}, Some(true));
		let result = query.resolve(&rules, &mut known, &mut HashMap::new());
		assert!(result.is_some());
		assert!(result.unwrap());
	}

	#[test]
	fn test_resolve_with_fact_not_in_operation()
	{
		// !A => B
		let rules = vec!(Rule::new("!A => B").unwrap());

		// ?B
		let query = Fact {name: 'B', not: false};

		// =A -> B should be FALSE
		let mut known: HashMap<Fact, Option<bool>> = HashMap::new();
		known.insert(Fact{name: 'A', not: false}, Some(true));
		let result = query.resolve(&rules, &mut known, &mut HashMap::new());
		assert!(result.is_some());
		assert!(!result.unwrap());

		// = -> B should be TRUE
		let mut known: HashMap<Fact, Option<bool>> = HashMap::new();
		let result = query.resolve(&rules, &mut known, &mut HashMap::new());
		assert!(result.is_some());
		assert!(result.unwrap());
	}

	#[test]
	fn test_resolve_with_fact_not_in_conclusion()
	{
		// A => !B
		let rules = vec!(Rule::new("A => !B").unwrap());

		// ?B
		let query = Fact {name: 'B', not: false};

		// =A -> B should be FALSE
		let mut known: HashMap<Fact, Option<bool>> = HashMap::new();
		known.insert(Fact{name: 'A', not: false}, Some(true));
		let result = query.resolve(&rules, &mut known, &mut HashMap::new());
		assert!(result.is_some());
		assert!(!result.unwrap());

		// = -> B should be TRUE
		let mut known: HashMap<Fact, Option<bool>> = HashMap::new();
		let result = query.resolve(&rules, &mut known, &mut HashMap::new());
		assert!(result.is_some());
		assert!(result.unwrap());
	}

	#[test]
	fn test_resolve_with_multi_fact_in_conclusion()
	{
		// A => B + C + D
		let rules = vec!(Rule::new("A => B + C + D").unwrap());

		// ?B
		let query_b = Fact {name: 'B', not: false};
		// ?C
		let query_c = Fact {name: 'C', not: false};
		// ?D
		let query_d = Fact {name: 'D', not: false};

		// =A -> B, C and D should be TRUE
		let mut known: HashMap<Fact, Option<bool>> = HashMap::new();
		known.insert(Fact{name: 'A', not: false}, Some(true));
		let result = query_b.resolve(&rules, &mut known, &mut HashMap::new());
		assert!(result.is_some());
		assert!(result.unwrap());
		assert!(known.contains_key(&Fact {name: 'C', not: false}));
		assert!(known.get(&Fact {name: 'C', not: false}).unwrap().unwrap());
		assert!(known.contains_key(&Fact {name: 'D', not: false}));
		assert!(known.get(&Fact {name: 'D', not: false}).unwrap().unwrap());
		let result = query_c.resolve(&rules, &mut known, &mut HashMap::new());
		assert!(result.is_some());
		assert!(result.unwrap());
		assert!(known.contains_key(&Fact {name: 'B', not: false}));
		assert!(known.get(&Fact {name: 'B', not: false}).unwrap().unwrap());
		assert!(known.contains_key(&Fact {name: 'D', not: false}));
		assert!(known.get(&Fact {name: 'D', not: false}).unwrap().unwrap());
		let result = query_d.resolve(&rules, &mut known, &mut HashMap::new());
		assert!(result.is_some());
		assert!(result.unwrap());
		assert!(known.contains_key(&Fact {name: 'B', not: false}));
		assert!(known.get(&Fact {name: 'B', not: false}).unwrap().unwrap());
		assert!(known.contains_key(&Fact {name: 'C', not: false}));
		assert!(known.get(&Fact {name: 'C', not: false}).unwrap().unwrap());

		// = -> B, C and D should be FALSE
		let mut known: HashMap<Fact, Option<bool>> = HashMap::new();
		let result = query_b.resolve(&rules, &mut known, &mut HashMap::new());
		assert!(result.is_some());
		assert!(!result.unwrap());
		assert!(known.contains_key(&Fact {name: 'C', not: false}));
		assert!(!known.get(&Fact {name: 'C', not: false}).unwrap().unwrap());
		assert!(known.contains_key(&Fact {name: 'D', not: false}));
		assert!(!known.get(&Fact {name: 'D', not: false}).unwrap().unwrap());
		let result = query_c.resolve(&rules, &mut known, &mut HashMap::new());
		assert!(result.is_some());
		assert!(!result.unwrap());
		assert!(known.contains_key(&Fact {name: 'B', not: false}));
		assert!(!known.get(&Fact {name: 'B', not: false}).unwrap().unwrap());
		assert!(known.contains_key(&Fact {name: 'D', not: false}));
		assert!(!known.get(&Fact {name: 'D', not: false}).unwrap().unwrap());
		let result = query_d.resolve(&rules, &mut known, &mut HashMap::new());
		assert!(result.is_some());
		assert!(!result.unwrap());
		assert!(known.contains_key(&Fact {name: 'B', not: false}));
		assert!(!known.get(&Fact {name: 'B', not: false}).unwrap().unwrap());
		assert!(known.contains_key(&Fact {name: 'C', not: false}));
		assert!(!known.get(&Fact {name: 'C', not: false}).unwrap().unwrap());
	}

	#[test]
	fn test_resolve_with_multi_fact_and_not_in_conclusion()
	{
		// A => B + !C + D
		let rules = vec!(Rule::new("A => B + !C + D").unwrap());

		// ?B
		let query_b = Fact {name: 'B', not: false};
		// ?C
		let query_c = Fact {name: 'C', not: false};
		// ?D
		let query_d = Fact {name: 'D', not: false};

		// =A -> B and D should be TRUE, C should be FALSE
		let mut known: HashMap<Fact, Option<bool>> = HashMap::new();
		known.insert(Fact{name: 'A', not: false}, Some(true));
		let result = query_b.resolve(&rules, &mut known, &mut HashMap::new());
		assert!(result.is_some());
		assert!(result.unwrap());
		assert!(known.contains_key(&Fact {name: 'C', not: false}));
		assert!(!known.get(&Fact {name: 'C', not: false}).unwrap().unwrap());
		assert!(known.contains_key(&Fact {name: 'D', not: false}));
		assert!(known.get(&Fact {name: 'D', not: false}).unwrap().unwrap());
		let result = query_c.resolve(&rules, &mut known, &mut HashMap::new());
		assert!(result.is_some());
		assert!(!result.unwrap());
		assert!(known.contains_key(&Fact {name: 'B', not: false}));
		assert!(known.get(&Fact {name: 'B', not: false}).unwrap().unwrap());
		assert!(known.contains_key(&Fact {name: 'D', not: false}));
		assert!(known.get(&Fact {name: 'D', not: false}).unwrap().unwrap());
		let result = query_d.resolve(&rules, &mut known, &mut HashMap::new());
		assert!(result.is_some());
		assert!(result.unwrap());
		assert!(known.contains_key(&Fact {name: 'B', not: false}));
		assert!(known.get(&Fact {name: 'B', not: false}).unwrap().unwrap());
		assert!(known.contains_key(&Fact {name: 'C', not: false}));
		assert!(!known.get(&Fact {name: 'C', not: false}).unwrap().unwrap());

		// = -> B and D should be FALSE, C should be TRUE
		let mut known: HashMap<Fact, Option<bool>> = HashMap::new();
		let result = query_b.resolve(&rules, &mut known, &mut HashMap::new());
		assert!(result.is_some());
		assert!(!result.unwrap());
		assert!(known.contains_key(&Fact {name: 'C', not: false}));
		assert!(known.get(&Fact {name: 'C', not: false}).unwrap().unwrap());
		assert!(known.contains_key(&Fact {name: 'D', not: false}));
		assert!(!known.get(&Fact {name: 'D', not: false}).unwrap().unwrap());
		let result = query_c.resolve(&rules, &mut known, &mut HashMap::new());
		assert!(result.is_some());
		assert!(result.unwrap());
		assert!(known.contains_key(&Fact {name: 'B', not: false}));
		assert!(!known.get(&Fact {name: 'B', not: false}).unwrap().unwrap());
		assert!(known.contains_key(&Fact {name: 'D', not: false}));
		assert!(!known.get(&Fact {name: 'D', not: false}).unwrap().unwrap());
		let result = query_d.resolve(&rules, &mut known, &mut HashMap::new());
		assert!(result.is_some());
		assert!(!result.unwrap());
		assert!(known.contains_key(&Fact {name: 'B', not: false}));
		assert!(!known.get(&Fact {name: 'B', not: false}).unwrap().unwrap());
		assert!(known.contains_key(&Fact {name: 'C', not: false}));
		assert!(known.get(&Fact {name: 'C', not: false}).unwrap().unwrap());
	}
}