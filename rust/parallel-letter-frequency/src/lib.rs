use std::collections::HashMap;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    // initiate empty HashMap
    let mut frequency_map = HashMap::new();

    // collect input as a vector of characters
    let collective_input: String = input.join("").to_lowercase();
    
    // get the input length
    let input_len = collective_input.len();

    // if input length is less than the worker count, just input_len workers
    let worker_count = match input_len < worker_count { 
        true => input_len,
        false => worker_count,
    };

    println!("worker_count: {:?}", worker_count);
    println!("input_len: {:?}", input_len);

    // spawn threads and give each a chunk of the input to go through
    let mut handles = Vec::with_capacity(worker_count);
    for n in 0..worker_count {
        // clone collective_input
        let local_input = collective_input.clone();
        
        // compute index range
        let start = n * input_len / worker_count;
        let end = start + (input_len / worker_count);

        // create a child worker to sift through the chunk
        handles.push( thread::spawn( move || {
            // initiate empty local HashMap
            let mut local_map = HashMap::new();

            // go through the local input and build the map
            // make sure that the last worker gets the rest of the input
            if n == worker_count - 1 {
                for c in local_input.chars().skip(start) {
                    if c.is_alphabetic() {
                        *local_map.entry(c).or_insert(0) += 1;
                    }
                }
            }
            else {
                for c in local_input.chars().skip(start).take(end - start) {
                    if c.is_alphabetic() {
                        *local_map.entry(c).or_insert(0) += 1;
                    }
                }
            }

            // return the local_map to the handles
            return local_map;
        }));
    }

    // compile the local maps into frequency map
    for handle in handles {
        let local_map = handle.join().unwrap();
        for (c, count) in local_map {
            *frequency_map.entry(c).or_insert(0) += count;
        }
    }

    println!("{:?}", frequency_map);

    return frequency_map;
}
