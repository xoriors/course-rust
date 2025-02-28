use rand::random_range;

pub fn weighted_distribution(loop_amount:u64, extracted_array:&mut [u64], matched:[bool;100]) -> u64 {
    //initalizing variables
    const  ONE_HUNDRED:u64 = 100;
    let mut loop_amount = loop_amount;
    let mut matched = matched;

    let mut weight_interval = 0;
    let mut weight_array:[(u64,u64,u64);10] = [(0,ONE_HUNDRED,0);10];
    for value in weight_array.iter_mut() {
        value.0 += weight_interval;
        value.1 += weight_interval;
        for i in extracted_array.iter() {
            if value.0 < *i && *i < value.1 { value.2+=1 }
        }
        weight_interval += ONE_HUNDRED;
    }
    //sorting based on they're weights
    weight_array.sort_by(|b, a| a.2.cmp(&b.2));
    println!("WEIGHT MAP \n {:?}",weight_array);

    // Loop to generate the same numbers based on they're weights
    loop {
        // Generate a random float
        let generated_num = random_range(weight_array[0].0..weight_array[0].1);

        // Iterate over the array to find a match
        for (i,value) in extracted_array.iter().enumerate() {
            if !matched[i] && *value == generated_num {
                println!("HIT, match found for interval {} - {}",weight_array[0].0,weight_array[0].1);
                //making sure the weight decreases for each hit 
                weight_array[0].2 -= 1;
                // checking for matched random numbers so it doesnt match it again
                matched[i] = true;
            }
        }

        // re-sorting the arrays so that we always generate numbers for the highest weight interval
        weight_array.sort_by(|b, a| a.2.cmp(&b.2));
        loop_amount += 1;

        if weight_array[0].2 == 0 {break}
    }
    loop_amount
}
