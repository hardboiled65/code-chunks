// practice handling with iterator. e.g. find etc.

fn main() {
    let mut a = Vec::new();
    a.push(10);
    a.push(20);
    a.push(30);
    a.push(40);

    let mut it = a.iter();
    let found = it.find(|&&num| num == 30);
    println!("{:?}", found);
}
