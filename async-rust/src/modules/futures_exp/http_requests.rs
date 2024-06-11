use reqwest::Error;
use tokio::time::Instant;

async fn fetch_url(url: &str) -> Result<String, Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

pub async fn fetch_concurrently() -> Result<(), Error> {
    let urls = vec!["https://www.rust-lang.org", "https://docs.rs"];
    let start = Instant::now();

    let fetches = urls.iter().map(|&url| fetch_url(url));
    let results = futures::future::join_all(fetches).await;

    for result in results{
        match result{
            Ok(body) => println!("Fetched : {}", &body[..10]),
            Err(err) => println!("Error occured: {}", err)
        }
    }
    println!("Total time elapsed is : {:?}", start.elapsed());
    Ok(())
}
