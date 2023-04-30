use colored::*;

pub mod cmd;

fn main() {
    let a = nvidia::args();
    match a {
        Some(v) => {
            let mut c = 0;
            for arument in &v {
                match arument.as_str() {
                    "help" | "h" | "-h" | "--help" => {
                        help()
                    },
                    _ => {
                        c += 1;
                    },
                };
                if c == v.len() {
                    normal();
                }
            }
        },
        None => {
            normal();
        },
    };
}
fn normal() {
    println!("{}", cmd::get_name().yellow());
    println!("{}", cmd::get_temp());
    println!("{}", cmd::get_last());
}
fn help() {
    println!("help");
}
