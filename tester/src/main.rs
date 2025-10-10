// +-----------+
// |  Imports  |
// +-----------+
use std::fs::{
    self,
    File
}; // for file manip
use std::io::{
    BufReader,
    BufRead,
    Result,
    Lines
}; // for the buffer itself
use std::path::{
    Path,
    PathBuf
}; // for filenames
use std::env;
use des::{
    initial_permutation
}; // for des library

// +-----------+
// |  Structs  |
// +-----------+
struct TestConfig {
    id: u64,
    key: u64,
    plaintext: u64,
    ciphertext: u64,
}

// +-----------+
// |  Helpers  |
// +-----------+

// print out a formatted TestConfig
fn print_test_config(config: &TestConfig) {
    println!("id: {}\nkey: {}\nplaintext: {}\nciphertext: {}\n", config.id, config.key, config.plaintext, config.ciphertext);
}

// split a string into its non-whitespace components
fn tokenize_line(line: &str) -> Vec<String> {
    // create the return object
    let mut tokens: Vec<String> = Vec::new();

    // iterate over each token and add to return
    for token in line.split_whitespace() {
        tokens.push(String::from(token));
    }

    tokens
}

// determine whether a given string is a valid dir
fn path_exists_and_is_dir(path: &str) -> bool {
    Path::new(path).is_dir()    
}

// function to check the given args
fn validate_args(args: &Vec<String>) {
    if args.len() == 1 {
        panic!("Did not provide any path to explore!");
    }
    if args.len() > 2 {
        panic!("Gave too many paths!");
    }
    if !path_exists_and_is_dir(&args[1]) {
        panic!("The given path was not a valid directory!");
    }
}

// enumerate files in a directory and return vector of string paths
fn get_files_in_dir(dirpath: &Path) -> Vec<String> {
    // the vector to hold the valid paths I want
    let mut files: Vec<String> = Vec::new();
    
    // use read_dir() to get an iterator over entries in a dir
    if let Ok(entries) = fs::read_dir(dirpath) {
        for entry in entries.flatten() {
            // get file information
            let os_name = entry.file_name();
            let name = os_name.to_string_lossy();

            // get rid of annoying emacs files
            let autosave: bool = name.starts_with("#") && name.ends_with("#");
            let backup: bool = name.ends_with("~");
            if autosave || backup {
                continue;
            }
            
            // only add to the vector if it's not another dir
            let metadata = entry.metadata().expect("Could not access file metadata.");
            if !metadata.is_dir() {
                files.push(name.to_string());
            }
        }
    }

    // return the vector
    files
}

// +--------------+
// |  Core Logic  |
// +--------------+

// function to gather all lines in a buffer
fn read_lines(filename: &PathBuf) -> Lines<BufReader<File>> {
    // open the file, but this is wrapped in a result
    let file = File::open(filename).expect("Could not open given path");

    // return an iterator of the lines in the file
    BufReader::<File>::new(file).lines()
}

// function to act on the file line by line
fn process_lines(lines: Lines<BufReader<File>>) -> Vec<TestConfig> {
    // dict to hold values
    let mut tests: Vec<TestConfig> = Vec::new();

    // vector to hold relevant values
    let mut temp_hold: Vec<String> = Vec::new();
    
    // go over each line in the file we read
    // map_while() auto stops iterating when an error is encountered
    for line in lines.map_while(Result::ok) {
        // skip empty strings
        if line.is_empty() {
            continue;
        }
        
        // determine action based on how the line starts
        if line.starts_with("COUNT") ||
            line.starts_with("KEYs") ||
            line.starts_with("PLAINTEXT") ||
            line.starts_with("CIPHERTEXT") {
                // when a known prefix, add the token
                let tokens: Vec<String> = tokenize_line(&line);

                // only push the value onto the temp hold
                temp_hold.push(tokens[2].clone());
            } else if line.starts_with("[") {
                // identify the sections, idk if i'm doing anything with them at the moment
                // println!("Section: {}", line);

                // for now, only focus on encryption
                if line.contains("DECRYPT") {
                    break;
                }
            } else {
                // if not a pre-established beginning, just skip
                continue;
            }

        // when an entire case has been found, form the struct
        if temp_hold.len() == 4 {
            // take the values in temp hold and create a test config
            let test_config = TestConfig {
                id: u64::from_str_radix(&temp_hold[0], 16).unwrap(),
                key: u64::from_str_radix(&temp_hold[1], 16).unwrap(),
                plaintext: u64::from_str_radix(&temp_hold[2], 16).unwrap(),
                ciphertext: u64::from_str_radix(&temp_hold[3], 16).unwrap(),
            };

            // clear the temp hold
            temp_hold.clear();

            // store the config
            tests.push(test_config);
        }
    }

    // return the test parsed test configs
    tests
}

fn run_test(test: &TestConfig) -> bool {
    let expected: u64 = test.ciphertext;
    let actual: u64 = initial_permutation(&test.plaintext);
    // print_test_config(&test);
    println!("[{:>03}] Expected: {:25}, Actual: {:25}", test.id, expected, actual);

    true
    // actual return, uncomment when done testing and want to actually use this
    // actual == expected
}

// +--------+
// |  Main  |
// +--------+

// core logic driver
fn main() {
    // ensure the given args are valid
    let args: Vec<String> = env::args().collect();
    validate_args(&args);
    
    // path to explore
    let dirpath: &Path = Path::new(&args[1]);

    // enumerate the files in given directory
    let files = get_files_in_dir(&dirpath);
    
    // for each file present in the directory, extract the test configs
    let mut tests: Vec<TestConfig> = Vec::new();
    for file in files {
        // full the path out
        let full_path = dirpath.join(file);
        
        // read a file and return Result<> holding an iterator 
        let lines = read_lines(&full_path);
        
        // perform some action on the lines, expecting iterator
        tests.extend(process_lines(lines));

        // sanity check prints of test configs
        // for test in &tests {
        //     print_test_config(&test);
        // }
    }

    // perform tests using the configs
    println!("Starting test execution...");
    let mut success: bool = false;
    for test in &tests {
        success = run_test(&test);
        if !success {
            println!("Failed on test {}!", test.id);
            break;
        }
    }
    println!("Test execution finished.");

    // final print depends on the test
    if success {
        println!("Successful run :)");
    } else {
        println!("Unsuccessful run :(");
    }
}
