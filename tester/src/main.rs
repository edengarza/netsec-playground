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
    decipher_block,
    encipher_block,
    generate_key_schedule
}; // for des library

// +-----------+
// |  Structs  |
// +-----------+
struct TestConfig {
    id: u64,
    key: u64,
    plaintext: u64,
    ciphertext: u64,
    encrypting: bool
}

// +-----------+
// |  Helpers  |
// +-----------+

// print out a formatted TestConfig
// commented out to avoid warning
// fn print_test_config(config: &TestConfig) {
//     println!("id: {}\nkey: 0x{:016x}\nplaintext: 0x{:016x}\nciphertext: 0x{:016x}\nencrypting: {}", config.id, config.key, config.plaintext, config.ciphertext, config.encrypting);
// }

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

    // value to determine the op
    let mut encrypting: bool = false;

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
                // set the encrypting variable to know what operation to do
                if line.contains("DECRYPT") {
                    encrypting = false;
                } else {
                    encrypting = true;
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
                encrypting: encrypting,
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
    let key_schedule: [u64; 16] = generate_key_schedule(&test.key);
    let expected: u64 = test.ciphertext;
    let actual: u64;
    if test.encrypting {
        actual = encipher_block(&test.plaintext, &key_schedule);
    } else {
        actual = decipher_block(&test.plaintext, &key_schedule);
    }
    //print_test_config(&test);
    println!("[{:>03}] Expected: 0x{:016x}, Actual: 0x{:016x}", test.id, expected, actual);

    // actual return, uncomment when done testing and want to actually use this
    actual == expected
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
