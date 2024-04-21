fn grep_lite() {
    let search_term = "picture";
    let mut line_number = 1;
    let quote = " picture
    and dark square is a picture
    Every face, every shop, bedroom window, public-house,
    fevershily turmed--in search of what ? It is the same with book.
    What do we seek through millions of pages ? ";

    for line in quote.lines() {
        if line.contains(search_term) {
            println!("{} in line number {}", line, line_number);
        }
        line_number += 1;
    }

    for (i, line) in quote.lines().enumerate() {
        let line_num = i + 1;
        if line.contains(search_term) {
            println!(
                "{} found in line {} with line number {}",
                search_term, line, line_num
            );
        }
    }
}

fn playing_with_arrays() {
    let one = [1, 2, 3];
    let two: [u16; 3] = [1, 2, 3];
    let blank1 = [0; 3];
    let blank2: [u16; 3] = [0; 3];
    let arrays = [one,two,blank1,blank2];
    for a in &arrays{
        print!("{:?} : ",a);
        for item in a.iter(){
            print!("\t{} + 10= {} ",item,item+10);
        }
        let mut sum = 0;
        for item in a.iter(){
            sum += item;
        }
        println!("\tSum of the array is {}",sum)
    }
    
}
fn main() {
    println!("Grep-Lite");
    grep_lite();
    playing_with_arrays();
}
