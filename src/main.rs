/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tlernoul <tlernoul@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/10/02 19:29:01 by tlernoul          #+#    #+#             */
/*   Updated: 2019/10/11 21:14:04 by tlernoul         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#[macro_use]
extern crate clap;
use clap::{Arg, App};
use std::path::Path;
use std::string;
use std::fs;
use std::io::{Error};


// TODO Error enum for Expert system

struct Rule 
{
    left: Vec<Operation>,
    right: Vec<Operation>,
    middle: Operation,
}

struct Operation
{
    outcome: Result<Outcome, Error>,
    operator: Operators,
    facts: (String, String),
    reverse: (bool, bool),
}

enum Operators
{
    And,
    Or,
    Xor,
    Then,
    IfOnly,
}

enum Outcome
{
    True,
    False,
    Unkn,
}


fn tokenize(facts: Vec<&str>)
{
    let mut oper: Vec<Operation> = Vec::new();

    for f in facts.iter()
    {
        
    }
}


fn parse_and_return(input: String) -> Result<String, Error>
{
    let mut rule: Vec<&str> = Vec::new();

    'outer: for line in input.lines()
    {
        if !(line.find('#') == Some(0)) && line.find(char::is_alphabetic) == Some(0)
        {
            'inner: for c in line.split_whitespace()
            {
                if c.chars().all(char::is_alphabetic) ||
                c.chars().nth(0) == Some('!') && c.chars().nth(1).unwrap().is_alphabetic()
                {
                    rule.push(c);
                }
                if c == "+" || c == "|" || c == "^" || c == "=>" || c == "<=>"
                {
                    rule.push(c);
                }
                if c == "#" { break 'inner; }
            }
            rule.push("\n")
        }
    }
    for a in rule.iter()
    {
        print!("{} ", a);
    }
    println!("");
    tokenize(rule);
    return Ok("truc".into())
}

fn main() 
{
    let matches = App::new("Expert System")
                    .version(crate_version!())
                    .author(crate_authors!())
                    .about(crate_description!())
                    .arg(Arg::with_name("input")
                        .required(true)
                        .help("<file.es> input"))
                    .get_matches();

    let file = Path::new(matches.value_of("input").unwrap());
    let mut content: String = string::String::from("");

    match file.exists()
    {
        true => content = string::String::from(fs::read_to_string(file).unwrap()),
        false => panic!("Invalid input"),
    }
    parse_and_return(content);
    //println!("{}", content);
}
