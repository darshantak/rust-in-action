use tokio::fs::{File, OpenOptions};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use std::env;
use tokio::time::Instant;
//tokio::io::Result<T> is a type alias for the I/O ops specifically. It assumes that the error type is io::Error and it takes care of this by itself only.
pub async fn read_file(file_path: &str) -> io::Result<String> {
    println!("Read file function is called with file {}", file_path);
    let mut file: File = match File::open(file_path).await {
        Ok(file) => {
            println!("File opened successfully: {:?}", file_path);
            file
        }
        Err(e) => {
            eprintln!("Failed to open file: {}. Error: {}", file_path, e);
            return Err(e);
        }
    };
    
    let mut contents = String::new();
    if let Err(e) = file.read_to_string(&mut contents).await {
        eprintln!("Failed to read file: {}. Error: {}", file_path, e);
        return Err(e);
    }
    
    Ok(contents)
}
pub async fn write_file(file_path: &str, contents: &str) -> io::Result<()>{
    println!("Write File function is called");
    let mut file = OpenOptions::new()
    .write(true)
    .create(true)
    .open(file_path)
    .await?;

    file.write_all(contents.as_bytes()).await?;

    Ok(())
}

pub async fn file_example() -> io::Result<()> {
    let start_time = Instant::now();
    let file_path = "example.txt";
    let contents = "Hello this is an example of asynchronous programming.";
    println!("Current working directory: {:?}", env::current_dir().unwrap());

    let read_contents = read_file(&file_path).await?;
    write_file("new_example.txt", contents).await?;
    println!("Total time elapsed is : {:?}", start_time.elapsed());
    Ok(())
}