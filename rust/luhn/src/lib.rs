/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    // split at the whitespaces and combine
    let stripped_code: String = code.split_whitespace().collect();

    // check if string length is 2 or larger 
    if stripped_code.len() < 2 { return false }

    // run through the stripped string and push the values to luhn_values
    let mut luhn_values: Vec<i8> = Vec::with_capacity(stripped_code.len());

    for (i, bval) in stripped_code.as_bytes().into_iter().rev().enumerate() {
        // extract u8 value from the utf8 byte value
        let mut uval: u8 = match bval {
            &48u8 => 0,
            &49u8 => 1,
            &50u8 => 2,
            &51u8 => 3,
            &52u8 => 4,
            &53u8 => 5,
            &54u8 => 6,
            &55u8 => 7,
            &56u8 => 8,
            &57u8 => 9,
            _ => return false,
        };
        
        // perform operation on every second digit (starting from the right)
        if (i+1) % 2 == 0 {
            // double uval
            uval = ( uval * 2 );

            // if product is greater than 9, subtract 9
            if uval > 9 {uval = uval - 9};
        };
        
        // push value to the luhn_values vector
        luhn_values.push(uval as i8);
    }

    // check if valid luhn number, i.e. evenly divisible by 10
    let sum: i8 = luhn_values.into_iter().sum();

    return sum % 10 == 0;
}
