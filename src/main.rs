/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tlernoul <tlernoul@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/10/02 19:29:01 by tlernoul          #+#    #+#             */
/*   Updated: 2019/10/08 21:43:28 by tlernoul         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#[macro_use]
extern crate clap;
extern crate nom;
use clap::{Arg, App};
use std::path::Path;
use std::string;
use std::fs;
use std::io::{Error};

struct Rule 
{
    Left: Vec<Operator>,
    Right: Vec<Operator>,
    Middle: Vec<Operator>,
}

enum Facts
{
    Res,
    Fact,
}

enum Operator
{
    AND (Fact, Fact),
    OR (Fact, Fact),
    XOR (Fact, Fact),
    THEN,
}

struct Fact
{
    name: String,
    value: bool,
}


fn parse_and_return(input: String) -> Result<String, Error>
{
    for c in input.lines()
    {
        if c.chars().next().unwrap().is_alphanumeric()
        {
            println!("{}", c);
        }
    }
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
