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

    println!("Xandrew in the outer scope: {xandrew}")
}
