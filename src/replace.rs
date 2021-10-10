use std::{fs::File, io::{prelude::*, BufReader, BufWriter}, path::PathBuf};

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "replace", about = "file '|' to ',' splitter.")]
struct Trans {
    /// src file path
    src_file: PathBuf,

    /// output file path
    output_file: PathBuf,

    /// src file splitter, default '|'
    #[structopt(short, long, default_value = "|")]
    src_splitter: String,

    /// target splitter, default ','
    #[structopt(short, long, default_value = ",")]
    target_splitter: String,
}

pub fn exec() {
    println!("trans start.");
    let trans = Trans::from_args();
    let src_file = File::open(trans.src_file).expect("open src file error");
    let target_file = File::create(trans.output_file).expect("create output file error");
    let reader = BufReader::new(src_file);
    let mut writer = BufWriter::new(target_file);
    for line in reader.lines().map(|x| x.unwrap()) {
        let mut new_line = line.replace(trans.src_splitter.as_str(), trans.target_splitter.as_str());
        new_line.push('\n');
        writer.write(new_line.as_bytes()).expect("write into target error.");
    }
    println!("trans success.");
}
