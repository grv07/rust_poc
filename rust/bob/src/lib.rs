pub fn reply(message: &str) -> &str {
    let mut result = "Whatever.";
    let x: &[_] = &[' ', '\t', '\n', '\r'];
    let msg = message.trim_matches(x);

    if msg.len() == 0 {
        result = "Fine. Be that way!";
    } else if msg.chars().any(|c| c.is_alphabetic()) {
        if msg.to_uppercase() == msg {
            if msg.ends_with("?") {
                result = "Calm down, I know what I'm doing!";
            } else {
                result = "Whoa, chill out!";
            }
        } else if msg.ends_with("?") {
            result = "Sure."
        }
    } else if msg.ends_with("?") {
        result = "Sure."
    }
    result
}
