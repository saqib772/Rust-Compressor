extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant; //This is for how much time it take to compress file



fn main(){

    if args().len()!=3{
        eprintln!("Usage :`source` `target`");
        return;
    }

    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    //Take the input of the file

    let output =File::create(args().nth(2).unwrap()).unwrap();
    // this is for the output file 

    let mut encoder = GzEncoder::new(output,Compression::default()); //encoder for the compression

    let start=Instant::now(); // Start: Time taken to compress

    copy(&mut input, &mut encoder).unwrap();// Take the input file and apply encoder for compression

    let output=encoder.finish().unwrap(); // give the compressed file in output var

    println!(
        "Source len : {:?}",
        input.get_ref().metadata().unwrap().len()
    ); // length of source file

    println!("Target len: {:?}",output.metadata().unwrap().len()); //lengt of compressed file 
    println!("Elapsed : {:?}",start.elapsed()); // time elapsed in seconds

}
