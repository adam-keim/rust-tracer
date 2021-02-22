fn main() {
    let width = 256;
    let height = 256;

    println!("P3\n{} {}\n255", width, height);
    for j in height..0 {
        println!("{}", j);
    }
}
