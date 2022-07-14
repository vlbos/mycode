use error_chain::error_chain;
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::{BufRead, Write};
use std::path::Path;

use glob::glob;

error_chain! {
    foreign_links {
        Glob(glob::GlobError);
        Pattern(glob::PatternError);
    }
}

fn main() -> Result<()> {
    println!("Welcome to leetcode-rust system.\n");
    let lines: Vec<Vec<i32>> = io::BufReader::new(File::open("./ids.txt").unwrap())
        .lines()
        .map(|x| {
            let xa = x
                .unwrap()
                .split(",")
                .filter(|c| !c.is_empty())
                .map(|b| b.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            if xa.len() == 2 {
                (xa[0]..=xa[1]).collect::<Vec<i32>>()
            } else {
                xa
            }
        })
        .collect();
    let ids_set: std::collections::HashSet<i32> = lines.into_iter().flatten().collect();
    println!("{:?}", ids_set.len());
    for entry in glob("/Users/lisheng/Downloads/myleetcode/leetcode-repo-master/src-all/Rust/*.rs")?
    {
        let mut filenamepath = format!("{}", entry?.display());
        let j = filenamepath.rfind("/").unwrap_or(0);
        let mut filename = filenamepath[j + 1..].to_string();
        let id = filename[1..5].to_string();
        if !ids_set.contains(&id.parse::<i32>().unwrap()) {
            continue;
        }
        filename = filename.replace("-", "_");

        let new_filepath = format!("./src/solutions/no_{}{}", id, &filename[5..],);
        let new_filename = format!("no_{}{}", id, &filename[5..filename.len() - 3]);
        println!("{},{},{}", filenamepath, new_filename, new_filepath);
        deal_solving(&filenamepath, &new_filename, &new_filepath);
    }

    Ok(())
}

fn deal_solving(filename: &String, new_filename: &String, new_filepath: &String) {
    let file_path = Path::new(filename);
    if !file_path.exists() {
        println!("file_path no exist:{:?}", file_path);
        return;
    }

    let solution_path = Path::new(new_filepath);
    if solution_path.exists() {
        println!("solution_path no exist :{:?}", solution_path);
        return;
    }
    // rename/move file
    fs::rename(file_path, solution_path).unwrap();
    let mut lib_file = fs::OpenOptions::new()
        .append(true)
        .open("./src/solutions/mod.rs")
        .unwrap();
    writeln!(lib_file, "mod {};", new_filename);
}
fn main1() -> Result<()> {
    println!("Welcome to leetcode-rust system.\n");
    for entry in glob("/Users/lisheng/Downloads/myleetcode/rusty-leetcode-master/src/**/*.rs")? {
        let mut filenamepath = format!("{}", entry?.display());
        let j = filenamepath.rfind("/").unwrap_or(0);
        let mut filename = filenamepath[j + 1..].to_string();
        if let Some(i) = filename.rfind("_") {
            let id = filename[i..filename.len() - 2].to_string();
            let new_filepath = format!(
                "./src/solutions/no{}_{}.rs",
                &filename[i..filename.len() - 3],
                &filename[..i]
            );
            let new_filename = format!("no{}_{}", &filename[i..filename.len() - 3], &filename[..i]);
            println!("{},{},{}", filenamepath, new_filename, new_filepath);
            deal_solving(&filenamepath, &new_filename, &new_filepath);
        }
    }

    Ok(())
}
