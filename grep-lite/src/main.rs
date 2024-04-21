fn grep_lite(){
    let search_term = "picture";
    let quote ="
    Every face, every shop, bedroom window, public-house,
    and dark square is a picture
    fevershily turmed--in search of what ? It is the same with book.
    What do we seek through millions of pages ? ";

    for line in quote.lines(){
        if line.contains(search_term){
            println!("{}",line);
        }
    }
}
fn main() {
    println!("Grep-Lite");
    grep_lite();
}
