extern crate argparse;
mod uc_table;

use std::u32;
use std::char;
use argparse::{ArgumentParser, Store, StoreTrue, Collect};

fn describe(c: &uc_table::UCEntry) -> &'static str {
    if c.na.len() > 0 {
        return c.na;
    }
    if c.na1.len() > 0 {
        return c.na1;
    }
    return "";
}

fn parse_int(codepoint_str : &str) -> Option<u32> {
    let maybe_cp = match codepoint_str.chars().nth(0) {
        Some('x') => {
            let (_, sliced) = codepoint_str.split_at(1);
            u32::from_str_radix(sliced, 16)
        },
        Some('o') => {
            let (_, sliced) = codepoint_str.split_at(1);
            u32::from_str_radix(sliced, 8)
        },
        _ => codepoint_str.parse::<u32>(),
    };
    return match maybe_cp {
        Ok(n) => Some(n),
        _ => None,
    }
}

fn search(search_str: &str) {
    let uc_table = uc_table::get_uc_table();

    let upper_arg = search_str.to_uppercase();
    for c in &uc_table {
        if c.na1.find(&upper_arg).is_some() || c.na.find(&upper_arg).is_some() {
            println!("{}\t{:#x}\t{}\t{}", c.c, c.cp, c.cp, describe(c));
        }
    }
}

fn codepoint_str_lookup(codepoint_str: &str) {
    match parse_int(codepoint_str) {
        Some(n) => codepoint_lookup(n),
        None => println!("Malformed integer: {}", codepoint_str),
    };
}

fn print_codepoint(codepoint_str: &str) {
    match parse_int(codepoint_str) {
        Some(n) => match char::from_u32(n) {
            Some(c) => print!("{}", c),
            None => println!("Bad codepoint: {}", codepoint_str),
        },
        None => println!("Malformed integer: {}", codepoint_str),
    };
}

fn codepoint_lookup(cp: u32) {
    let uc_table = uc_table::get_uc_table();
    for c in &uc_table {
        if c.cp == cp {
            println!("{}\t{:#x}\t{}\t{}", c.c, c.cp, c.cp, describe(c));
        }
    }
}

fn main() {
    let mut search_str = "".to_string();
    let mut codepoint_strs : Vec<String> = vec![];
    let mut search_ch = "".to_string();
    let mut transcribe = false;

    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut search_str)
            .add_option(&["-s", "--search"], Store,
            "Search for a character by description");
        ap.refer(&mut search_ch)
            .add_option(&["-d", "--describe"], Store,
            "Describe characters");
        ap.refer(&mut transcribe)
            .add_option(&["-t", "--transcribe"], StoreTrue,
            "Convert codepoints to characters");
        ap.refer(&mut codepoint_strs)
            .add_argument("codepoint", Collect,
            "Codepoint to describe. Prefix with x/o for hex/octal.");
        ap.parse_args_or_exit();
    }

    if search_str.len() > 0 {
        search(search_str.as_str());
    }

    if search_ch.len() > 0 {
        for c in search_ch.chars() {
            codepoint_lookup(c as u32);
        }
    }

    if codepoint_strs.len() > 0 {
        if transcribe {
            for codepoint_str in codepoint_strs {
                print_codepoint(codepoint_str.as_str());
            }
        }
        else {
            for codepoint_str in codepoint_strs {
                codepoint_str_lookup(codepoint_str.as_str());
            }
        }
    }
}
