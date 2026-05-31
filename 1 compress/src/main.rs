extern crate flate2;

use flate2::Compression;
use flate2::write::GzEncoder;
use std::env::args;
use std::fs::File;
use std::io::BufReader; //Read the file
use std::io::copy; //Copy the file to the encoder
use std::time::Instant; //Time the compression shows the time taken to compress the file

fn main() {
    if args().len() != 3 {
        eprint!("Usage : `Source` `Target`");
        return;
    }

    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap()); //Open the file and create a buffered reader
    let output = File::create(args().nth(2).unwrap().unwrap()).unwrap(); //Create the output file
    let mut encoder = GzEncoder::new(output, Compression::default()); //Create a new GzEncoder with the output file and default compression level
    let start = Instant::now();

    copy(&mut input, &mut encoder).unwrap(); //Copy the input file to the encoder and compress it
    let output = encoder.finish().unwrap(); //Finish the compression and get the output file

    println!(
        "Source length : {:?}",
        input.get_ref().matadata().unwrap().len()
    );
    print!("Target length : {:?}", output.metadata().unwrap().len());
    println!("Elasped Time : {:?}", start.elapsed()); //Print the elapsed time
}
