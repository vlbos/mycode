use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
   
    let url = "https://leetcode.doocs.org/en/lc/3949/";

   
    let response = reqwest::get(url).await?;

   
    if response.status().is_success() {
        let html_source = response.text().await?;
let md = html2md::rewrite_html(&html_source, false);
println!("{md}");
       
    }

    Ok(())
}

