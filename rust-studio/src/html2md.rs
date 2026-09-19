use std::error::Error;
use std::fs::File;
use std::io::Write;
//  use reqwest::{Client,Proxy};
// #[tokio::main]
async fn main_url() -> Result<(), Box<dyn Error>> {
    let url = "https://leetcode.doocs.org/en/lc/4042/";
// let url ="https://leetcode.com/problems/create-grid-with-exactly-k-paths-ii/";
 

    let response = reqwest::get(url).await?;

    if response.status().is_success() {
        let html_source = response.text().await?;
        let md = html2md::rewrite_html(&html_source, false);
        // println!("{md}");
        let mut file = File::create("md.md").expect("creat failed"); // Create or overwrite the file
        file.write_all(md.as_bytes()).expect("write failed");
    }else{
    println!("{response:?}");
    }

    Ok(())
}


// #[tokio::main]
async fn main_file() -> Result<(), Box<dyn Error>> {

        let html_source = std::fs::read_to_string("/Users/lisheng/Downloads/Merge Close Characters II - LeetCode.html").unwrap();
        let md = html2md::rewrite_html(&html_source, false);
        // println!("{md}");
        let mut file = File::create("md.md").expect("creat failed"); // Create or overwrite the file
        file.write_all(md.as_bytes()).expect("write failed");
  

    Ok(())
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

let _= main_url().await;
// let _= main_file().await;


Ok(())
}

// 