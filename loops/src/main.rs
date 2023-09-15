fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 21 {
            break counter * 2;
        }
    };

    println!("we counted to {counter} and the result was {result}");

    let mut count_two = 0;
    'counting_upward: loop {
        println!("count is: {count_two}");

        let mut remaining = 10;
        loop {
            println!("remaining: {remaining}");
            if remaining == 9 {
                break;
            }
            if count_two == 2 {
                break 'counting_upward;
            }
            remaining -= 1;
        }

        count_two += 1;
    }

    println!("We counted a bunch and ended up at {count_two}");


    println!("TVB?");
    let mut three = 3;

    while three != 0 {
        println!("Three is {three}");
        three -= 1;
    }
    println!("GOGOGOGOGOGOGOGOGOGOGOGOGOGOSGOGOGOGOGOGOGOGOGOOGOGOOGOGOGOGOGGOGOGOGOGOGOGOGO");


    let array = [10, 7, 23, 14, 6, 17337, 0];
    let mut index = 0;
    while index < 7 {
        print!("Accessing array with a while loop like a goober.\nIndex: {index}\nValue: {}\n", array[index]);
        index += 1;
    }

    for element in array {
        println!("Accessing array with for like a chad: value {element}");
    }

    for number in (1..5).rev() {
        println!("We countses precious, we countses to {number}");
    }

    for number in 1..=5 {
        println!("we countses inclusively, we countses to {number} precioussss");
    }
}
