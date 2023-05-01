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
        if ld.len() > 10 {
            ld.remove(0);
        }
        if td.len() > 10 {
            td.remove(0);
        }
        draw(&ld, &td, &name);
        thread::sleep(one_sec);
    }
}
fn draw(lastdata: &Vec<i32>, tempdata: &Vec<i32>, name: &String) {
    graph(lastdata, 4);
    println!("");
    println!("");
    graph(tempdata, 4);
    
}
fn graph(data: &Vec<i32>, ratio: i32) {
    let mut lauf = 1;
    let data = data.to_owned();
    // let mut l = 0;
    // for item in &data {
    //     if item.to_owned() > l {
    //         l = item.to_owned();
    //     }
    // }
    let mut höhe = 0;
    termsize::get().map(|size| {
        höhe = size.rows as i32;
    });
    // if max > höhe / 3 {
    //     max = höhe / 3;
    // } else if max > 10 {
    //     max = max;
    // } else {
    //     max = 10;
    // }
    let max = höhe / 3;
    for zeile in (1..=max).rev() {
        lauf += 1;
        {
            let zeile = zeile * ratio;
            print!("{zeile:>2}");
        }
        for stelle in 0..data.len() {
            if data[stelle] >= zeile * ratio {
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
        1 => print!("{}", "  ".on_red()),
        10 => print!("{}", "  ".on_blue()),
        11 => print!("{}", "  ".on_black()),
        _ => print!("{}", "  ".on_green()),
        // 0 => print!("{}", "  ".on_truecolor(244, 80, 0)),
        // 1 => print!("{}", "  ".on_truecolor(221, 89, 1)),
        // 2 => print!("{}", "  ".on_truecolor(198, 98, 2)),
        // 3 => print!("{}", "  ".on_truecolor(176, 108, 3)),
        // 4 => print!("{}", "  ".on_truecolor(130, 126, 6)),
        // 5 => print!("{}", "  ".on_truecolor(107, 135, 7)),
        // 6 => print!("{}", "  ".on_truecolor(85, 145, 8)),
        // 7 => print!("{}", "  ".on_truecolor(62, 154, 9)),
        // _ => print!("{}", "  ".on_truecolor(39, 163, 10)),
    };
}
