#[derive(Default, Debug)]
pub struct Fact
{
    name: char,
    not: bool,
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
					Some((prev_i, prev_v)) if prev_i + 1 == i => not = Some((i, !prev_v)),
					None if fact.is_some() => return Err(format!("`{}`: a fact cannot be declared with a NOT (`!`) after its name", input)),
					None => not = Some((i, true)),
					_ => return Err(format!("`{}`: NOT (`!`) is allowed only next to (excluding whitespaces) another NOT or a fact's name", input))
				},
				c if c.is_ascii_uppercase() => match not {
					Some((index, _)) if index + 1 != i => return Err(format!("`{}`: NOT (`!`) is allowed only next to (excluding whitespaces) another NOT or a fact's name", input)),
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
}
