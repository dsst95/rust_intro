use itertools::Itertools;

fn main() {
    control_flow();
    assert_eq!(
        convert_celsius_to_fahrenheit(convert_fahrenheit_to_celsius(20.0)),
        20.0
    );
    assert_eq!(fibonacci(50), 12_586_269_025);
    twelve_days_song();
}

fn control_flow() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn convert_fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn convert_celsius_to_fahrenheit(celsius: f32) -> f32 {
    celsius * 9.0 / 5.0 + 32.0
}

fn fibonacci(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }

    let mut f1: u64 = 0;
    let mut f2: u64 = 1;

    for _ in 2..n {
        let val = f1;
        f1 = f2;
        f2 = f1 + val;
    }

    f1 + f2
}

fn twelve_days_song() {
    let days = [
        "first", "second", "firth", "fourth", "fifth", "sixth", "seventh", "eight", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let gifts = [
        "And a partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four colly birds",
        "Five gold rings",
        "Six geese a laying",
        "Seven swans a swimming",
        "Eight maids a milking",
        "Nine drummers drumming",
        "Ten pipers piping",
        "Eleven ladies dancing",
        "Twelve lords a leaping",
    ];

    for i in 0..11 {
        println!("On the {} day of Christmas,", days[i]);
        println!("My true love gave to me:");

        if i == 0 {
            println!("A partridge in a pear tree.");
        } else {
            let mut gifts_slice = gifts[0..i + 1].iter().cloned().rev();
            println!("{}.", Itertools::join(&mut gifts_slice, ",\n"));
        }

        println!("");
    }
}
