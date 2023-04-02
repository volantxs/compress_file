use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::{BufReader, BufWriter};
use std::time::Instant;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 3 { 
        eprintln!("Usage: 'Input' 'Output'"); 
        return;
    }
    let mut input = BufReader::new(File::open(&args[1]).unwrap());
    let output = BufWriter::new(File::create(&args[2]).unwrap());
    let mut encoder = GzEncoder::new(output, Compression::default());
    let start = Instant::now();
    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();
    println!(
        "source len: {:?}",
        input.get_ref().metadata().unwrap().len()
        );
    println!("Target len: {:?}", 
            output.get_ref().metadata().unwrap().len()
        );
    println!("Elapsed: {:?}", start.elapsed());
}
