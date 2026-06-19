use std::error::Error;
use std::fs::File;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://leetcode.doocs.org/en/lc/3831/";

    let response = reqwest::get(url).await?;

    if response.status().is_success() {
        let html_source = response.text().await?;
        let md = html2md::rewrite_html(&html_source, false);
        // println!("{md}");
let mut file = File::create("md.md").expect("creat failed"); // Create or overwrite the file
file.write_all(md.as_bytes()).expect("write failed");
        
    }

    Ok(())
}
