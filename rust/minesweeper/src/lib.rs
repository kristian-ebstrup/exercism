pub fn annotate(minefield: &[&str]) -> Vec<String> {
    // u8 -> char
    // 32u8 -> " "
    // 42u8 -> "*"
    // 48u8 -> "0"
    // 49u8 -> "1"
    // 50u8 -> "2"
    // 51u8 -> "3"
    // 52u8 -> "4"

    // initiate the output field as empty strings
    let mut output_field: Vec<String> = vec![String::from(""); minefield.len()];

    // nested loops through the rows and cols
    for (i, row) in minefield.into_iter().enumerate() {
        for (j, col) in row.as_bytes().into_iter().enumerate() {
            // if mine, simply add a "*" string to output_field's relevant row
            if col == &42u8 { output_field[i].push_str("*") };

            // if empty, check all adjacent spots to find its value
            if col == &32u8 {
                let mut temp_val: u8 = 48;

                // check right
                temp_val = match row.as_bytes().get(j+1) {
                    Some(x) if x == &42u8 => temp_val + 1,
                    _ => temp_val,
                };

                // check left
                // ensure j > 0 to avoid subtraction overflow
                if j > 0 {
                    temp_val = match row.as_bytes().get(j-1) {
                        Some(x) if x == &42u8 => temp_val + 1,
                        _ => temp_val,
                    };
                }

                // check three above
                match minefield.into_iter().nth(i+1) {
                    Some(_row) => {
                        // directly above
                        temp_val = match _row.as_bytes().get(j) {
                            Some(x) if x == &42u8 => temp_val + 1,
                            _ => temp_val,
                        };

                        // diagonal up-and-left
                        // ensure j > 0 to avoid subtraction overflow
                        if j > 0 {
                            temp_val = match _row.as_bytes().get(j-1) {
                                Some(x) if x == &42u8 => temp_val + 1,
                                _ => temp_val,
                            };
                        }

                        // diagonal up-and-right
                        temp_val = match _row.as_bytes().get(j+1) {
                            Some(x) if x == &42u8 => temp_val + 1,
                            _ => temp_val,
                        };
                    },
                    None => (),
                }

                // check three below
                // ensure i > 0 to avoid subtraction overflow
                if i > 0 {
                    match minefield.into_iter().nth(i-1) {
                        Some(_row) => {
                            // directly above
                            temp_val = match _row.as_bytes().get(j) {
                                Some(x) if x == &42u8 => temp_val + 1,
                                _ => temp_val,
                            };

                            // diagonal up-and-left
                            // ensure j > 0 to avoid subtraction overflow 
                            if j > 0 {
                                temp_val = match _row.as_bytes().get(j-1) {
                                    Some(x) if x == &42u8 => temp_val + 1,
                                    _ => temp_val,
                                };
                            }

                            // diagonal up-and-right
                            temp_val = match _row.as_bytes().get(j+1) {
                                Some(x) if x == &42u8 => temp_val + 1,
                                _ => temp_val,
                            };
                        },
                        None => (),
                    }
                }

                // add value to output_field
                if temp_val > 48 {
                    match String::from_utf8(vec![temp_val]) {
                        Ok(s) => output_field[i].push_str(s.as_str()),
                        _ => unimplemented!(),
                    };
                }
                else {
                    output_field[i].push_str(" ");
                }
            };
        }
    }


    return output_field;
}
