#[derive(Default, Debug)]
pub struct Fact
{
    name: char,
    not: bool,
}

impl Fact
{
    fn new(input: &str) -> Self
    {
        if input.len() == 2
        {
            Fact{
                name: input.chars().nth(1).unwrap(),
                not: true,
            }
        }
        else
        {
            Fact{
                name: input.chars().nth(0).unwrap(),
                not: false,
            }
        }
    }
}
