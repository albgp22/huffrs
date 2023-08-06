use std::collections::HashMap;

fn get_frequencies(s: &str) -> HashMap<char, u32> {
    s.chars().fold(HashMap::new(), |mut map, c| {
        *map.entry(c).or_insert(0u32) += 1;
        map
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_requencies_test_empty() {
        let expected: HashMap<char, u32> = HashMap::new();
        assert_eq!(get_frequencies(""), expected);
    }
    #[test]
    fn get_requencies_test_single_char() {
        let expected: HashMap<char, u32> = [('a', 1u32)].iter().cloned().collect();
        assert_eq!(get_frequencies("a"), expected);
    }
    #[test]
    fn get_requencies_test_multiple_chars() {
        let expected: HashMap<char, u32> = [('a', 1u32), ('b', 1u32), ('c', 1u32), ('d', 1u32)]
            .iter()
            .cloned()
            .collect();
        assert_eq!(get_frequencies("abcd"), expected);
    }
    #[test]
    fn get_requencies_test_multiple_chars_2() {
        let expected: HashMap<char, u32> = [('a', 2u32), ('b', 1u32), ('c', 1u32), ('d', 3u32)]
            .iter()
            .cloned()
            .collect();
        assert_eq!(get_frequencies("abcddda"), expected);
    }
}
