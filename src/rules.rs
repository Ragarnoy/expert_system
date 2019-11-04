use crate::{operation::Operation, operators::Operators};

#[derive(Debug)]
pub struct Rule 
{
    left: Operation,
    right: Operation,
    middle: Operators,
}

impl Default for Rule
{
    fn default() -> Self
    {
        Rule
        {
            left: Operation::default(),
            right: Operation::default(),
            middle: Operators::Then,
        }
    }
}
