// use flate2::bufread::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;

fn main() {
    if args().len() != 3 {
        eprintln!("Usage: `source` `target`");
        return;
    } 
    
    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());

    let output = File::create(args().nth(2).unwrap()).unwrap();

    let mut encoder = GzEncoder::new(output, Compression::default());

    let start = Instant::now();

    //this will copy the thing from the reader to writer continuously and will return you the no of bytes it copied
    copy(&mut input,&mut encoder).unwrap();

    let output = encoder.finish().unwrap();
    println!("{:?}",input);

    println!("Source Len: {:?}", input.get_ref().metadata().unwrap().len());

    println!("Target len: {:?}", output.metadata().unwrap().len());

    println!("Elapsed Time: {:?}", start.elapsed());

    // println!("args are {:?}", args());



}

