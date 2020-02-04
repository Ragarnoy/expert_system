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
use std::{path::Path, fs, io::Error, collections::HashMap, iter::FromIterator};
use fact::Fact;
use operators::Operators; 
use operation::Operation;
use token::Factoken;
use rules::Rule;

fn get_facts_in_line(input: &str, output: &mut Vec<Fact>) -> Result<(), String>
{
	for c in input.chars()
	{
		if !c.is_ascii_whitespace()
		{
			output.push(Fact::new(&c.to_string())?)
		}
	}
	Ok(())
}

fn parse(input: String) -> Result<(Vec<Rule>, Vec<Fact>, Vec<Fact>), String>
{
	let mut rules: Vec<Rule> = Vec::new();
	let mut initials: Vec<Fact> = Vec::new();
	let mut queries: Vec<Fact> = Vec::new();
	let mut initials_mark = false;

	for line in input.lines().filter_map(|line| {
		let line = line.trim();
		if line.starts_with('#') || line.is_empty()
		{
			return None;
		}
		if let Some((index, _)) = line.match_indices('#').next()
		{
			return Some(line[..index].into());
		}
		Some(line)
	})
	{
		// This case probably makes no sense
		let line = line.trim();
		if line.is_empty()
		{
			continue;
		}
		match &line[..1]
		{
			"=" => {
				get_facts_in_line(&line[1..], &mut initials)?;
				initials_mark = true;
			},
			"?" => get_facts_in_line(&line[1..], &mut queries)?,
			_ => rules.push(Rule::new(line)?)
		}
	}
	if rules.is_empty()
	{
		return Err("the input file must contains at least one rule - e.g: `A + B => C`".into());
	}
	if queries.is_empty()
	{
		return Err("the input file must contains at least one query - e.g: `?C` to request the value of `C` or `?AC` to request the value of both `A` and `C`".into());
	}
	if !initials_mark
	{
		return Err("the input file must contains at least an initial facts mark `=` - e.g: `=` to declare all facts as FALSE or `=AB` to declare both `A` and `B` as TRUE".into());
	}
	Ok((rules, initials, queries))
}

fn resolve(rules: Vec<Rule>, initials: Vec<Fact>, queries: Vec<Fact>) -> String
{
	let mut seen: HashMap<Rule, Vec<Fact>> = HashMap::new();
	let mut initials: HashMap<Fact, Option<bool>> = HashMap::from_iter(initials.iter().map(|&f| (f, Some(true))));
	let mut result = String::new();

	for fact in queries
	{
		match fact.resolve(&rules, &mut initials, &mut seen)
		{
			Some(value) => result.push_str(&format!("{} -> {}\n", fact, value)),
			None => result.push_str(&format!("{} -> undefined\n", fact))
		}
	}
	result
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
	if !content.is_ascii()
	{
		eprintln!("expert-system: parsing error: the input file MUST contains only ASCII characters");
	}

    match parse(content)
    {
		Ok((rules, initials, queries)) => println!(
			"Parsing has successfully ended.\nHere is the result of your queries:\n\n{}",
			resolve(rules, initials, queries)
		),
        Err(error) => eprintln!("expert-system: parsing error: {}", error)
    }
}
