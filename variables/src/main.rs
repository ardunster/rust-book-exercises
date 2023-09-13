fn main() {
    let mut xavier = 5;
    println!("value of x: {xavier}");
    xavier = 6;
    println!("value of x: {xavier}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("It's very important you know. Three hours worth of seconds: {THREE_HOURS_IN_SECONDS}");

    let xandrew = 12;

    let xandrew = xandrew + 4;

    {
        let xandrew = xandrew * 3;
        println!("Xandrew in the inner scope: {xandrew}");
    }

    println!("Xandrew in the outer scope: {xandrew}");

//     Destructuring
    let tup = (100, 7.4, 2);

    let (x, y, z) = tup;
    println!("destructured values: {x}, {y}, {z}");
    println!("direct access: {}, {}, {}", tup.0, tup.1, tup.2);


//     println with named args
    println!("Test some {1}, and {0}", "things", "stuff");
    println!("Testing some{thing} else, you know, {stuff}", stuff = "stuff", thing = "thing");
    println!("Can we {} the {stuff}? If so, how does it {}?", "combo", "work", stuff = "stuff");

    // Arrays
    // [type; length]
    let _a: [i32; 5] = [1, 2, 3, 4, 5];
    let months: [&str; 12] = ["jan", "feb", "mar", "april", "may", "june", "july", "aug", "sep", "oct", "nov", "dec"];
    // array indexing is different than tuple indexing
    println!("{}", months[1]);
    // generate array all the same
    let samesies = ["it's the same"; 20];
    println!("is it all the same? {}", samesies[19]);
}
