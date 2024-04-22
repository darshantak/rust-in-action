#![allow(unused)]
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

fn grep_lite_with_vectors(){
    let context_lines = 1;
    let needle = "same";
    let haystack = "\
    Every face, every shop,
    bedroom window, public-house, and 
    dark square is a picture
    feversihly turned--in search of what?
    It is the same with books.
    What to we seek
    through millions of pages.";

    let mut tags : Vec<usize> = Vec::new();
    let mut ctx : Vec<Vec<(usize,String)>> = Vec::new();

    for (i,line) in haystack.lines().enumerate(){
        if line.contains(needle){
            tags.push(i);

            let v = Vec::with_capacity(2*context_lines+1);
            ctx.push(v);           
        }
    }

    if tags.len()==0{ //can use `tags.is_empty()``
        return;
    }

    for (i,line) in haystack.lines().enumerate(){
        for (j,tag) in tags.iter().enumerate(){
            let lower_bound = tag.saturating_sub(context_lines);
            let upper_bound = tag + context_lines;
            
            if (i>=lower_bound) && (i<=upper_bound){
                let line_as_string = String::from(line);
                let local_ctx = (i,line_as_string);
                ctx[j].push(local_ctx);
            }
        }
    }
    for local_ctx in ctx.iter(){
        for &(j,ref line) in local_ctx.iter(){
            let line_num = j+1;
            println!("{} {}",line_num,line);
        }
    }
}
fn main() {
    println!("Grep-Lite");
    // grep_lite();
    // playing_with_arrays();
    grep_lite_with_vectors();
}
