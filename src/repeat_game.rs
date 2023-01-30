use std::io;
use std::time::Instant;
use rand::Rng;

use crate::colors::*;

pub fn start() {
    let green:&str = &green() as &str;
    let red:&str = &red() as &str;
    let endcolor:&str = &endcolor() as &str;

    let mut rng = rand::thread_rng();
    let mut display:u8;
    let mut count:u8;


    let now = Instant::now();
    let mut fails:u8 = 0;
    let mut successes:u8 = 0;
    let mut stop = false;

    loop {
        display = rng.gen_range(0..=9);
        count = rng.gen_range(4..=7);

        // This loop allows the user to correct potential mistakes
        // by reusing the same generated numbers until they get it right
        loop {
            for _i in 1..=count {
                print!("{}", &display);
            }
            print!("\n");
            println!("Answer:");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Whoah there!");
            // Check for Stop
            if input.starts_with("s") {
                stop = true;
                break;
            } else {
                let input:u8 = input.trim()
                    .parse()
                    .expect("Not a number!");

                // Correct
                if input == count {
                    successes += 1;
                    println!("{green}Correct{endcolor}");
                    break;

                    // Incorrect
                } else {
                    fails += 1;
                    println!("{red}Incorrect{endcolor}");
                }
            }
            if (successes + fails) == (30 - 1) {
                stop = true;
            }
        }
        //failed_attempts = 0;
        if stop {break;} else {
            println!("The count was {}", count);
        }
    }
    let elapsed = now.elapsed().as_secs();
    println!("Time elapsed: {elapsed}sec");
    if fails != 0 {
        println!("{green}You answered correct {successes} times {endcolor}");
        println!("{red}You answered wrong {fails} times{endcolor}");
    } else {
        println!("{green}You answered everything correctly!!{endcolor}")
    }
    println!("Score: {}", (successes - fails) as u64 * 4 / elapsed); // Casting (successes - fails) from u8 to u64 to comply with :elapsed: and avoiding integer overflows
}