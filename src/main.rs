mod storage;

use std::path::Path;
use storage::create_file;

fn main() {
    println!("Hello, world!");
    let path: &Path = Path::new("./testfile.bin");
    create_file(path);
}
