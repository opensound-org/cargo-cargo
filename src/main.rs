use std::process::{exit, Command};

fn main() {
    exit(
        match Command::new("cargo")
            .args(std::env::args_os().skip(2))
            .spawn()
        {
            Err(err) => {
                eprintln!("{}", err);
                -1
            }
            Ok(mut child) => match child.wait() {
                Err(err) => {
                    eprintln!("{}", err);
                    -2
                }
                Ok(code) => match code.code() {
                    None => -3,
                    Some(code) => code,
                },
            },
        },
    );
}
