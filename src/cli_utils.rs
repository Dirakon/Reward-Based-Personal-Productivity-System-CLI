pub fn get_line(prompt: Option<&str>) -> String {
    match prompt {
        Some(prompt_str) => println!("{}", prompt_str),
        _ => (),
    };
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Console reading failed");
    return input.strip_suffix('\n').unwrap().to_string();
}

pub fn get_parsed_line<T: std::str::FromStr>(prompt: Option<&str>) -> T {
    return match get_line(prompt).parse::<T>() {
        Ok(val) => val,
        Err(er) => get_parsed_line(prompt),
    };
}

pub fn get_line_with_condition<F>(prompt: Option<&str>, condition: F) -> String
where
    F: Fn(&String) -> bool,
{
    let line = get_line(prompt);
    return match condition(&line) {
        true => line,
        false => get_line_with_condition(prompt, condition),
    };
}

pub fn get_parsed_line_with_condition<T: std::str::FromStr, F>(
    prompt: Option<&str>,
    condition: F,
) -> T
where
    F: Fn(&T) -> bool,
{
    return match get_line(prompt).parse::<T>() {
        Ok(val) => match condition(&val) {
            true => val,
            false => get_parsed_line_with_condition(prompt, condition),
        },
        Err(er) => get_parsed_line_with_condition(prompt, condition),
    };
}
