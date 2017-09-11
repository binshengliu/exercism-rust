pub fn build_proverb(input: Vec<&str>) -> String {
    if input.is_empty() {
        return String::new()
    }

    // The use of zip is inspired by
    // http://exercism.io/submissions/c3da68d562de4f85898e465da43228b9
    // let mut output: Vec<String> = Vec::new();
    let mut output: Vec<String> = input.iter().zip(input.iter().skip(1))
        .map(|pair| format!("For want of a {} the {} was lost.", pair.0, pair.1))
        .collect();

    if input.len() <= 2 {
        output.push(String::from("And all for the want of a nail."));
    } else {
        output.push(String::from("And all for the want of a horseshoe nail."));
    }
    if let 2 = input.len() {
        ()
    }

    output.join("\n")
}
