extern crate argparse;
mod uc_table;

use std::u32;
use argparse::{ArgumentParser, Store};

fn describe(c: &uc_table::UCEntry) -> &'static str {
    if c.na.len() > 0 {
        return c.na;
    }
    if c.na1.len() > 0 {
        return c.na1;
    }
    return "";
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

    if maybe_cp.is_ok() {
        codepoint_lookup(maybe_cp.unwrap());
    }
    else {
        println!("Malformed integer: {}", codepoint_str);
    }
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
    let mut codepoint_str = "".to_string();
    let mut search_ch = "".to_string();
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut search_str)
            .add_option(&["-s", "--search"], Store,
            "Search for a character by description");
        ap.refer(&mut search_ch)
            .add_option(&["-d", "--describe"], Store,
            "Describe characters");
        ap.refer(&mut codepoint_str)
            .add_argument("codepoint", Store,
            "Codepoint to describe. Prefix with x/o for hex/octal.");
        ap.parse_args_or_exit();
    }

    if search_str.len() > 0 {
        search(search_str.as_str());
    }

    if codepoint_str.len() > 0 {
        codepoint_str_lookup(codepoint_str.as_str());
    }

    if search_ch.len() > 0 {
        for c in search_ch.chars() {
            codepoint_lookup(c as u32);
        }
    }
}
