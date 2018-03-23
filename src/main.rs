use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

fn main() {
//struct Sentence {
//    id: u32,
//    clause: u64,
//}

//struct Block {
//    timestamp: ,
//    source_host: ,
//    data: ,
//    prev_hash: ,
//    nonce: ,
//}

let mut hasher = DefaultHasher::new();
hasher.write(b"sv0001");

println!("Hash is {:x}!", hasher.finish());
}
