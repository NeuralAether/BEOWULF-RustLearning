use std::collections::HashMap; 

fn main() {
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);
    map.entry("d").or_insert(4);
    map.entry("a").or_insert(5); // not updated
    println!("{:?}", map);
}
