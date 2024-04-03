use std::sync::mpsc;

use image_compressor::Factor;
use image_compressor::FolderCompressor;

fn main() {
    let path = std::env::args().nth(1).expect("no pattern given");
    let out = std::env::args().nth(2).expect("no path given");
    let threds = 4;
    let (tx, tr) = mpsc::channel();

    let mut comp = FolderCompressor::new(path, out);
    comp.set_factor(Factor::new(80., 0.8));
    comp.set_thread_count(threds);
    comp.set_sender(tx);
    match comp.compress() {
        Ok(_) => {}
        Err(e) => println!("cannot compres {}", e),
    }
}
