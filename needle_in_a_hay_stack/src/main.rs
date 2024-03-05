use std::fs;
use std::{path::PathBuf, path::Path};
use std::fs::File;
use std::io::Write;
use std::env;

use rand::distributions::{Alphanumeric, DistString};
use rand::Rng;

use obfstr::obfstr;

mod dialogue;

fn main() {
    let flag: &str = obfstr!("flag{U28geW91IGZvdW5kIGEgbWFnbmV0}");
    let root_folder: &Path = Path::new("HSCTF_Odd_Length_Flags");
    let folder_count: i32 = 20;
    let file_count: i32 = 20;
    let nested_depth_range: i32 = 7;

    if env::args().len() == 1 {
        let abs_path = env::current_dir().unwrap().to_str().unwrap().to_owned()+&root_folder.to_str().unwrap();
        dialogue::print_intro(&abs_path);
    } else {
        let abs_path = env::current_dir().unwrap().to_str().unwrap().to_owned()+"\\"+&root_folder.to_str().unwrap()+"\\";
        dialogue::print_intro_fast(&abs_path);
    }

    fs::create_dir(root_folder).expect("Could not create folder. Does it already exist?");

    for _ in 0..folder_count {
        let folder: PathBuf = root_folder.join(Alphanumeric.sample_string(&mut rand::thread_rng(), 10));
        create_flagless_folder(&folder, file_count, nested_depth_range);
    }

    let folder: PathBuf = root_folder.join(Alphanumeric.sample_string(&mut rand::thread_rng(), 10));
    create_flag_folder(&folder, file_count, nested_depth_range);
    
}

fn create_flag_folder(folder: &Path, file_count: i32, nested_count: i32) {
    fs::create_dir(folder).unwrap();

    for _ in 0..file_count {
        let new_file_path: PathBuf = folder.join(Alphanumeric.sample_string(&mut rand::thread_rng(), 10));
        let mut new_file: File = File::create(new_file_path).unwrap();
        write_flagless_file(&mut new_file);
    }

    if nested_count == 0 {
        return;
    }

    let next_folder: PathBuf = folder.join(Alphanumeric.sample_string(&mut rand::thread_rng(), 10));
    create_flag_folder(&next_folder, file_count, nested_count-1);
}

fn create_flagless_folder(folder: &Path, file_count: i32, nested_count: i32) {
    fs::create_dir(folder).unwrap();

    for _ in 0..file_count {
        let new_file_path: PathBuf = folder.join(Alphanumeric.sample_string(&mut rand::thread_rng(), 10));
        let mut new_file: File = File::create(new_file_path).unwrap();
        write_flagless_file(&mut new_file);
    }

    if nested_count == 0 {
        return;
    } else {
        for _ in 0..nested_count {
            let next_folder: PathBuf = folder.join(Alphanumeric.sample_string(&mut rand::thread_rng(), 10));
            create_flagless_folder(&next_folder, file_count, nested_count-1);
        }
    }
}

fn write_flagless_file(file: &mut File) {
    let flag_count: i32 = 200; 
    
    let mut flags = String::new();
    for _ in 0..flag_count {
        flags.push_str(&("flag{".to_owned()+&gen_flag()+"}"));
    };
    write!(file, "{}", flags).unwrap();
}

fn gen_flag() -> String {
    let mut rng = rand::thread_rng();
    let len: usize = 2*rng.gen_range(5..20)+1;
    Alphanumeric.sample_string(&mut rand::thread_rng(), len)
}