use std::fs::File;

use glob1rs::legacy::map;

fn main() {
	env_logger::init();
	let file_name = std::env::args().nth(1).expect("Missing map filename");
	let file = File::open(file_name).expect("Cannot open map filename");
	let _map = map::load(file).expect("Error reading map");
}