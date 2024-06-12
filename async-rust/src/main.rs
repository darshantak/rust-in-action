#[allow(unused)]
mod modules;
use futures::executor::block_on;
use modules::async_exp;
use modules::futures_exp;
use tokio::time::Instant;

#[tokio::main]
async fn main() {
    async_exp::intro();

    // call_async_http().await;
    // call_async_file_io().await;
    // sync_file_io();

}

async fn call_async_http(){
    if let Ok(_) = futures_exp::http_requests::fetch_concurrently().await {
        println!("This ran successfully ");
    }
}

async fn call_async_file_io() {
    let result = futures_exp::file_io::file_example().await;
    match result{
        Ok(_) => println!("This is successfull"),
        Err(err) => println!("This has exited with this error {}",err)
    }

}

fn sync_file_io() {

    let start_time = Instant::now();
    let file_path = "example.txt";
    let contents_to_write = "Hi this is Darshan";
    // let read_contents = block_on(futures_exp::file_io::read_file(&file_path));
    if let Ok(read_contents) = block_on(futures_exp::file_io::read_file(&file_path)){
        println!("Contents from the file {} : {}",&file_path,read_contents);
    }
    let _ = block_on(futures_exp::file_io::write_file(&file_path, &contents_to_write));
    println!("Total time elapsed is : {:?}", start_time.elapsed());

}