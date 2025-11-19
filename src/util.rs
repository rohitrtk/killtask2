pub fn prettify_strings(strings: Vec<String>) -> String {
    if strings.len() == 0 {
        return "".to_string();
    } else if strings.len() == 1 {
        return format!("{}", strings[0]);
    } else if strings.len() == 2 {
        return format!("{} and {}", strings[0], strings[1]);
    } else {
        let before_last = &strings[0..strings.len() - 1].join(", ");
        let last = strings.last().unwrap();
        return format!("{}, and {}", before_last, last);
    }
}