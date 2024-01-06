mod colors;
mod utils;

use colors::*;
use std::env::var;
use systemstat::{Duration, Platform, System};

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

    // format fetch text
    let fetch_text = vec![
        format!(
            "{WHITE}{}{RED}@{RESET}{}{BLUE}",
            whoami::username(),
            whoami::hostname()
        ),
        format!("{CYAN}󰻀 {WHITE} ~ {CYAN}{}{BLUE}", whoami::distro()),
        format!("{YELLOW}  {WHITE} ~ {YELLOW}{}{BLUE}", format_uptime(uptime)),
        format!("{GREEN} {WHITE} ~ {GREEN}{wm}{BLUE}"),
        format!("{MAGENTA} {WHITE} ~ {MAGENTA}{term}{BLUE}"),
        format!("{YELLOW_BRIGHT} {WHITE} ~ {YELLOW_BRIGHT}{shell}{BLUE}"),
        format!("{WHITE}● {RED}● {YELLOW}● {GREEN}● {CYAN}● {BLUE}● {PURPLE}● {BLACK}● {RESET}"),
    ]
    .join("\n");

    println!("{fetch_text}");
}
