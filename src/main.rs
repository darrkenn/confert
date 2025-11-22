use std::{env, fs, path::PathBuf, process, str::FromStr};

use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        println!("Usage: confert <file> [--size N] [--output FILE]");
        process::exit(1);
    }

    let file_location = args[0].as_str();
    let mut space_size: Option<usize> = None;
    let mut output_file: Option<String> = None;

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
            _ => {}
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
                println!("comfert: Couldnt write to file: {e}");
                process::exit(1);
            }
        }
    } else {
        println!("{code_block}");
    }
    process::exit(0)
}

fn read_file(file_location: &str) -> String {
    let path = match PathBuf::from_str(file_location) {
        Ok(pb) => pb,
        Err(e) => {
            println!("confert: Cant get path: {e}");
            process::exit(1);
        }
    };
    match fs::read_to_string(path) {
        Ok(data) => data.trim().to_string(),
        Err(e) => {
            println!("confert: Error reading file: {e}");
            process::exit(1);
        }
    }
}

fn convert_spaces(data: String, space_size: usize) -> String {
    let re = Regex::new(r"(?m)^ +").unwrap();

    let replaced = re.replace_all(&data, |caps: &regex::Captures| {
        let space_count = caps[0].len();
        let num_spaces = space_count / space_size;
        "&nbsp;".repeat(num_spaces)
    });
    replaced.to_string()
}

fn wrap_in_span(data: String) -> String {
    let re = Regex::new(r"(?m)^.+").unwrap();

    let wrapped = re.replace_all(&data, |caps: &regex::Captures| {
        format!("<span>{}</span>", &caps[0])
    });
    wrapped.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_spaces_successful() {
        let text = "    ensure_installed = {},\n    elixirls = false,";
        let output_space_size_two = convert_spaces(text.to_string(), 2);
        let output_space_size_four = convert_spaces(text.to_string(), 4);

        assert_eq!(
            output_space_size_two,
            "&nbsp;&nbsp;ensure_installed = {},\n&nbsp;&nbsp;elixirls = false,"
        );
        assert_eq!(
            output_space_size_four,
            "&nbsp;ensure_installed = {},\n&nbsp;elixirls = false,"
        )
    }

    #[test]
    fn wrap_in_span_successful() {
        let text = "mason-org/mason-lspconfig.nvim\nneovim/nvim-lspconfig";
        let output = wrap_in_span(text.to_string());

        assert_eq!(
            output,
            "<span>mason-org/mason-lspconfig.nvim</span>\n<span>neovim/nvim-lspconfig</span>"
        );
    }
}
