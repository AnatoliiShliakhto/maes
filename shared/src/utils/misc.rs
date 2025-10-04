pub fn extract_first_chars(input: &str) -> String {
    let mut words = input.split_whitespace();
    match (words.next(), words.next()) {
        (Some(first), Some(second)) => {
            let first_char = first.chars().next().unwrap_or('?');
            let second_char = second.chars().next().unwrap_or(' ');
            let mut result = String::with_capacity(8);
            result.push(first_char);
            result.push(second_char);
            result
        }
        (Some(single), None) => {
            let mut chars = single.chars();
            match (chars.next(), chars.next()) {
                (Some(first), Some(second)) => {
                    let mut result = String::with_capacity(8);
                    result.push(first);
                    result.push(second);
                    result
                }
                (Some(first), None) => first.to_string(),
                _ => "?".to_string(),
            }
        }
        _ => "?".to_string(),
    }
}

pub fn extract_form_checkboxes(payload: &[String]) -> Vec<bool> {
    let mut result = Vec::<bool>::new();
    let mut i = 0;
    while i < payload.len() {
        if payload[i] == "true" {
            result.push(true);
            i += 2;
        } else {
            result.push(false);
            i += 1;
        }
    }
    result
}
