mod colors;
mod utils;

use colors::*;
use std::env::var;
use systemstat::{Duration, Platform, System};

use std::process::Command;
use os_release::OsRelease;

fn format_uptime(uptime: Duration) -> String {
   let total_seconds = uptime.as_secs();
   let days = total_seconds / 86400;
   let hours = (total_seconds % 86400) / 3600;
   let minutes = ((total_seconds % 86400) % 3600) / 60;
   let seconds = ((total_seconds % 86400) % 3600) % 60;

   let mut result = String::new();
   if days > 0 {
       result.push_str(&format!("{}d ", days));
   }
   if days > 0 || hours > 0 {
       result.push_str(&format!("{}h ", hours));
   }
   if days > 0 || hours > 0 || minutes > 0 {
       result.push_str(&format!("{}m ", minutes));
   }
   if days > 0 || hours > 0 || minutes > 0 || seconds > 0 {
       result.push_str(&format!("{}s", seconds));
   }

   result.trim().to_string()
}



fn getPkgs() -> Result<String, Box<dyn std::error::Error>> {
  let release = OsRelease::new()?;
  match release.id.as_str() {
      "debian" | "ubuntu" => {
          let output = Command::new("sh")
              .arg("-c")
              .arg("dpkg --get-selections | grep -v deinstall | wc -l")
              .output()?;
          let output_str = String::from_utf8_lossy(&output.stdout).into_owned();
          Ok(output_str)
      },
      "centos" | "fedora" => {
          let output = Command::new("sh")
              .arg("-c")
              .arg("rpm -qa | wc -l")
              .output()?;
          let output_str = String::from_utf8_lossy(&output.stdout).into_owned();
          Ok(output_str)
      },
      "arch" | "manjaro" | "endevouros" => {
          let output = Command::new("sh")
              .arg("-c")
              .arg("pacman -Q | wc -l")
              .output()?;
          let output_str = String::from_utf8_lossy(&output.stdout).into_owned();
          Ok(output_str)
      },
      "nixos" | "snowflakeos" => {
          let output = Command::new("sh")
              .arg("-c")
              .arg("nix-store -qR /run/current-system/sw ~/.nix-profile | wc -l")
              .output()?;
          let output_str = String::from_utf8_lossy(&output.stdout).into_owned();
          Ok(output_str)
      },
      _ => Ok("0".to_string()),
  }
}

fn main() {
    let sys = System::new();

    // get wm
    let wm = utils::get_wm().unwrap_or_else(|| String::from("unknown"));

    // get terminal
    let term = var("TERM").unwrap_or_else(|_| String::from("unknown"));

    // get shell
    let shell = utils::get_shell().unwrap_or_else(|| String::from("unknown"));

    // get uptime
    let uptime = sys.uptime().unwrap_or_else(|_| Duration::default());   
      
    let pkgs = match getPkgs() {
       Ok(pkgs) => pkgs,
       Err(_) => String::from("An error occurred"),
    };

    //get kernel
    let kernel_version = String::from_utf8_lossy(&Command::new("uname").arg("-r").output().expect("Failed to execute command").stdout).into_owned();

    // format fetch text
    let fetch_text = vec![
        format!("{RED} {WHITE} ~ {WHITE}{}{CYAN}@{WHITE}{}{BLUE}", whoami::username(), whoami::hostname()),
        format!("{YELLOW}󰻀 {WHITE} ~ {YELLOW}{}{BLUE}", whoami::distro()),
        format!("{GREEN} {WHITE} ~ {GREEN}{}{BLUE}", format_uptime(uptime)),
        format!("{CYAN} {WHITE} ~ {CYAN}{wm}{BLUE}"),
        format!("{BLUE} {WHITE} ~ {BLUE}{term}{BLUE}"),
        format!("{MAGENTA} {WHITE} ~ {MAGENTA}{shell}{BLUE}"),
        format!("{RED}󰏖 {WHITE} ~ {RED}{pkgs}{BLUE}"),
        format!("{WHITE}● {RED}● {YELLOW}● {GREEN}● {CYAN}● {BLUE}● {MAGENTA}● {BLACK}● {RESET}"),
    ]
    .join("\n");

    println!("{fetch_text}");
}
