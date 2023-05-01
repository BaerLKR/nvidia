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
        draw(&ld, &td, &name);
        thread::sleep(one_sec);
    }
}
fn draw(lastdata: &Vec<i32>, tempdata: &Vec<i32>, name: &String) {
    // println!("Auslastung: {:?}", lastdata);
    // println!("Temperatur: {:?}", tempdata);
    // println!("Name: {name}");
    graph(lastdata);
}
fn graph(data: &Vec<i32>) {
    let mut lauf = 1;
    let data = data.to_owned();
    let mut max = 0;
    for item in &data {
        if item.to_owned() > max {
            max = item.to_owned();
        }
    }
    for zeile in (1..=max).rev() {
        lauf += 1;
        print!("{zeile:>2}");
        for stelle in 0..data.len() {
            if data[stelle] >= zeile {
                farbe(&lauf);
            } else {
                print!("  ");
            }
        }
        print!("\n");
    }
}
fn help() {
    println!("help");
}
fn farbe(lauf: &i32) {
    match lauf {
        1 => print!("{}", "  ".on_truecolor(39, 163, 10)),
        2 => print!("{}", "  ".on_truecolor(62, 154, 9)),
        3 => print!("{}", "  ".on_truecolor(85, 145, 8)),
        4 => print!("{}", "  ".on_truecolor(107, 135, 7)),
        5 => print!("{}", "  ".on_truecolor(130, 126, 6)),
        6 => print!("{}", "  ".on_truecolor(153, 117, 4)),
        7 => print!("{}", "  ".on_truecolor(176, 108, 3)),
        8 => print!("{}", "  ".on_truecolor(198, 98, 2)),
        9 => print!("{}", "  ".on_truecolor(221, 89, 1)),
        _ => print!("{}", "  ".on_truecolor(244, 80, 0)),
    };
}
