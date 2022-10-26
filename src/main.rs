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
    if codepoint_str == "0" {
        return Some(0);
    }

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

fn parse_numeric_block(block_str: &str) -> Option<(u32, u32)> {
    let sides = block_str.split("-").collect::<Vec<&str>>();
    if sides.len() == 2 {
        let start = parse_int(sides[0]);
        let end = parse_int(sides[1]);

        if start.is_some() && end.is_some() {
            return Some((start.unwrap(), end.unwrap()));
        }
    }
    return None;
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

fn print_highlighted(str: &str, hi_start: u32, hi_end: u32) {
    for c in str.chars() {
        let c_u32 = c as u32;
        if c_u32 < hi_start || c_u32 > hi_end  {
            // 1:  bold
            // 37: white foreground
            // 97: axiterm bright white forground
            // 45: magenta background
            print!("\x1b[1;37;97;45m");
            match c_u32 {
                0x0  => print!("<null>"),
                0x1  => print!("<start-of-heading>"),
                0x2  => print!("<start-of-text>"),
                0x3  => print!("<end-of-text>"),
                0x4  => print!("<end-of-transmission>"),
                0x5  => print!("<enquiry>"),
                0x6  => print!("<acknowledge>"),
                0x7  => print!("<bell>"),
                0x8  => print!("<backspace>"),
                0x9  => print!("<character-tabulation>"),
                0xb  => print!("<line-tabulation>"),
                0xc  => print!("<form-feed-(ff)>"),
                0xe  => print!("<shift-out>"),
                0xf  => print!("<shift-in>"),
                0x10 => print!("<data-link-escape>"),
                0x11 => print!("<device-control-one>"),
                0x12 => print!("<device-control-two>"),
                0x13 => print!("<device-control-three>"),
                0x14 => print!("<device-control-four>"),
                0x15 => print!("<negative-acknowledge>"),
                0x16 => print!("<synchronous-idle>"),
                0x17 => print!("<end-of-transmission-block>"),
                0x18 => print!("<cancel>"),
                0x19 => print!("<end-of-medium>"),
                0x1a => print!("<substitute>"),
                0x1b => print!("<escape>"),
                0x1c => print!("<information-separator-four>"),
                0x1d => print!("<information-separator-three>"),
                0x1e => print!("<information-separator-two>"),
                0x1f => print!("<information-separator-one>"),
                0x200b => print!("<zero-width-space>"),
                0x200c => print!("<zero-width-non-joiner>"),
                0x200d => print!("<zero-width-joiner>"),
                0xfeff => print!("<zero-width-no-break-space>"),
                _ => print!("{}", c)
            };

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
    let mut describe = true;
    let mut identify = false;
    let mut search = false;
    let mut list = false;
    let mut highlight = false;

    let mut ascii = false;
    let mut ascii_ext = false;
    let mut emoji = false;
    let mut start : u32 = 0;
    let mut end : u32 = 0;
    let mut block_arg : Option<String> = None;

    let mut file : Option<String> = None;

    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut describe)
            .add_option(&["-d", "--describe"], StoreTrue,
            "Describe codepoints (default)");
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
            "Highlight characters not in the current block");
        ap.refer(&mut file)
            .add_option(&["-f", "--file"], StoreOption,
            "Specify file");
        ap.refer(&mut list)
            .add_option(&["-l", "--list-blocks"], StoreTrue,
            "List known blocks");
        ap.refer(&mut block_arg)
            .add_option(&["-b", "--block"], StoreOption,
            "Consider only a named block or range as start-end");
        ap.refer(&mut ascii)
            .add_option(&["--ascii"], StoreTrue,
            "Consider only the ASCII block");
        ap.refer(&mut ascii_ext)
            .add_option(&["--ascii-ext"], StoreTrue,
            "Consider only the ASCII extended block");
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
                    match parse_numeric_block(block_name.as_str()) {
                        Some((block_start, block_end)) => {
                            start = block_start;
                            end = block_end;
                        }
                        None => {
                            eprintln!("Unknown block: {}", block_name);
                            exit(1);
                        }
                    }
                }
            };
        },
        None => {}
    };

    if ascii {
        start = 0u32;
        end = 127u32;
    }
    else if ascii_ext {
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
            let (hi_start, hi_end) = if end > 0 {
                (start, end)
            } else {
                (0, 127)
            };
            print_highlighted(str, hi_start, hi_end);
        }
    }
    else if describe && positional_args.len() > 0 {
        for codepoint_str in &positional_args {
            codepoint_str_lookup(uc_block, codepoint_str.as_str());
        }
    }
    else if end != 0 {
        search_keyword(uc_block, "*");
    }
}
