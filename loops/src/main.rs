fn main() {
    println!("Hello, world!");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 21 {
            break counter * 2;
        }
    };

    println!("we counted to {counter} and the result was {result}");
}
