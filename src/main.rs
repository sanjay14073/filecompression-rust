extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;
fn main() {
    //boiler code
    //println!("Hello, world!");

    //now we have used a crate flate2 added in our dependency also
    //now we need to accept the file and compress it

    //we need a minium of 3 agruments form cli for this application to work
    //println!("{:?}",args().nth(1).unwrap());
    if args().len()!=3{
      println!("A minimum of 3 arguments are required in the format usage:source target");
      return;
      //You can also use panic but here i wont use it
      //panic!("Not enough of arguments.A minimun of 3 arguments are required");
    }
    let mut input=BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    let output=File::create(args().nth(2).unwrap()).unwrap();
    let mut encoder=GzEncoder::new(output,Compression::default());
    let start=Instant::now();
    copy(&mut input,&mut encoder).unwrap();
    let output=encoder.finish().unwrap();
    println!("Source length {:?}",input.get_ref().metadata().unwrap().len());
    println!("Target length {:?}",output.metadata().unwrap().len());
    println!("Elapsed {:?}",start.elapsed());
    println!("Compression Successful");
}