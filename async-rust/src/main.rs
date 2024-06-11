mod modules;
use modules::async_exp;
use futures::executor::block_on;
fn main() {
    println!("Hello, world!");
    async_exp::intro();
    let song = block_on(async_exp::learn_song());
    block_on(async_exp::sing_song(song));
    block_on(async_exp::dance());
}

