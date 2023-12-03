use std::collections::HashMap;
use std::fmt::format;
use std::iter::Rev;

fn main() {
    let number = 3;

    if number < 5 {
        println!("condition is true");
    } else{
        println!("condition is false");
    }

    if number % 4 ==0 { println!("number is divisible by 4") }
    else if number % 3 ==0 { println!("number is divisible by 3")  }
    else if number % 2 ==0 { println!("number is divisible by 2")}
    else { println!("number is neither a multiples of 4, 3 and 2") }

    let condition = true;
    if condition { println!("condition is true")}

    //loops

    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break
                counter * 2; //returns the value of counter * 2 to result
        }
    };

    println!("value of result at the end of loop is {result}");

   let message = count_up_and_down_loop(10, 5, 8);
    println!("{message}");

    let phone_book = ["Peter", "John", "Jude",
        "Matthew", "John the beloved",
        "Thomas", "Judas", "Andrew", "James", "Bartholomew"];

    print_all_names_in_phonebook_with_while_loop(phone_book);
    print_all_names_in_phonebook_with_enhanced_for_loop(phone_book);
    print_range_of_numbers(1, 10, true);

}


//Loop Labels to Disambiguate Between Multiple Loops
/****
If you have loops within loops, break and continue apply to the innermost loop at that point.
You can optionally specify a loop label on a loop that you can then use with break or continue
to specify that those keywords apply to the labeled loop instead of the innermost loop.
Loop labels must begin with a single quote
*/

fn count_up_and_down_loop(remaining_initial_value: i32, count_break:i32, remaining_break:i32) -> String {
    let mut message = String::new();
    let mut count = 0;

    let loop_condition = count_break > count && remaining_break < remaining_initial_value;
    if !loop_condition {
        message = String::from("for the loop to happen, \
        count break must be greater than 0 and remaining break less than remaining initial value");
        return message;
    }

    'main_loop: loop {
        let mut remaining = remaining_initial_value;

        loop {
            if remaining == remaining_break {
                println!("exiting inner loop at count down of {remaining}");
                break;
            }

            if count == count_break {
                println!("exiting main loop at count-up of {count}");
                break 'main_loop message = format!("we counted up to {count_break} and counted down to {remaining_break}")
            }
            remaining -= 1;
        }
        count += 1;
    };

    return message;
}

fn print_all_names_in_phonebook_with_while_loop (phone_book: [&str; 10]) {
    let mut name_position = 0;
    let phone_book_size = phone_book.len();
    while name_position < phone_book_size {
        println!("the name at position {} in phonebook is {}", name_position+1, phone_book[name_position]);
        name_position+=1
    }
}

fn print_all_names_in_phonebook_with_enhanced_for_loop (phone_book: [&str; 10]) {
    println!("names in phone book");
    for name in phone_book {
        println!("{name}");
    }
}

fn print_range_of_numbers (start: i32, end :i32, reverse: bool) {
    if start >= end {
        println!("invalid command! start range cannot be greater than or equal to end range");
        return;
    }

    if !reverse {
        println!("printing all numbers between {start} and {end} in ascending order");
        for num in start..end {
            println!("{num}")
        }
    } else {
        println!("printing all numbers between {start} and {end} in descending order");
        for num in (start..end).rev() {
            println!("{num}")
        }
    }
}

