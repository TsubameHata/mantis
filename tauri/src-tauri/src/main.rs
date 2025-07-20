#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use std::process::{Command, Stdio, Child};

fn launch_server()->Child {
    println!("{:?}", env::current_dir().expect("").join("resources").join("__main__.exe"));
    let exe_path = env::current_dir().expect("")
      .join("resources").join("__main__.exe");
    let mut cmd = Command::new(exe_path);
    cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit());
    return cmd.spawn().expect("Failed to launch server.");
}

fn main() {
  let mut child = launch_server();
  app_lib::run();
  child.kill().expect("");
}
