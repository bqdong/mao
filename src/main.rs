use std::env;

use rustix::fs;

#[cfg(target_os = "linux")]
fn main() {
    let mut cmd_args = env::args();
    let path = cmd_args.nth(1).unwrap();
    let fd = fs::open(path, fs::OFlags::RDONLY, fs::Mode::empty()).unwrap();
    let stat = fs::fstat(fd).unwrap();
    println!("{stat:?}");
    println!();
    println!("Device: {0}", stat.st_dev);
    println!("Link Num: {0}", stat.st_nlink);
    let file_type = fs::FileType::from_raw_mode(stat.st_mode);
    println!("File Type: {file_type:?}");
}
