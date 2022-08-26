pub fn reverse(input: &str) -> String {
    let mut input_as_string: String = input.to_string();
    let mut reversed_string: String = String::from("");

    while !input_as_string.is_empty() {
       reversed_string.push(input_as_string.pop().unwrap());
    }

    return reversed_string;
}
