use std::net::TcpStream;
use std::io;

fn main() {
    println!("TCP Scanner from scratch ");
    for port in 1..100 {
        let address = format!("scanme.nmap.org:{}",port);
        println!("Trying on port {}",port);
        match TcpStream::connect(address) {
            Ok(_) =>{
                println!("{} port is open",port);
            }     
            Err(_) => {
                continue;
            }
        }
    }
}
