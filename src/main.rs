use std::process::Command;
fn main() {
    get_name();
    get_temp();
}
fn get_temp() {
    let mut get = Command::new("sh");
    get.arg("-c");
    get.arg("nvidia-smi --query-gpu=temperature.gpu --format=csv,noheader,nounits");
    get.status().expect("process failed to execute");
}
fn get_name() {
    let mut get = Command::new("sh");
    get.arg("-c");
    get.arg("nvidia-smi --query-gpu=gpu_name --format=csv,noheader");
    get.status().expect("process failed to execute");
}
