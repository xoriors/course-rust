use std::io;


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
        //initalizing the array
        let mut float_array:[f32;1_00] = [0.0;1_00];
        //doubles as a counter and an index for the array
        let mut array_index:u16 = 0;


        //the loop to fill the array
        loop {
            //initalizing random float with in range
            let generated_float:f32 = rand::random_range(0.0..=1.0);

            //pushing random float into array
            float_array[array_index as usize] = generated_float;

            array_index += 1;
            if array_index == user_input || array_index == 100 {break}
        }
        //we are extracting the an array of the random generated numbers only since rust doesnt
        //allow dynamically sized arrays
        let extracted_array = &mut float_array[0..user_input as usize];
        println!("-------------------------------------\nThis is the extracted array with the random generated numbers:\n{:#?}", extracted_array);
        let mut total_loop = 0;
        let mut loop_array:[u64;100] = [0;100];
        //looping hundred times
        //initalizing the loop counter
        loop {        
            let mut matched = [false; 100]; // Fixed-size array to track matched elements
            let mut success = 0;
            let mut loop_amount = 0;
            println!("---------------------------------- Loop({})",total_loop + 1);

            // Loop to see how much it takes to regenerate the same numbers
            loop {
                // Generate a random float
                let generated_float: f32 = rand::random_range(0.0..=1.0);

                // Iterate over the array to find a match
                for i in 0..extracted_array.len() {
                    if !matched[i] && extracted_array[i] == generated_float {
                        println!("Match for index {} found",success);
                        success += 1;
                        // checking for matched random numbers so it doesnt match it again
                        matched[i] = true;
                        break; 
                    }
                }

                loop_amount += 1;
                if success == user_input {break}
            }
            loop_array[total_loop] = loop_amount;
            total_loop += 1;
            if total_loop == 100 {break}
        }
        loop_array.sort();
        //for debugging purposes
        //println!("Array {:#?}",loop_array);
        println!("This is the Minimum loop needed {}",loop_array[0]);
        println!("This is the Average loop needed {}",loop_array[49]);
        println!("This is the Maximum loop needed {}",loop_array[99]);

    }
}
