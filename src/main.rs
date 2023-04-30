use colored::*;
use std::{thread, time};

pub mod cmd;
pub mod graph;

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
    loop {
        clearscreen::clear().expect("Error cloearing the screen!");
        println!("{}", cmd::get_name().yellow());
        println!("{}", cmd::get_temp());
        println!("{}", cmd::get_last());
        let one_sec = time::Duration::from_secs(1);
        thread::sleep(one_sec);
    }
}
// fn graph(data: Vec<i32>) {

// }
fn help() {
    println!("help");
}
