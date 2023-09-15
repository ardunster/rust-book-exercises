fn main() {
    println!("Hello, {{world!}}");

    some_other_thing(7, ":O !!!!! ");

    let thing = returns_a_thing();

    println!("thing is: {thing}");

    println!("not return anything is: {:?}", not_return_anything());

    println!("add 42 to a thing: {}", add_forty_two(thing));
}

fn some_other_thing(input: i32, exclamation: &str) {
    println!("steve likes the number {input} {exclamation}");

    let steve = {
        let x = 1;
        // Note no semicolon here!
        x + 3
    };

    println!("Steve has added up to {steve}");
}

fn returns_a_thing() -> i32 {
    6
}

fn not_return_anything() {
    6;
}

fn add_forty_two(input: i32) -> i32 {
    input + 42
}
