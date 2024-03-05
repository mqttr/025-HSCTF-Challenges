use std::fs;
use std::path::Path;
use std::fs::File;
use std::io::Write;

use rand::distributions::{Alphanumeric, DistString};

use obfstr::obfstr;

const BYTES_PER_FILE: usize = 320_002;
// const FLAG: &str = "flag{U28geW91IGZvdW5kIGEgbWFnbmV0}"
// regex_hint = .{10,}

const flags_in_files: usize = 10_000;
const files_in_flags: usize = 100;
const folders: usize = 100; 


fn main() {
    let base_path = Path::new("NULLIFY_CHALLENGE_NEEDLE_IN_HAYSTACK");

    println!("Total size will be: {:} bytes", ((flags_in_files*files_in_flags*folders*BYTES_PER_FILE) as u64));

    fs::create_dir(base_path).expect("Aborting... folder could not be made. Does it already exist?");



    for _ in 0..9 {
        let file_name = Alphanumeric.sample_string(&mut rand::thread_rng(), 10);
        let mut file = File::create(base_path.join(file_name)).unwrap();
        create_flagless_file(&mut file)
    }

    let file_name = Alphanumeric.sample_string(&mut rand::thread_rng(), 10);
    let mut file = File::create(base_path.join(file_name)).unwrap();
    create_flagged_file(&mut file, &obfstr!("flag{U28geW91IGZvdW5kIGEgbWFnbmV0}"));
}

fn create_flagless_file(file: &mut File) {
    for _ in 0..9_998 {
        let string =  Alphanumeric.sample_string(&mut rand::thread_rng(), 26);
        write!(file, "flag{{{}}}", &string).unwrap();
    }

    for _ in 0..2 {
        let string =  Alphanumeric.sample_string(&mut rand::thread_rng(), 27);
        write!(file, "flag{{{}}}", &string).unwrap();
    }

}

fn gen_flag() -> String {
    "gag".to_string()
}



fn create_flagged_file(file: &mut File, flag: &str) {
    let val = 5821;
    for _ in 0..5821 {
        let string =  Alphanumeric.sample_string(&mut rand::thread_rng(), 26);
        write!(file, "flag{{{}}}", &string).unwrap();
    }

    write!(file, "{}", flag).unwrap();

    for _ in 0..9_999-val {
        let string =  Alphanumeric.sample_string(&mut rand::thread_rng(), 26);
        write!(file, "flag{{{}}}", &string).unwrap();
    }
}