extern crate argparse;
extern crate atty;

mod uc_table;
mod uc_blocks;

use std::fs::File;
use std::io::Read;
use std::io;
use std::u32;
use std::char;
use std::process::exit;
use argparse::{ArgumentParser, StoreOption, StoreTrue, Collect};

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
        Some('0') => {
            let (_, sliced) = codepoint_str.split_at(1);
            return parse_int(sliced);
        }
        Some('x') => {
            let (_, sliced) = codepoint_str.split_at(1);
            u32::from_str_radix(sliced, 16)
        },
        Some('o') => {
            let (_, sliced) = codepoint_str.split_at(1);
            u32::from_str_radix(sliced, 8)
        },
        Some('b') => {
            let (_, sliced) = codepoint_str.split_at(1);
            u32::from_str_radix(sliced, 2)
        },
        _ => codepoint_str.parse::<u32>(),
    };
    return match maybe_cp {
        Ok(n) => Some(n),
        _ => None,
    }
}

fn search_keyword(uc_block: &[uc_table::UCEntry], search_str: &str) {
    let upper_arg = search_str.to_uppercase();
    for c in uc_block {
        if search_str == "*" || c.na1.find(&upper_arg).is_some() || c.na.find(&upper_arg).is_some() {
            println!("{}\t{:#x}\t{}\t{}", c.c, c.cp, c.cp, describe(c));
        }
    }
}

fn codepoint_str_lookup(uc_block: &[uc_table::UCEntry], codepoint_str: &str) {
    match parse_int(codepoint_str) {
        Some(n) => codepoint_lookup(uc_block, n),
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

fn codepoint_lookup(uc_block: &[uc_table::UCEntry], cp: u32) {
    for c in uc_block {
        if c.cp == cp {
            println!("{}\t{:#x}\t{}\t{}", c.c, c.cp, c.cp, describe(c));
        }
    }
}

fn print_highlighted(str: &str) {
    for c in str.chars() {
        let c_u32 = c as u32;
        if c_u32 > 127 {
            // 1:  bold
            // 37: white foreground
            // 97: axiterm bright white forground
            // 45: magenta background
            print!("\x1b[1;37;97;45m");

            if c_u32 == 0x200b { print!("<zero-width-space>"); }
            else if c_u32 == 0x200c { print!("<zero-width-non-joiner>"); }
            else if c_u32 == 0x200d { print!("<zero-width-joiner>"); }
            else if c_u32 == 0xfeff { print!("<zero-width-no-break-space>"); }
            else { print!("{}", c); }
            print!("\x1b[0m");
        }
        else {
            print!("{}", c);
        }
    }
}

fn maybe_read_streams(file: Option<String>) -> Option<String> {
    if !atty::is(atty::Stream::Stdin) {
        let mut contents = String::from("");
        if io::stdin().read_to_string(&mut contents).is_ok() {
            return Some(contents);
        }
    }
    else if file.is_some() {
        match File::open(file.as_ref().unwrap()) {
            Ok(mut reader) => {
                let mut contents = String::from("");
                if reader.read_to_string(&mut contents).is_ok() {
                    return Some(contents);
                }
            },
            Err(e) => {
                eprintln!("Could not read: {}", file.as_ref().unwrap());
                eprintln!("{}", e);
            }
        };
    }
    return None;
}

fn main() {
    let mut positional_args : Vec<String> = vec![];

    let mut transcribe = false;
    let mut decode = true;
    let mut identify = false;
    let mut search = false;
    let mut list = false;
    let mut highlight = false;

    let mut ascii = false;
    let mut emoji = false;
    let mut start : u32 = 0;
    let mut end : u32 = 0;
    let mut block_arg : Option<String> = None;

    let mut file : Option<String> = None;

    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut decode)
            .add_option(&["-d", "--decode"], StoreTrue,
            "Decode codepoints (default)");
        ap.refer(&mut transcribe)
            .add_option(&["-t", "--transcribe"], StoreTrue,
            "Convert codepoints to characters");
        ap.refer(&mut identify)
            .add_option(&["-i", "--identify"], StoreTrue,
            "Identify characters");
        ap.refer(&mut search)
            .add_option(&["-s", "--search"], StoreTrue,
            "Search for a character by description");
        ap.refer(&mut highlight)
            .add_option(&["-h", "--highlight"], StoreTrue,
            "Highlight non-ascii characters");
        ap.refer(&mut file)
            .add_option(&["-f", "--file"], StoreOption,
            "Specify file");
        ap.refer(&mut list)
            .add_option(&["-l", "--list-blocks"], StoreTrue,
            "List known blocks");
        ap.refer(&mut block_arg)
            .add_option(&["-b", "--block"], StoreOption,
            "Specify a named block");
        ap.refer(&mut ascii)
            .add_option(&["--ascii"], StoreTrue,
            "Consider only the ASCII block");
        ap.refer(&mut emoji)
            .add_option(&["--emoji"], StoreTrue,
            "Consider only the emoji block");
        ap.refer(&mut positional_args)
            .add_argument("args", Collect,
            "Arguments to the selected operation");
        ap.parse_args_or_exit();
    }

    if list {
        for block in uc_blocks::get_uc_blocks() {
            println!("{:7x}{:7x}  {:40}\t{}", block.start, block.end, block.tex_name, block.description);
        }
        return;
    }

    match maybe_read_streams(file) {
        Some(contents) => positional_args.push(contents),
        None => {}
    }

    match block_arg {
        Some(block_name) => {
            match uc_blocks::get_uc_block(block_name.as_str()) {
                Some(block) => {
                    start = block.start;
                    end = block.end;
                }
                None => {
                    eprintln!("Unknown block: {}", block_name);
                    exit(1);
                }
            };
        },
        None => {}
    };

    if ascii {
        start = 0u32;
        end = 255u32;
    }
    else if emoji {
        start = 0x1F300u32;
        end = 0x1F64Fu32;
    }

    let uc_block = uc_table::get_uc_table(start, end);

    if search {
        for arg in &positional_args {
            search_keyword(uc_block, arg.as_str());
        }
    }
    else if identify {
        for arg in &positional_args {
            for c in arg.chars() {
                codepoint_lookup(uc_block, c as u32);
            }
        }
    }
    else if transcribe {
        for codepoint_str in &positional_args {
            print_codepoint(codepoint_str.as_str());
        }
    }
    else if highlight {
        for str in &positional_args {
            print_highlighted(str);
        }
    }
    else if decode && positional_args.len() > 0 {
        for codepoint_str in &positional_args {
            codepoint_str_lookup(uc_block, codepoint_str.as_str());
        }
    }
    else if start != 0 && end != 0 {
        search_keyword(uc_block, "*");
    }
}
