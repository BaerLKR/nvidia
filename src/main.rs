use colored::*;
use std::{thread, time};

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
    let mut ld: Vec<i32> = Vec::new();
    let mut td: Vec<i32> = Vec::new();
    let name = cmd::get_name();
    let driver = cmd::get_driver_version();
    let memtot = cmd::get_memory_total();
    loop {
        let memused = cmd::get_memory_used();
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

        let mut breite = 0;
        termsize::get().map(|size| {
            breite= size.cols as i32;
        });
        if ld.len() *  2> (breite * 3 / 4 - 3) as usize {
            ld.remove(0);
        }
        if td.len() * 2 > (breite *3 / 4 - 3 ) as usize {
            td.remove(0);
        }
        draw(&ld, &td, &name, &driver, &memtot, &memused, breite * 3 / 4);
        thread::sleep(one_sec);
    }
}
fn draw(lastdata: &Vec<i32>, tempdata: &Vec<i32>, name: &String, driver: &String, memtot: &String, memused: &String, len: i32) {
    print!("{}", name.bold().yellow());
    print!(" {}{}{}\n", "(".yellow(), driver.yellow(), ")".yellow());
    println!("");
    let util = String::from("Utilization in %-");
    topbar(util, len);
    graph(lastdata, 5, len);
    btmbar(len);
    println!("");
    let inc = String::from("Temperature in °C");
    topbar(inc, len);
    graph(tempdata, 5, len);
    btmbar(len);
    println!("");
    println!("Memory usage:");
    mem(memused, memtot)
}
fn mem(used: &String, tot: &String) {
    let toti = tot.to_owned().trim().parse::<i32>().unwrap();
    let usedi = used.to_owned().trim().parse::<i32>().unwrap();
    let frac = toti / usedi * 3;
    let out = format!("{} MiB/{} MiB", used.to_owned(), tot.to_owned());
    farbe(&frac, out);
    print!("\n");
}
fn topbar(title: String, len: i32) {
    print!("+-{}", title);
    for _ in 0..len - 11 {
        print!("-");
    }
    print!("+\n");
}
fn btmbar(len: i32) {
    print!("+");
    for _ in 0..len + 7 {
        print!("-");
    }
    print!("+\n");
}
fn graph(data: &Vec<i32>, ratio: i32, breite: i32) {
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
        for _ in 0..(breite / 2 - data.len() as i32) {
            print!("  ");
        }
        for stelle in 0..data.len() {
            if data[stelle] >= zeile * ratio {
                farbe(&lauf, String::from("  "));
            } else {
                print!("  ");
            }
        }
        print!(" |\n");
    }
}
fn help() {
    println!("{}","This is a wrapper for the nvidia-smi comman, provided by the nvidia drivers on linux.".green());
    println!("{}","The program will display the head, the usage ans the memory used.".green());
    println!("{}{}{}{}","the coding for this project was done ".green(), "by me, for fun, in rust".green().bold(), " and under the ".green(), "EUPL.".green().bold());
}
fn farbe(lauf: &i32, c: String) {
    let r = farbgen(lauf)[0].try_into().unwrap();
    let g = farbgen(lauf)[1].try_into().unwrap();
    let b = farbgen(lauf)[2].try_into().unwrap();
    print!("{}", c.on_truecolor(r, g, b));
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
