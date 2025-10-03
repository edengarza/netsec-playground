use std::fs::{self, File}; // for file manip
use std::io::{BufReader, BufRead, Result, Lines}; // for the buffer itself
use std::path::{Path, PathBuf}; // for filenames

// the file reading comes from the rust-lang docs
// https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html

// helper function to return a line by line iterator of a file
// P is a generic type constrained to AsRef<Path>, allowing strings or paths
fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>> where P: AsRef<Path>, {
    // open the file, but this is wrapped in a result
    // ? will attempt to unwrap and if not it quickly returns an err
    let file = File::open(filename)?;

    // return wrapped so that caller can match
    // BufReader will buffer the file efficiently (less internal allocs)
    // lines() converts the output into an interator by newlines
    Ok(BufReader::<File>::new(file).lines())
}

fn get_files_in_dir(dirpath: &Path) -> Result<Vec<PathBuf>> {
    // the vector to hold the valid paths I want
    let mut files = Vec::new();
    
    // use read_dir() to get an iterator over entries in a dir
    if dirpath.is_dir() {
        for entry in fs::read_dir(dirpath)? {
            let entry = entry?;
            let metadata = entry.metadata()?;

            // act differently based on type
            if !metadata.is_dir() {
                files.push(entry.path());
            }
        }
    }

    // return the vector
    Ok(files)
}

// helper function to take an iterator of lines from a buffer and do actions per line
fn process_lines(lines: Lines<BufReader<File>>) {
    // consume the iterator which returns a Result<String, io::Error>
    // map_while() auto stops iterating when an error is encountered
    for (idx, line) in lines.map_while(Result::ok).enumerate() {
        // action on each line, already unwrapped as String
        println!("line {}: {}", idx, line);
    }
}

fn main() {
    // path to explore
    let dirpath: &Path = Path::new("test_dir");

    // enumerate a directory and return values in Result<Vec<PathBuf>>
    println!("Enumerating directory {}:", dirpath.display());
    let files = get_files_in_dir(&dirpath)
        .expect("Should be able to open dirpath");

    // printing filepaths
    for file in files {
        println!("{}", file.display());
        
        // read a file and return Result<> holding an iterator 
        let lines = read_lines(file)
            .expect("Could not open {file}");

        // perform some action on the lines, expecting iterator
        process_lines(lines);
        println!();
    }
}
