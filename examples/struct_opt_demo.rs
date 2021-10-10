use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "hello", about = "struct-opt crate test.")]
struct Hello {
    name: String,
}

fn main() {
    let hello = Hello::from_args();
    println!("hi {}, nice to meet you!", hello.name);
}