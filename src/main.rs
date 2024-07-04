use std::env;
use std::process::ExitCode;

#[cfg(target_os = "linux")]
fn main() -> ExitCode {
    for argument in env::args() {
        println!("{argument}");
    }

    let key = "HOME";
    match env::var(key) {
        Ok(val) => println!("{key}: {val:?}"),
        Err(e) => println!("error {key}, {e}"),
    }

    ExitCode::SUCCESS
}
