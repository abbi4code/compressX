use std::{env::args, process};
mod archive;

fn main(){

    if args().len() != 4 {
        eprintln!("Usage: `program compress|decompress source target`");
        process::exit(1);
    }

    let operation = args().nth(1).unwrap();
    let source= args().nth(2).unwrap();
    let target = args().nth(3).unwrap();

    //converting the op into as_str()
    match operation.as_str(){
        "compress" => {
            //if let as here compress fn will result a option or result
            if let Err(e) = archive::compress::compress(&source, &target) {
                eprintln!("Error compressing: {}", e)
            }
        },
        "decompress" => {
            if let Err(e) = archive::decompress::decompress(&source, &target) {
                eprintln!("Error while decompressing {}",e);
            }
        },
        _ => {
            eprintln!("Invalid operation. Choose 'compress' or 'decompress' ");
            process::exit(1);
        }
    }
}