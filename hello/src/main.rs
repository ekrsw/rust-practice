fn main() {
    let s1 = "Hello";
    let s2: &str = &s1[..];

    if s1 == s2 {
        println!("s1 and s2 are equal");
    } else {
        println!("s1 and s2 are not equal");
    }

}