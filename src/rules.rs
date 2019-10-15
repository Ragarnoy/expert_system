use crate::*;

#[derive(Default, Debug)]
pub struct Rule 
{
    left: Operation,
    right: Operation,
    middle: Operation,
}