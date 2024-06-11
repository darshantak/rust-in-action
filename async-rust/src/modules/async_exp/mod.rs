use futures::executor::block_on;

pub fn intro(){
    println!("Async Programming");

}
pub async fn learn_song() -> String{
    String::from("learn_song function is called.")
}

pub async fn sing_song(song:String) {
println!("{}",song);
}

pub async fn dance() {
    println!("dance function is called.")
}
