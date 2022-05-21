mod list;
mod reader;
use clap::{App, Arg};
use std::io;
use list::Mod;

fn main() {
    let matches = App::new("modsReforged")
        .version("0.1.0")
        .author("Jakob K. <github@jeykey.net>")
        .about("Arma Reforger mod list generator.")
        .arg(
            Arg::with_name("PATH")
                .short("p")
                .long("path")
                .help("Path to the root Reforger Addon directory (documents)")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let path = matches.value_of("PATH").unwrap();
    let path = std::path::Path::new(path);

    let list = reader::read(&path).unwrap();

    println!("{}", format!(" - - - modsReforged - - - "));
    println!("");
    println!("{}", format!("Select mods from list.")); 

    for (i, m) in list.mods.iter().enumerate() {
        let num = format!("{}", i);
        let line = format!("{}: {}", num, m.name);
        println!("{}", line);
    }

    println!("Enter numbers, space separated:");
    let mut numbers = String::new();
    let stdin = io::stdin(); 
    stdin.read_line(&mut numbers).unwrap();
    let numbers = numbers.trim();
    let nums: Vec<usize> = numbers.split(' ').map(|x| {
        x.parse::<usize>().unwrap()
    }).collect();

    let mut filtered: Vec<Mod> = Vec::new();
    for n in nums {
        match list.mods.get(n) {
            Some(m) => {
                filtered.push(m.clone());
            }
            None => {}
        }
    }

    let el = list::ExportList(filtered);

    println!("");
    println!("{}", format!(" - - - Prettified - - - "));
    let j = serde_json::to_string_pretty(&el).unwrap();
    println!("{}", j);
    println!("{}", format!(" - - - Pterodactyl - - - "));
    let j = serde_json::to_string(&el).unwrap();
    println!("{}", j);
}

