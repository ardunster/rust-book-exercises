fn main() {
    println!("Hello, world!");

    let x = 12;

    if x % 4 == 0 {
        println!("divisible by 4")
    } else if x < 17 {
        println!("less than 17");
    } else {
        println!("greater than 17");
    }

    let want_pizza = true;
    let food_to_get = if want_pizza { "pizza" } else { "borgir" };

    println!("We're getting {food_to_get} today!")
}
