mod utils;
use std::{env, fs, process};

use comrak::{Options, markdown_to_html};

use crate::utils::{convert_spaces, read_file, wrap_in_span};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() || args.len() < 2 {
        println!("Usage: confert <type> <file> [OPTIONS]\n");
        println!("Types:");
        println!("markdown");
        println!("code\n");
        println!("Options:");
        println!("  --output FILE");
        println!("  --size N | Size of spaces (code only)");
        process::exit(1);
    }

    let r#type = args[0].as_str();

    match r#type {
        "code" => {
            convert_code(args);
        }
        "markdown" => {
            convert_markdown(args);
        }
        c => {
            println!("confert: {c} not a supported type");
            process::exit(1);
        }
    }

    process::exit(0)
}

fn convert_code(args: Vec<String>) {
    let mut space_size: Option<usize> = None;
    let mut output_file: Option<String> = None;
    let file_location = args[1].as_str();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--size" => {
                if let Some(space_size_str) = args.get(i + 1) {
                    space_size = match space_size_str.parse::<usize>() {
                        Ok(n) => Some(n),
                        Err(e) => {
                            println!("confert: Cant parse usize from str(space_size_str): {e}");
                            process::exit(1);
                        }
                    };
                    i += 2;
                } else {
                    println!("confert: No size provided");
                    process::exit(1);
                }
            }
            "--output" => {
                if let Some(file) = args.get(i + 1) {
                    output_file = Some(file.clone());
                    i += 2;
                } else {
                    println!("confert: No output file provided");
                    process::exit(1);
                }
            }
            _ => i += 1,
        }
    }
    let data = read_file(file_location);
    let spaces_converted = convert_spaces(data, space_size.unwrap_or(2));
    let wrapped = wrap_in_span(spaces_converted);
    let code_block = format!("<code>{}</code>", wrapped);

    if let Some(file) = output_file {
        match fs::write(file, code_block) {
            Ok(_) => {}
            Err(e) => {
                println!("confert: Couldnt write to file: {e}");
                process::exit(1);
            }
        }
    } else {
        println!("{code_block}");
    }
}

fn convert_markdown(args: Vec<String>) {
    let mut output_file: Option<String> = None;
    let file_location = args[1].as_str();

    let mut i = 1;
    while i < args.len() {
        if args[i].as_str() == "--output" {
            if let Some(file) = args.get(i + 1) {
                output_file = Some(file.clone());
                i += 2;
            } else {
                println!("confert: No output file provided");
                process::exit(1);
            }
        } else {
            i += 1
        }
    }
    let data = read_file(file_location);
    let html = markdown_to_html(&data, &Options::default());

    if let Some(file) = output_file {
        match fs::write(file, html) {
            Ok(_) => {}
            Err(e) => {
                println!("confert: Couldnt write to file: {e}");
                process::exit(1);
            }
        }
    } else {
        println!("<article>\n{html}</article>");
    }
}
