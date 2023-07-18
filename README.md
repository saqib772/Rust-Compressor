# File Compression in Rust
This Rust code demonstrates file compression using the flate2 crate. It compresses a source file and outputs the compressed file.

# Prerequisites
Before running this code, make sure you have Rust installed on your system. You can install Rust by following the official installation guide: Rust Installation Guide.

# Usage
To compress a file, provide the source file path and the target file path as command-line arguments when running the program.

```
$ cargo run -- <source> <target>
```
Replace <source> with the path to the source file you want to compress, and <target> with the desired path for the compressed output file.

# Code Explanation
```
extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;
```
The code begins by importing the necessary dependencies. It uses the flate2 crate for file compression and various types from the standard library for I/O operations and time measurement.


```
fn main() {
    if args().len() != 3 {
        eprintln!("Usage: `source` `target`");
        return;
    }
```
The main function is the entry point of the program. It first checks if the number of command-line arguments is equal to 3. If not, it prints an error message indicating the correct usage and returns.

```
    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    let output = File::create(args().nth(2).unwrap()).unwrap();
```
 The code opens the source file specified in the command-line arguments and creates a buffered reader (BufReader) for efficient reading. It also creates the output file specified in the arguments.

```
    let mut encoder = GzEncoder::new(output, Compression::default());
    let start = Instant::now();
```
A Gzip encoder (GzEncoder) is created with the output file and the default compression level. An Instant object is created to measure the time taken for compression.

```
    copy(&mut input, &mut encoder).unwrap();
```
The copy function is used to read data from the input file and write it to the encoder, which performs the compression. The function returns a Result indicating whether the operation was successful. If an error occurs, it is unwrapped and the program terminates.

```
    let output = encoder.finish().unwrap();

    println!("Source len: {:?}", input.get_ref().metadata().unwrap().len());
    println!("Target len: {:?}", output.metadata().unwrap().len());
    println!("Elapsed: {:?}", start.elapsed());
}
```
After the compression is complete, the finish method is called on the encoder to flush any remaining data and obtain the compressed output. The metadata of the source and target files are printed, displaying their respective lengths. Finally, the elapsed time for compression is printed.

