use std::str;
use std::process::Command;

fn main() {

    let hello = Command::new("sh")
        .arg("-c")
            .arg("cat /etc/issue")
            .output()
            .expect("failed to execute process");

    println!("{:?}", hello);
    println!("{}", str::from_utf8(&hello.stdout).unwrap() );
}
