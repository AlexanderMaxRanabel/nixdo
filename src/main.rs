use std::io;
use std::io::Write;
use std::io::Read;
use std::fs;
use std::path::{Path, PathBuf};
use std::env;
use std::process::Command;
use std::fs::File;
use std::io::{BufRead, BufReader};
use dirs::home_dir;


fn main() -> std::io::Result<()> {
    if let Some(home) = home_dir() {
        let home_path = home;
        let ptf = format!(
            "{}{}",
            home_path.to_string_lossy(),
            "/nixdo/c695eb00a50618ca50d17631f303f8c54c4011f74106e58907d3666fbd2e9733"
        );
        let dir_path = PathBuf::from(ptf);
        if !dir_path.exists() {
        let mut password = String::new();
            println!("Oh no, Nixdo doesnt exist. please enter your system password");
            io::stdin().read_line(&mut password).expect("Err");
            let password = password.trim();
            let mut file = fs::OpenOptions::new().append(true).create(true).open(&dir_path);
             writeln!(file?, "{}", password)?;
            fs::create_dir(&dir_path);
        }

        let mut readit = fs::File::open(dir_path).expect("Failure");
        let mut psword = String::new();

        readit.read_to_string(&mut psword).expect("Failure");

        let args: Vec<String> = env::args().collect();

        let command = &args[1].to_string();

        let command = format!("echo {} | su -c {}", psword, command);
        let output = Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .expect("Error");

          if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            println!("Command executed successfully:\n{}", stdout);
          } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            println!("Command failed:\n{}", stderr);
        }
    } else {
        println!("Unable to determine the home directory");
    }
    
    Ok(())
}
