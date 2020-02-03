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
	}
}

impl Eq for Operation { }

impl Hash for Operation
{
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		self.raw.hash(state);
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

    // The value to pass for `priorities` should comes from the get_operators_sorted_by_priority() method.
    // Actually we obviously can get this priority list directly inside this method but I want to avoid doing the same thing
    // multiple times on the same (or part of the same) object. That also reduce the complexity of this method
    pub fn new(input: &str, mut priorities: Vec<(usize, usize, Operators)>) -> Result<Self, String>
    {
        if priorities.is_empty()
        {
            return Err(format!("`{}`: an operation must contains at least 1 operator (`+`, `|` or `^`)", input))
        }
		let (index, priority, operator) = priorities.remove(0);
		let depth = (priority - (operator as usize + 1)) / Operators::get_highest_priority();
        let (left_priorities, mut right_priorities): (Vec<(usize, usize, Operators)>, Vec<(usize, usize, Operators)>) = priorities.iter().partition(|(i, _, _)| *i <= index);
        for (i, _, _) in right_priorities.iter_mut()
        {
			let offset = (index as isize + 1) - depth as isize;
			if offset < 0
			{
				*i += (-offset) as usize
			}
			else
			{
				*i -= offset as usize
			}
		}
		let (left_part, right_part_tmp) = input.split_at(index);
		let mut left_part: String = left_part.into();
		let mut right_part = "(".repeat(depth);
		left_part.push_str(&")".repeat(depth));
		right_part.push_str(&right_part_tmp[1..]);
        Ok(Operation
        {
            operator,
            facts: (
                Box::new(Factoken::new(&left_part, left_priorities)?),
                Box::new(Factoken::new(&right_part, right_priorities)?)
            ),
            raw: input.into()
        })
    }

    pub fn get_operators_sorted_by_priority(input: &str) -> Result<Vec<(usize, usize, Operators)>, String>
    {
        let mut depth: isize = 0;
        let mut priorities: Vec<(usize, usize, Operators)> = Vec::new();
        for (i, c) in input.char_indices()
        {
            match c
            {
                '(' => depth += 1,
                ')' => depth -= 1,
                c if Operators::is_operator(c) => {
                    let op = Operators::new(std::str::from_utf8(&[c as u8]).unwrap_or(""));
                    let op = op.unwrap();
                    priorities.push((i, op.get_priority(depth), op))
                },
                'A'..='Z' | '!' => continue,
                c if c.is_ascii_whitespace() => continue,
                _ => return Err(format!("`{}` in `{}` at pos [{}]: illegal token", c, input, i))
            }
            if depth < 0
            {
                return Err(format!("`{}` at pos [{}]: there is a closing parenthese without an opening one to match it", input, i));
            }
        }
        priorities.sort_by(|(_, p0, _), (_, p1, _)| p0.cmp(p1));
        Ok(priorities)
    }

	pub fn resolve(&self, rules: &Vec<Rule>, known: &mut HashMap<Fact, Option<bool>>, seen: &mut HashMap<Rule, Vec<Fact>>) -> Option<bool>
	{
		self.operator.resolve(self.facts.0.resolve(rules, known, seen), self.facts.1.resolve(rules, known, seen))
	}

	pub fn resolve_as_conclusion(&self, rules: &Vec<Rule>, known: &mut HashMap<Fact, Option<bool>>, seen: &mut HashMap<Rule, Vec<Fact>>, result: bool) -> HashMap<Fact, Option<bool>>
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
			Operators::And => HashMap::from_iter(self.get_facts().iter().map(|&f| {
				(f, Some(if f.is_not() { !result } else { result }))
			})),
			_ =>
			{
				eprintln!("expert-system: OR and XOR in conclusion are not supported: {}", self);
				std::process::exit(1);
			}
		}
	}

	pub fn get_facts(&self) -> Vec<Fact>
	{
		let mut facts = self.facts.0.get_facts();
		facts.append(&mut self.facts.1.get_facts());
		facts
	}

	pub fn contains_fact(&self, fact: &Fact) -> bool
	{
		self.facts.0.contains_fact(fact) || self.facts.1.contains_fact(fact)
	}
}
