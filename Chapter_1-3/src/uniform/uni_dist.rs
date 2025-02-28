pub fn uniform_distribution(loop_amount:u64, user_input:u16, extracted_array:&mut [f32], success:u16, matched:[bool;100]) -> u64 {
    let mut loop_amount = loop_amount;
    let mut success = success;
    let mut matched = matched;

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
    loop_amount
}
