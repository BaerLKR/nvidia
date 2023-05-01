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
        if ld.len() > 20 {
            ld.remove(0);
        }
        if td.len() > 20 {
            td.remove(0);
        }
        draw(&ld, &td, &name);
        // btmbar();
        thread::sleep(one_sec);
    }
}
fn draw(lastdata: &Vec<i32>, tempdata: &Vec<i32>, name: &String) {
    println!("{}", name.bold().yellow());
    let util = String::from("Utilization in %-");
    topbar(util);
    graph(lastdata, 5);
    btmbar();
    println!("");
    println!("");
    let inc = String::from("Temperature in °C");
    topbar(inc);
    graph(tempdata, 5);
    btmbar();
}
fn topbar(title: String) {
    print!("+-{}", title);
    for _ in 0..29 {
        print!("-");
    }
    print!("+\n");
}
fn btmbar() {
    print!("+");
    for _ in 0..48 {
        print!("-");
    }
    print!("+\n");
}
fn graph(data: &Vec<i32>, ratio: i32) {
    let mut lauf = 1;
    let data = data.to_owned();
    let mut höhe = 0;
    termsize::get().map(|size| {
        höhe = size.rows as i32;
    });
    let max = höhe / 3;
    for zeile in (0..=max).rev() {
        lauf += 1;
        {
            let zeile = zeile * ratio;
            print!("| {:>3} | ", zeile.to_string().blue());
        }
        for _ in 0..(20 - data.len()) {
            print!("  ");
        }
        for stelle in (0..data.len()).rev() {
            if data[stelle] >= zeile * ratio {
                farbe(&lauf);
            } else {
                print!("  ");
            }
        }
        print!(" |\n");
    }
}
fn help() {
    println!("help");
}
fn farbe(lauf: &i32) {
    let r = farbgen(lauf)[0].try_into().unwrap();
    let g = farbgen(lauf)[1].try_into().unwrap();
    let b = farbgen(lauf)[2].try_into().unwrap();
    print!("{}", "  ".on_truecolor(r, g, b));
}
fn farbgen(num: &i32) -> Vec<i32> {
    let mut ausgangsfarbe = vec![235, 111, 146];
    for _ in 0..num.to_owned() {
        ausgangsfarbe[0] -= 10;
        ausgangsfarbe[1] += 2;
        ausgangsfarbe[2] -= 3;
    }
    ausgangsfarbe
}
