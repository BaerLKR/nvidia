use std::process::Command;
use colored::*;

fn main() {
    println!("{}", get_name().yellow());
    println!("{}", get_temp());
    println!("{}", get_last());
}
fn get_temp() -> String {
    let mut get = Command::new("sh");
    get.arg("-c");
    get.arg("nvidia-smi --query-gpu=temperature.gpu --format=csv,noheader,nounits");
    let mut res = String::new();
    match get.output() {
        Ok(o) => {
            res = String::from_utf8(o.stdout).expect("error translating");
        },
        Err(e) => {
            println!("{e}");
        }
    }
    res.trim().to_string()
}
fn get_name() -> String {
    let mut get = Command::new("sh");
    get.arg("-c");
    get.arg("nvidia-smi --query-gpu=gpu_name --format=csv,noheader");
    let mut res = String::new();
    match get.output() {
        Ok(o) => {
            res = String::from_utf8(o.stdout).expect("error translating");
        },
        Err(e) => {
            println!("{e}");
        }
    }
    res.trim().to_string()
}
fn get_last() -> String {
    let mut get = Command::new("sh");
    get.arg("-c");
    get.arg("nvidia-smi --query-gpu=utilization.gpu --format=csv,noheader");
    let mut res = String::new();
    match get.output() {
        Ok(o) => {
            res = String::from_utf8(o.stdout).expect("error translating");
        },
        Err(e) => {
            println!("{e}");
        }
    }
    res.trim().to_string()
}
