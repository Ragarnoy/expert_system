use crate::{token::Factoken, operators::Operators};

#[derive(Debug, Default)]
pub struct Operation
{
    operator: Operators,
    facts: (Box<Factoken>, Box<Factoken>),
	raw: String
}

impl Operation
{
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
}
