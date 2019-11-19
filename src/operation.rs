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
        let (index, _, operator) = priorities.remove(0);
        let (left_priorities, mut right_priorities): (Vec<(usize, usize, Operators)>, Vec<(usize, usize, Operators)>) = priorities.iter().partition(|(i, _, _)| *i <= index);
        for (ref mut i, _, _) in right_priorities
        {
            *i -= index + 1;
        }
        let (left_part, right_part) = input.split_at(index);
        Ok(Operation
        {
            operator,
            facts: (
                Box::new(Factoken::new(left_part, left_priorities)?),
                Box::new(Factoken::new(right_part[1..], right_priorities)?)
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
                    priorities.push((i, op.get_priority(depth).unwrap(), op))
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
