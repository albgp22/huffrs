mod utils;
mod huffman;

use utils::stats;
use huffman::huffman_engine;

fn main() {
    println!("Hello, world!");
    huffman_engine::compute_huffman_code("");
}
