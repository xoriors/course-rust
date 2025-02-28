use std::{io,time::Instant};
use rand::random_range;

mod uniform;
use uniform::uni_dist;

mod weighted;
use weighted::wei_dist;

fn main() {
    // taking input from the user 
    println!("Please enter the amount of random generated floats you want in your array \n-------------------------------------");
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("unable to read user input");
    //parsing the input into a unsigned int
    let user_input = user_input.trim().parse::<u16>().unwrap();
    //Instructing only valid inputs are between 1-10_000
    if user_input < 1 { println!("user input is less than one")} else if user_input > 1_00 {println!("user input is greater that 100")} 
    // here the user generates userinput(N) amount of random float numbers between 0 and 1 and
    // subsequently push them into an array
    else {
        const _BILLION:u64 = 1000000000;

        //initalizing the arrays
        let mut float_array:[f32;1_00] = [0.0;1_00];
        let mut int_array:[u64;100] = [0;100];

        //doubles as a counter and an index for the array
        let mut array_index:u16 = 0;


        //the loop to fill the array
        loop {
            //initalizing random float with in range
            let generated_float:f32 = random_range(0.0..=1.0);

            //initalizing random float with in range
            let generated_int:u64 = random_range(0..=1000);

            //pushing random float into array
            float_array[array_index as usize] = generated_float;

            //pushing random float into array
            int_array[array_index as usize] = generated_int;

            array_index += 1;
            if array_index == user_input || array_index == 100 {break}
        }
        //we are extracting the an array of the random generated numbers only since rust doesnt
        //allow dynamically sized arrays
        let extracted_float = &mut float_array[0..user_input as usize];
        let extracted_int = &mut int_array[0..user_input as usize];

        println!("-------------------------------------\nThis is the extracted array with the random generated numbers:\n{:#?}", extracted_float);
        println!("-------------------------------------\nThis is the extracted array with the random generated numbers:\n{:#?}", extracted_int);
        let mut uniform_total_loop = 0;
        let mut weighted_total_loop = 0;

        let mut uniform_loop_array:[u64;7] = [0;7];
        let mut uniform_time_array:[f32;7] = [0.0;7];

        let mut weighted_loop_array:[u64;7] = [0;7];
        let mut weighted_time_array:[f32;7] = [0.0;7];
        //looping hundred times
        //initalizing the loop counter
        '_uniform_distribution:loop {        
            let matched = [false; 100]; // Fixed-size array to track matched elements
            let success = 0;
            let loop_amount = 0;
            println!("---------------------------------- Loop({})-UNIFORM_DISTRIBUTION",uniform_total_loop + 1);

            // Loop to see how much it takes to regenerate the same numbers
            let now = Instant::now();
            let loop_amount = uni_dist::uniform_distribution(loop_amount, user_input, extracted_float, success, matched);
            let time_taken = now.elapsed().as_secs_f32();
            uniform_loop_array[uniform_total_loop] = loop_amount;
            uniform_time_array[uniform_total_loop] = time_taken;
            uniform_total_loop += 1;
            if uniform_total_loop == 7 {break}
        }

        '_weighted_distribution:loop {        
            let matched = [false; 100]; // Fixed-size array to track matched elements
            let loop_amount = 0;
            println!("---------------------------------- Loop({})-WEIGHTED_DISTRIBUTION",weighted_total_loop + 1);

            // Loop to see how much it takes to regenerate the same numbers
            let now = Instant::now();
            let loop_amount = wei_dist::weighted_distribution(loop_amount,extracted_int, matched);
            let time_taken = now.elapsed().as_secs_f32();
            weighted_loop_array[weighted_total_loop] = loop_amount;
            weighted_time_array[weighted_total_loop] = time_taken;
            weighted_total_loop += 1;
            if weighted_total_loop == 7 {break}
        }

        uniform_loop_array.sort();
        uniform_time_array.sort_by(|a, b| a.partial_cmp(b).unwrap());

        weighted_loop_array.sort();
        weighted_time_array.sort_by(|a, b| a.partial_cmp(b).unwrap());
        //for debugging purposes
        //println!("Array {:#?}",loop_array);
        //println!("Array {:#?}",time_array);
        println!("\n------------FOR THE UNIFORM DISTRIBUTION------------");
        println!("This is the Minimum loop {} and time {} needed",uniform_loop_array[0] ,uniform_time_array[0]);
        println!("This is the Average loop {} and time {} needed",uniform_loop_array[3] ,uniform_time_array[3]);
        println!("This is the Maxmimum loop {} and time {} needed",uniform_loop_array[6] ,uniform_time_array[6]);
        println!("\n------------FOR THE WEIGHTED DISTRIBUTION------------");
        println!("This is the Minimum loop {} and time {} needed",weighted_loop_array[0] ,weighted_time_array[0]);
        println!("This is the Average loop {} and time {} needed",weighted_loop_array[3] ,weighted_time_array[3]);
        println!("This is the Maxmimum loop {} and time {} needed",weighted_loop_array[6] ,weighted_time_array[6]);

    }
}
