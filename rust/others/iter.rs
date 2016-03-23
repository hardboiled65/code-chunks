// practice handling with iterator. e.g. find etc.

fn main() {
    let mut a = Vec::new();
    a.push(10);
    a.push(20);
    a.push(30);
    a.push(40);
    a.push(50);
    a.push(60);

    let mut it = a.iter();
    let found = it.find(|&&num| num == 30);
    println!("{:?}", found);

    it.next();  // now `it` points value 40

    let found = it.find(|&&num| num == 30);
    println!("{:?}", found);

    // not .next() version
    let mut it = a.iter();
    let found = it.find(|&&num| num == 40);
    println!("{:?}", found);
    // cannot find the value 40 again
    let found = it.find(|&&num| num == 40);
    println!("{:?}", found);

    ///////////////////////////////////////
    let mut b = Vec::new();
    b.push("Amy".to_string());
    b.push("Benjamin".to_string());
    b.push("Carol".to_string());
    b.push("Daniel".to_string());
    b.push("Elizabeth".to_string());
    b.push("Finn".to_string());
    b.push("Graham".to_string());

    // `next` method
    let mut it = b.iter();
    it.next();
    let benjamin: Option<&String> = it.next();
    println!("{}", benjamin.unwrap());

    // `count` method
    let c: usize = it.count();  // `it` is consumed
    println!("size: {}", c);    // not 7, but 5 since `count` method counts
                                // while next() returns something.
                                // in this state, `it` count from "Carol"

    // `last` method
    let mut it = b.iter();
    let l: Option<&String> = it.last();
    println!("last: {}", l.unwrap());

    // `chain` method
    // TODO: write code
}
