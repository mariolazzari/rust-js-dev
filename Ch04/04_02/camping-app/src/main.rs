fn main() {
    // macro
    let packing_list = vec!["sunglasses", "sunscreen", "hat"];
    println!("{:?}", packing_list);

    // Vector
    let mut packing_list = Vec::new();
    packing_list.push("One");
    packing_list.push("Two");
    packing_list.push("Three");
    println!("Vector: {:?}", packing_list);
}
