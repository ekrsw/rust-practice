fn main() {
    let five = Some(5);
    let six = plus_one(five);
    println!("five is {:?}", five);
    println!("six is {:?}", six);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}