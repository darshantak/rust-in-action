use std::process::Command;
use std::fs::File;
use std::io::{self,Read};


fn main() {
    println!("Let's use Squoosh CLI to compress images.");
    let input_file = "input.jpg";
    let output_file = "output.jpg";

    let status = Command::new("squoosh-cli")
    .args(&[input_file,"--output",output_file,"--resize","{width:300}"])
    .status()
    .expect("Failed to execute squoosh cli");

    if status.success(){
        println!("Image compressed successfully");
        let mut file = File::open(output_file).expect("Failed to open the file");
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).expect("Failed to read the output file");

        println!("The size of the output file is {}",buffer.len());

    }else {
        println!("Image compression failed");
    }

}
