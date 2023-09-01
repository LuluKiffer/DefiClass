fn main() {
    let db: sled::Db = sled::open("user_storage_2").unwrap();
    let tree = db.open_tree("e2a").unwrap();
    for entry in &tree {
        if let Ok((email, eth_addr)) = entry {
            let email = std::str::from_utf8(&email).unwrap();
            let eth_addr = std::str::from_utf8(&eth_addr).unwrap();
            println!("{}: {}", email, eth_addr);
        }
    }
}
