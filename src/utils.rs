use std::io::{stdout, Write};

pub fn percentage_printer(progress: usize, total: usize) {
    if progress % 10000 != 0 {
        return;
    }

    let n = progress as f32 / total as f32 * 100.0 + 2.0;
    let n = n as u8;
    print!("\r[");
    for _ in 0..n {
        print!("#");
    }

    for _ in n..101 {
        print!("-");
    }

    print!("] {:02}%", u8::min(n, 100));
    stdout().flush().unwrap();
}

pub fn find_strings(str: &str) -> Vec<String> {
    let mut cursor = 0;
    let chars = str.chars().collect::<Vec<char>>();
    let len = chars.len();
    let mut vec = Vec::new();

    while cursor < len {
        if chars[cursor] != '"' {
            cursor += 1;
            continue;
        }

        cursor += 1;
        let start = cursor;
        while cursor < len && !(chars[cursor] == '"' && chars[cursor - 1] != '\\') {
            cursor += 1;
        }

        vec.push(str.get(start..cursor).unwrap().trim().to_string().replace("\\", ""));
        cursor += 1;
    }

    vec
}

#[cfg(test)]
mod test {
    use super::find_strings;

    #[test]
    fn test_finding_strings() {
        let result = find_strings("[\"hello \\\" world\"]");
        assert_eq!(result, vec!["hello \" world"]);
    }

    #[test]
    fn test_finding_multiple() {
        let result = find_strings("[\"Hello \", \" John\"]");
        let expected = vec!["Hello", "John"];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_finding_long_with_ands() {
        let result = find_strings("\"This is a long one\" , \"Another one\" and \"Yet Done \\\" rightly\"");
        let expected = vec!["This is a long one", "Another one", "Yet Done \" rightly"];
        assert_eq!(result, expected);
    }
}
