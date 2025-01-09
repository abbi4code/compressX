use flate2::Compression;
use flate2::write::GzEncoder;
//This will be used to open a file or create a file
use std::fs::File;
//this will used for read and write traits
use std::io::{self, BufReader, copy};
use std::time::Instant;

//BufReader will wrap our reader(File) with a buffer to imp performance and instead of reading small chunks of data from underlying reader it read larger chunk of data into memory to reduce system calls
//copy function will used to copy all data from one reader (like buffer or file ) to a writer(another file)

//A buffer is a temp storage area in memory that hold data while transfering it between two places 
// like disk to memory (reading file)
// memory to disk (writing a file)

//what happens in buffer is data is read/written in larger chunks, and smaller portions are served from the buffer in memory.

pub fn compress(source: &str, target: &str ) -> io::Result<()> {
    let mut input = BufReader::new(File::open(source)?);
    //this input is a wrapper around the object
    //this question mark ensure we get the file instead getting a result with <File, error>
    //format! macro in rust used to create a formatted string similar to printf in C
    //eg
    //let formatted = format!("Hello, {}!", "World");
    // `formatted` now contains "Hello, World!"
    let output_file_name = format!("{}.gz", target);
    //now creating a file with this file name
    let output = File::create(output_file_name)?;

    // so now we will use the GzEncoder to compress the output data while we copy the file from source to target one by one and this will be compress in the output as well one by one untill all files are copied

    let mut encoder = GzEncoder::new(output, Compression::best());

    let start = Instant::now();

    copy(&mut input, &mut encoder)?;

    let output = encoder.finish()?;
    //get_ref() will call is used to access the ori obj (file)
    println!("Source Len: {:?}", input.get_ref().metadata()?.len());
    println!("Target Len: {:?}", output.metadata()?.len());
    println!("Elapsed Time: {:?}", start.elapsed());

    Ok(())

}