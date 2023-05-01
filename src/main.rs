use colored::*;
use std::{thread, time, io::Write, slice::Iter};

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
    let mut ld: Vec<i32> = Vec::new();
    let mut td: Vec<i32> = Vec::new();
    loop {
        let name = cmd::get_name();
        let temp = cmd::get_temp();
        let auslast = cmd::get_last();
        clearscreen::clear().expect("Error clearing screen.");
        let one_sec = time::Duration::from_secs(1);
        {
            let temp = temp.parse::<i32>().unwrap();
            td.push(temp);
        };
        {
            let mut auslast = auslast.chars();
            auslast.next_back();
            let laststring = String::from(auslast.as_str());
            let lastzahl = laststring.trim().parse::<i32>().unwrap();
            ld.push(lastzahl);
        };
        if ld.len() > 3 {
            ld.remove(0);
        }
        if td.len() > 3 {
            td.remove(0);
        }
        graph(&ld, &td, &name);
        thread::sleep(one_sec);
    }
}
fn graph(lastdata: &Vec<i32>, tempdata: &Vec<i32>, name: &String) {
    println!("Auslastung: {:?}", lastdata);
    println!("Temperatur: {:?}", tempdata);
    println!("Name: {name}");
}
fn help() {
    println!("help");
}
