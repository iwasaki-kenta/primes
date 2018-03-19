#![feature(fs_read_write)]
extern crate rayon;

use rayon::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::fs;

fn main() {
    let output = process();
    println!("Extracted {:?} primes from all files.", output.len());

    fs::write("result.csv", output.join("\n").as_bytes()).unwrap();

    println!("Wrote to result.csv; program finished.");
}

fn process() -> Vec<String> {
    let files: Vec<String> = (1..51).into_iter().map(|i| format!("data/primes{}.txt", i)).collect();

    // Read and process batch files in parallel.
    let file_data: Vec<String> = files
        .par_iter()
        .map(|name| {
            let mut file: File = File::open(name).expect("Failed to open file");
            file.seek(SeekFrom::Start(75)).expect("Failed to seek file");

            let size = file.metadata().unwrap().len() as usize;
            let mut contents: String = String::with_capacity(size);

            file.read_to_string(&mut contents).expect("Failed to read file");

            return contents;
        })
        .flat_map(|raw: String| {
            // Pre-allocate 1,000,000 entries.
            let mut output: Vec<String> = Vec::with_capacity(1_000_000);

            let mut start = -1;

            for (index, chr) in raw.char_indices() {
                if chr >= '0' && chr <= '9' {
                    if start == -1 {
                        start = index as i32;
                    }
                } else {
                    if start != -1 {
                        output.push(raw[start as usize..index].to_string());
                        start = -1;
                    }
                }
            }

            return output;
        })
        .collect();

    return file_data;
}
