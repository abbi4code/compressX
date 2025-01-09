use flate2::read::GzDecoder;
use std::fs::File;
use std::io::{BufReader,copy, self};
use std::time::Instant;

pub fn decompress(source: &str, target: &str) -> io::Result<()>{
    let input = BufReader::new(File::open(source)?);
    let input_metadata = input.get_ref().metadata();
    let output_file_name = target;
    let mut output = File::create(output_file_name)?;

    //this will take the data for which it will be decoding one by one
    let mut decoder = GzDecoder::new(input);

    let start = Instant::now();

    copy(&mut decoder, &mut output)?;

    println!("Input Len: {:?}", input_metadata?.len());
    println!("Ouput Len: {:?}", output.metadata()?.len());
    println!("Elapsed TIme: {:?}", start.elapsed());

    Ok(())
}
