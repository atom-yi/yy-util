use std::io::{BufRead, BufReader, BufWriter, Write};
use std::{path::PathBuf};
use std::fs::{self, File};

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "merge", about = "merge folder into one file")]
struct Merge {
    /// folder path
    folder: PathBuf,

    /// output file path
    output_file: PathBuf,

    /// file suffix, default .csv
    #[structopt(short, long, default_value=".csv")]
    suffix: String,
}

pub fn exec() {
    let merge = Merge::from_args();
    let files = fs::read_dir(merge.folder).expect("open folder error");
    let target_file = File::create(merge.output_file).expect("create output file error");
    let mut writer = BufWriter::new(target_file);
    for file_path in files {
        let entry_path = file_path.unwrap().path();
        let same_suffix = entry_path.as_path().to_str().unwrap().ends_with(merge.suffix.as_str());
        if entry_path.is_file() && same_suffix {
            println!("merge {}", entry_path.display());
            append_file(&entry_path, &mut writer);
        }
    }
    println!("merge success.");
}

pub fn append_file(file_path: &PathBuf, writer: &mut BufWriter<File>) {
    let src_file = File::open(file_path).expect("open file error");
    let reader = BufReader::new(src_file);
    for line in reader.lines().map(|x| x.unwrap()) {
        writer.write(line.as_bytes()).unwrap();
        writer.write("\n".as_bytes()).unwrap();
    }
    writer.flush().unwrap();
}