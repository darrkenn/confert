use std::{fs, path::PathBuf, process, str::FromStr};

use regex::Regex;

pub fn read_file(file_location: &str) -> String {
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

pub fn convert_spaces(data: String, space_size: usize) -> String {
    let re = Regex::new(r"(?m)^ +").unwrap();

    let replaced = re.replace_all(&data, |caps: &regex::Captures| {
        let space_count = caps[0].len();
        let num_spaces = space_count / space_size;
        "&nbsp;".repeat(num_spaces)
    });
    replaced.to_string()
}

pub fn wrap_in_span(data: String) -> String {
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
