pub fn string_to_pascal_case(string: &str) -> String {
    let mut result = String::with_capacity(string.chars().count());
    let mut next_uppercase = true;

    for char in string.chars() {
        if char == '_' {
            next_uppercase = true;
            continue;
        }

        result.push(if next_uppercase {
            next_uppercase = false;
            char.to_uppercase().next().unwrap_or(char)
        } else {
            char
        });
    }

    result
}
