// You should change this.
//
// Depending on your implementation, there are a variety of potential errors
// which might occur. They aren't checked by the test suite in order to
// allow the greatest freedom of implementation, but real libraries should
// provide useful, descriptive errors so that downstream code can react
// appropriately.
//
// One common idiom is to define an Error enum which wraps all potential
// errors. Another common idiom is to use a helper type such as failure::Error
// which does more or less the same thing but automatically.
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}

#[derive(Debug)]
enum ErrorKind {
    Tonic,
    Interval,
}

pub struct Scale {
    scale: Vec<String>,
}



impl<'b> Scale {
    pub fn new(tonic: &'b str, intervals: &'b str) -> Result<Self, Error> {
        // constructs a new scale given a tonic and interval
        
        // generate chromatic based on tonic, and an iterator of that scale
        let chromatic = Scale::chromatic(tonic);

        // continue based on whether the chromatic was generated or not
        match chromatic {
            Ok(_chromatic) => {
                // create a cycling iterator of the chromatic scale
                let mut iter_chromatic = _chromatic.scale.iter();

                // create empty scale vector, to be filled based on the intervals
                let mut scale: Vec<String> = Vec::new();

                // push and advance iter_chromatic to introduce the tonic to scale
                scale.push(iter_chromatic.next().unwrap().clone());

                // iterate through the intervals
                for c in intervals.chars() {
                    match c {
                        // minor second
                        'm' => {
                            // advance once, and save the note
                            scale.push(iter_chromatic.next().unwrap().clone());
                            },
                        // major second
                        'M' => {
                            // advance twice, and save the note
                            iter_chromatic.next();
                            scale.push(iter_chromatic.next().unwrap().clone()); 
                            },
                        // augmented second
                        'A' => {
                            // advance thrice (a whole step and a half step)
                            iter_chromatic.next();
                            iter_chromatic.next();
                            scale.push(iter_chromatic.next().unwrap().clone());
                            },
                        // if intervals do not match, return error 
                        _ => return Err( Error { kind: ErrorKind::Interval } )
                    }

                }
                
                return Ok( Scale{ scale } ); 
            }
            Err(error) => return Err(error),
        }
    }

    pub fn chromatic(tonic: &'b str) -> Result<Self, Error> {
        // constructs a new chromatic scale with tonic
        // It basically does it as follows:
        //      Step 1 -> Identify flat or sharp chromatic scale
        //      Step 2 -> Generate vector of strings based on the scale type (flat, sharp)
        //      Step 3 -> Find the index of the tonic in the chromatic scale, and rotate
        //                the chromatic scale such that the tonic is the first tone
        //      Step 4 -> Append the tonic to the scale
        //      Step 5 -> Return
        
        // identify scale type, where 0 = flat, 1 = sharp
        let scale_type: u8 = match tonic.len() {
            // if the tonic only as a single char, check against list
            1 => {
                if ["F", "d", "g", "c", "f"].contains(&tonic) {
                    0
                }
                else {
                    1
                }
            }
            // if the tonic has a second char, check if flat or sharp
            // and ensure that the first char is uppercase
            2 => {
                if tonic.contains("b") { 
                    0 
                }
                else {
                    1
                }
            }
            _ => return Err( Error { kind: ErrorKind::Tonic } ),
        };

        // create the base chromatic scale based on the found scale type
        let mut chromatic: Vec<String> = match scale_type {
            // if the tonic indicates a flat scale
            0 => {
                vec![
                    String::from("C"),
                    String::from("Db"), 
                    String::from("D"),
                    String::from("Eb"),
                    String::from("E"),
                    String::from("F"),
                    String::from("Gb"),
                    String::from("G"),
                    String::from("Ab"),
                    String::from("A"),
                    String::from("Bb"),
                    String::from("B"),
                ]
            }
            1 => {
                vec![
                    String::from("C"),
                    String::from("C#"), 
                    String::from("D"),
                    String::from("D#"),
                    String::from("E"),
                    String::from("F"),
                    String::from("F#"),
                    String::from("G"),
                    String::from("G#"),
                    String::from("A"),
                    String::from("A#"),
                    String::from("B"),
                ]
            }
            _ => return Err( Error { kind: ErrorKind::Tonic } ),
        };

        // find the index of the tonic. This is made complicated by having to correctly capitalize
        // the tonic, which is convoluted in Rust when the string has more than one char.
        //
        // first reformat tonic to correct capitalized format, stored in _tonic
        let _tonic: String = match tonic.len() {
            // if len == 1, simply use to_uppercase on tonic
            1 => tonic.to_uppercase(),
            
            // if len == 2, extract the capitalized first char, and add the sharp or flat based on
            // the scale type
            2 => {
                match scale_type {
                    0 => {
                        // extract first char as string
                        let mut tonic_string: String = tonic.to_string().to_uppercase();
                        tonic_string.pop();

                        // add the sharp or flat back on
                        tonic_string.push_str("b");

                        tonic_string
                    }  
                    1 => {
                        // extract first char as string
                        let mut tonic_string: String = tonic.to_string().to_uppercase();
                        tonic_string.pop();

                        // add the sharp or flat back on
                        tonic_string.push_str("#");

                        tonic_string
                    },
                    _ => return Err( Error { kind: ErrorKind::Tonic } )
                }
            },
            _ => return Err( Error { kind: ErrorKind::Tonic } ),
        };

        // find the tonic index
        let tonic_index: usize = match chromatic.iter().position(|c| c == &_tonic) {
            Some(i) => i,
            None => return Err( Error { kind: ErrorKind::Tonic } ),
        };

        // rotate based on the found index
        chromatic.rotate_left(tonic_index);

        // push the tonic's octave to wrap up the chromatic.
        chromatic.push(String::from(&chromatic[0]));

        // return the chromatic scale as a Scale structure
        return Ok(Scale { scale: chromatic } );
    }

    pub fn enumerate(&self) -> Vec<String> {
        // return the scale as a Vec<String>
        return self.scale.clone();
    }
}
