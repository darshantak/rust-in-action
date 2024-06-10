use futures::future::join_all;
use std::{io, result};
use std::net::TcpStream;
use tokio::net::TcpStream as tokiotcp;
use tokio::task;

fn primitive_sync() {
    println!("TCP Scanner from scratch ");
    for port in 1..100 {
        let address = format!("scanme.nmap.org:{}", port);
        println!("Trying on port {}", port);
        match TcpStream::connect(address) {
            Ok(_) => {
                println!("{} port is open", port);
            }
            Err(_) => {
                continue;
            }
        }
    }
}

async fn scan_port(port: u16) -> u16 {
    let address = format!("scanme.nmap.org:{}", port);
    match tokiotcp::connect(address).await {
        Ok(_) => port,
        Err(_) => 0,
    }
}


#[tokio::main]
async fn main() {
    println!("TCP Scanner Sync and Async Impls");
    let mut tasks = Vec::new();
    for port in 1..=24{
        tasks.push(task::spawn(scan_port(port)));
    }
    
    let result = join_all(tasks).await;
    // println!("{:?}", result);
    for (port,result) in (1..=24).zip(result)  {
        println!("{},{:?}",port,result);
        if let Ok(open_port) = result {
            if open_port!=0{
                println!("{} is open", open_port);
            }
        }
    }
}
