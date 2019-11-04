use crate::{operators::Operators, token::Factoken};

#[derive(Default, Debug)]
pub struct Rule 
{
    pub left: Factoken,
    pub right: Factoken,
    pub middle: Operators,
}
