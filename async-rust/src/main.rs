#[allow(unused)]
mod modules;
use modules::async_exp;
use futures::executor::block_on;
use modules::futures_exp;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    async_exp::intro();

    let async_http = futures_exp::http_requests::fetch_concurrently().await;
    match async_http{
        Ok(_) => println!("This ran successfully."),
        Err(err) => println!("Exited this with error : {}", err)
    }

    // if let Ok(_) = futures_exp::http_requests::fetch_concurrently().await{
    //     println!("Worked fine");
    // }
}

