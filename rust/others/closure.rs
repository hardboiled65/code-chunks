fn main() {
    let greeting = "Bonne nuit!".to_string();

    let greet = || println!("{}", greeting);

    greet();
    greet();

    ////////////////////////////////////////
    let mut answer = 42;    // <----------------------|

    {   // <--- `answer` in this scope differs from 'this'
        let mut increase = move || {
            answer += 1;
            println!("{}", answer);
        };

        increase();
        increase();
    }
    println!("{}", &answer);
}
