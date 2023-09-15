fn main() {
    {
        let mut s = String::from("well hi there");

        s.push_str(".... if you're into that kind of thing");

        println!("{s}");

        // Compile error, Copy trait is not implemented for String
        // let s2 = s;

        // This works when first String is immutable, but then s3 can't be accessed anymore - s4 has borrowed it.
        let s3 = String::from("Steve");
        println!("s3: {s3}");
        let s4 = s3;
        println!("s4: {s4}");
        // println!("s3: {s3}");
    }
    // Compile error, out of scope.
    // println!("{s}");
    {
        let s1 = String::from("pistachio");
        println!("s1: {s1}");
        {
            let s2 = s1;
            println!("s2: {s2}");
        }
        // Still can't access s1 here, s2 borrowed it, s1 is gone, value poof
        // println!("s1: {s1}");
    }

    {
        let s1 = String::from("pine nut");
        let s2 = s1.clone();
        println!("s1: {s1}, s2: {s2}");
    }

    let steve = String::from("steve");
    takes_ownership(steve);
    // Not valid, steve has been dropped cruelly after ownership was taken D:
    // println!("{steve}");

    let numb = 12;
    makes_copy(numb);
    println!("{numb}");
}

fn takes_ownership(input_string: String) {
    println!("input was: {input_string}");
}

fn makes_copy(input_number: i32) {
    println!("input was: {input_number}");
}
