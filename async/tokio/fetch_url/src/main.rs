use tokio;
use reqwest;

async fn fetch_url(url: String) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;
    let body = response.text().await?;
    Ok(body)
}

async fn fetch_all_urls(urls: Vec<String>) {
    let mut tasks = vec![];

    for url in urls {
        let task = tokio::spawn(async move {
            match fetch_url(url).await {
                Ok(body) => println!("Response from URL: {}", body),
                Err(err) => eprintln!("Error fetching URL: {:?}", err),
            }
        });
        tasks.push(task);
    }

    // Wait for all tasks to complete
    for task in tasks {
        task.await.expect("Task failed");
    }
}

#[tokio::main]
async fn main() {
    let urls = vec![
        "http://ifzq.gtimg.cn/appstock/app/kline/mkline?param=sz000413,m1,,32000&_var=m1_today".to_string(),
        "http://ifzq.gtimg.cn/appstock/app/kline/mkline?param=sz000413,m1,,32000&_var=m1_today".to_string(),
        "http://ifzq.gtimg.cn/appstock/app/kline/mkline?param=sz000413,m1,,32000&_var=m1_today".to_string(),
    ];

    fetch_all_urls(urls).await;
}