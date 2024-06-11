#[allow(unused)]
use futures::executor::block_on;

pub fn intro(){
    println!("Async Programming");

}
#[allow(unused)]
pub async fn learn_song() -> String{
    String::from("learn_song function is called.")
}
#[allow(unused)]
pub async fn sing_song(song:String) {
println!("{}",song);
}
#[allow(unused)]
pub async fn dance() {
    println!("dance function is called.")
}
