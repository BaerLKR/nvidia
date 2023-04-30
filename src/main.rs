use std::process::Command;
use colored::*;

fn main() {
    let name = get_name();
    println!("{name}");
    // let temp = get_temp();
    // println!("{temp}");
    // let auslast = get_last();
    // println!("{auslast}");
}
fn get_temp() -> String {
    let mut get = Command::new("sh");
    get.arg("-c");
    get.arg("nvidia-smi --query-gpu=temperature.gpu --format=csv,noheader,nounits");
    get.status().expect("process failed to execute").to_string()
}
fn get_name() -> String {
    let mut get = Command::new("sh");
    get.arg("-c");
    get.arg("nvidia-smi --query-gpu=gpu_name --format=csv,noheader");
    get.status().expect("process failed to execute").to_string()
}
fn get_last() -> String {
    let mut get = Command::new("sh");
    get.arg("-c");
    get.arg("nvidia-smi --query-gpu=utilization.gpu --format=csv,noheader");
    get.status().expect("process failed to execute").to_string()
}
