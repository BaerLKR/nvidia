use std::{process::Command};
pub fn get_temp() -> String {
    let mut get = Command::new("sh");
    get.arg("-c");
    get.arg("nvidia-smi --query-gpu=temperature.gpu --format=csv,noheader,nounits");
    format(get)
}
pub fn get_name() -> String {
    let mut get = Command::new("sh");
    get.arg("-c");
    get.arg("nvidia-smi --query-gpu=gpu_name --format=csv,noheader");
    format(get)
}
pub fn get_last() -> String {
    let mut get = Command::new("sh");
    get.arg("-c");
    get.arg("nvidia-smi --query-gpu=utilization.gpu --format=csv,noheader");
    format(get)
}
pub fn get_driver_version() -> String {
    let mut get = Command::new("sh");
    get.arg("-c");
    get.arg("nvidia-smi --query-gpu=driver_version --format=csv,noheader");
    format(get)
}
pub fn get_memory_total() -> String {
    let mut get = Command::new("sh");
    get.arg("-c").arg("nvidia-smi --query-gpu=memory.total --format=csv,noheader");
    format(get)
}
pub fn get_memory_used() -> String {
    let mut get = Command::new("sh");
    get.arg("-c").arg("nvidia-smi --query-gpu=memory.used --format=csv,noheader");
    format(get)
}
fn format(mut i: Command) -> String {
    let mut res = String::new();
    match i.output() {
        Ok(o) => {
            res = String::from_utf8(o.stdout).expect("error translating");  
        },
        Err(e) => {
            println!("{e}");
        },
    }
    res.trim().to_string()
}
