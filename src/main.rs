use walkdir::WalkDir;
use rayon::prelude::*;
use colored::*;
use std::{fs, io, path::{Path, PathBuf}, time::Instant};

const HEADER_PART_1: &str = r#"
    _   __           __     __  ______
   / | / /_  ___  __/ /_   / / / /  _/
  /  |/ / / / / |/_/ __/  / / / // /  
 / /|  / /_/ />  </ /_   / /_/ // /   
/_/ |_/\__,_/_/|_|\__/   \____/___/              
"#;

const HEADER_PART_2: &str = r#"
    ____                ____                             
   / __ \_________     / __ )__  ______  ____ ___________
  / /_/ / ___/ __ \   / __  / / / / __ \/ __ `/ ___/ ___/
 / ____/ /  / /_/ /  / /_/ / /_/ / /_/ / /_/ (__  |__  ) 
/_/   /_/   \____/  /_____/\__, / .___/\__,_/____/____/  
                          /____/_/                      
"#;

const VERIFY_URL: &str = "https://api.nuxtlabs.com/ui-pro/verify";
const BYPASS_URL: &str = "https://httpbin.org/status/200";
const TARGET: &str = "node_modules/@nuxt";

fn main() -> io::Result<()> {
    control::set_override(true);
    
    let start = Instant::now();

    print!("{}", HEADER_PART_1.bright_blue());
    println!("{}", HEADER_PART_2.bright_green());
    println!("{} {}", "[i]".bright_yellow(), "This script is intended only for showing the problem".bright_black());
    println!("{} {}", "[!]".bright_red(), "Never use it in production it is illegal xd!!!)".bright_black());
    println!("{} {}", "[@]".bright_cyan(), "By qweme32 ( https://github.com/qweme32 )".bright_black());

    let root = Path::new(TARGET);

    if !root.exists() || !root.is_dir() {
        println!("{} {}", "[X]".bright_red(), format!("Dir {} does not exist or is not accessible.", TARGET));
        
        return Ok(());
    }

    println!("{} {} {}", "[?]", "Scanning dir:".bright_black(), TARGET.bright_black());

    // Collect paths
    let paths: Vec<PathBuf> = WalkDir::new(&root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| {
            entry.file_type().is_file()
            && matches!(
                entry.path().extension().and_then(|e| e.to_str()),
                Some("js") | Some("mjs") | Some("ts") | Some("mts")
            )
        })
        .filter(|entry| {
            fs::read_to_string(entry.path())
                .map(|content| content.contains(VERIFY_URL))
                .unwrap_or(false)
        })
        .map(|entry| entry.into_path())
        .collect();

    // Zero paths check
    if paths.len() == 0 {
        println!("{} {}", "[X]".bright_red(), "It looks like you have already bypassed the check or the script is outdated");
        return Ok(());
    }

    // Rewrite for bypass
    paths.par_iter().for_each(|path| {
        match fs::read_to_string(path) {
            Ok(content) => {
                let new_content = content.replace(VERIFY_URL, BYPASS_URL);
                match fs::write(path, new_content) {
                    Ok(_) => println!("{} {} {}", "[+]".green(), "Bypassed", path.as_path().to_str().unwrap().bright_black()),
                    Err(err) => eprintln!("{} {} {}", "[X]".bright_red(), "Rewrite failed", err),
                }
            }
            Err(err) => eprintln!("{} {} {}", "[X]".bright_red(), "Rewrite failed", err),
        }
    });

    // Finish
    let dur = start.elapsed().as_millis();
    println!("\n{} {}", "[T]".bright_magenta(), format!("{}ms elapsed", dur.to_string()).bright_black());

    Ok(())
}

