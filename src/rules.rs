use crate::*;

pub struct Rule 
{
    left: Vec<Operation>,
    right: Vec<Operation>,
    middle: Operation,
}