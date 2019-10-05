/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tlernoul <tlernoul@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/10/02 19:29:01 by tlernoul          #+#    #+#             */
/*   Updated: 2019/10/05 17:59:09 by tlernoul         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#[macro_use]
extern crate clap;
use clap::{Arg, App};
use std::path::Path;
use std::string;
use std::fs;

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
    println!("{}", content);
}
