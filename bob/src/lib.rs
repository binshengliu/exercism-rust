// Bob is a lackadaisical teenager. In conversation, his responses are very limited.
// Bob answers 'Sure.' if you ask him a question.
// He answers 'Whoa, chill out!' if you yell at him.
// He says 'Fine. Be that way!' if you address him without actually saying
// anything.
// He answers 'Whatever.' to anything else.

fn is_yelling(text: &str) -> bool {
    text == text.to_uppercase() && text != text.to_lowercase()
}

fn is_empty(text: &str) -> bool {
    text.trim().is_empty()
}

fn is_question(text: &str) -> bool {
    text.ends_with("?")
}

pub fn reply(text: &str) -> String {
    if is_empty(text) {
        return "Fine. Be that way!".to_string();
    }

    if is_yelling(text) {
        return "Whoa, chill out!".to_string();
    }

    if is_question(text.trim()) {
        return "Sure.".to_string();
    }

    return "Whatever.".to_string();
}
