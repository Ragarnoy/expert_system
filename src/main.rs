/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tlernoul <tlernoul@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/10/02 19:29:01 by tlernoul          #+#    #+#             */
/*   Updated: 2019/10/15 18:39:39 by tlernoul         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#[macro_use]
extern crate clap;

mod fact;
mod operators; 
mod operation;
mod token;
mod rules;

use clap::{Arg, App};
use std::{path::Path, fs, io::Error};
use operation::Operation;


// TODO Error enum for Expert system

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
            'inner: for str in line.split_whitespace()
            {
                if str.chars().all(char::is_alphabetic) && str.len() == 1 ||
                str.chars().nth(0) == Some('!') && str.chars().nth(1).unwrap().is_alphabetic() && str.len() == 2
                {
                    rule.push(str);
                }
                if str == "+" || str == "|" || str == "^" || str == "=>" || str == "<=>"
                {
                    rule.push(str);
                }
                if str == "#" { break 'inner; }
            }
            rule.push("\n")
        }
    }
    print!(" ");
    for a in rule.iter()
    {
        print!("{} ", a);
    }
    println!("");
    //tokenize(rule);
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

    let content = fs::read_to_string(file).unwrap();

    match parse_and_return(content)
    {
        Ok(ret) => 1,
        Err(error) => 0,
    };
    //println!("{}", content);
}
