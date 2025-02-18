use core::f64;
use std::io;

fn main() {
    // taking input from the user 
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("unable to read user input");
    //parsing the input into a unsigned int
    let user_input = user_input.trim().parse::<u16>().unwrap();
    //Instructing only valid inputs are between 1-10_000
    if user_input < 1 { println!("user input is less than one")} else if user_input > 10_000 {println!("user input is greater that 10_000")} 
    // here the user generates userinput(N) amount of random float numbers between 0 and 1 and
    // subsequently push them into an array
    else {
        //initalizing the array
        let mut float_array:[f64;1_000] = [0.0;1_000];
        //doubles as a counter and an index for the array
        let mut array_index:u16 = 0;
        let mut loop_amount:u16 = 0;

        //the loop to fill the array
        loop {
            //initalizing random float with in range
            let generated_float:f64 = rand::random_range(0.0..=1.0);

            //pushing random float into array
            float_array[array_index as usize] = generated_float;

            array_index += 1;
            if array_index == user_input || array_index == 1000 {break}
        }
        println!("this is the array {:?}",float_array);
        //loop to see how much it takes to regenerate the same numbers, in the new array
        loop {
            //initalizing random float with in range
            let generated_float:f64 = rand::random_range(0.0..=1.0);
            //initalizing the counter
            let mut counter = 0;

            loop_amount += 1;
            counter +=1;
            if counter == user_input {break}
        }

    }
    


}
