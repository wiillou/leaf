mod colors;
mod utils;

use colors::*;
use std::env::var;
use systemstat::{Duration, Platform, System};

use std::process::Command;

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


fn getPkgs() -> String {
  let distroName = match whoami::distro() {
      Ok(distro) => distro.split(' ').next().unwrap_or("Unknown"),
      Err(_) => "Unknown".to_string(),
  };

  if distroName == "NixOS" {
      let output = Command::new("sh")
          .arg("-c")
          .arg("nix-store -qR /run/current-system/sw ~/.nix-profile | wc -l")
          .output()
          .expect("Failed to execute command");
      let output_str = String::from_utf8(output.stdout).unwrap_or_else(|_| "".to_string());
      return output_str;
  }
  else {
      return "0".to_string();
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
      Ok(count) => count,
      Err(error) => {
          eprintln!("An error occurred: {}", error);
          "0".to_string()
      },
    };

    // format fetch text
    let fetch_text = vec![
        format!("{RED} {WHITE} ~ {WHITE}{}{CYAN}@{WHITE}{}{BLUE}", whoami::username(), whoami::hostname()),
        format!("{YELLOW}󰻀 {WHITE} ~ {YELLOW}{}{BLUE}", whoami::distro()),
        format!("{GREEN} {WHITE} ~ {GREEN}{}{BLUE}", format_uptime(uptime)),
        format!("{CYAN} {WHITE} ~ {CYAN}{wm}{BLUE}"),
        format!("{BLUE} {WHITE} ~ {BLUE}{term}{BLUE}"),
        format!("{PURPLE} {WHITE} ~ {PURPLE}{shell}{BLUE}"),
        format!("{RED}󰏖 {WHITE} ~ {RED}{pkgs}{BLUE}")
        format!("{WHITE}● {RED}● {YELLOW}● {GREEN}● {CYAN}● {BLUE}● {PURPLE}● {BLACK}● {RESET}"),
    ]
    .join("\n");

    println!("{fetch_text}");
}
