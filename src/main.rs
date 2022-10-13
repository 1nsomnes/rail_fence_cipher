fn main() { }

pub fn encode_rail_fence_cipher(text: &str, num_rails: usize) -> String {
    let mut counter = Counter::new(num_rails as i64);

    let mut chars_by_row:Vec<(char, i64)> = text.chars().into_iter().zip(counter.into_iter()).collect();
    chars_by_row.sort_by_key(|c| c.1);

    let mut result = String::new();

    for c in chars_by_row {
        println!("Char: {}, Row: {}", c.0, c.1);
        result.push_str(&c.0.to_string());
    }

    result
}

struct Counter {
    pub num: i64,
    pub size: i64,
    pub ascending: bool,
}

impl Counter {
    pub fn new(size:i64) -> Counter {
        Counter {
            num: 0,
            size,
            ascending: true,
        }
    }
}

impl Iterator for Counter {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.num;

        if self.ascending { self.num += 1; }
        else { self.num -= 1 }
        if self.num == self.size - 1 { self.ascending = false; }
        else if self.num == 0 { self.ascending = true; }

        return Some(result);
    }
}

fn decode_rail_fence_cipher(text: &str, num_rails: usize) -> String {
    let mut sizes:Vec<i64> = vec![0; num_rails];
    let mut counter = Counter::new(num_rails as i64);
    let mut text_chars:Vec<char> = Vec::new();

    for c in text.chars() {
        sizes[counter.num as usize] += 1;
        text_chars.push(c);
        counter.next();
    }

    let mut groups:Vec<Vec<char>> = vec![Vec::new(); num_rails];
    let mut text_index = 0;

    for group_size_index in 0..sizes.len() {
        for _ in 0..sizes[group_size_index] {
            groups[group_size_index].push(text_chars[text_index]);
            text_index += 1;
        }
    }

    let mut result = String::new();
    counter.num = 0;
    counter.ascending = true;

    for i in 0..text.len() {
        let group = &groups[counter.num as usize];
        result.push(group[0]);
        groups[counter.num as usize].remove(0);
        counter.next();
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::{Counter, decode_rail_fence_cipher, encode_rail_fence_cipher};

    #[test]
    fn test_tests() {
        assert_eq!(2,2);
    }

    #[test]
    fn counter_test() {
        let left:Vec<i64> = vec!(0,1,2,1,0,1,2);
        let mut right = Counter::new(3);

        for i in 0..7 {
            assert_eq!(left[i], right.next().unwrap());
        }
    }

    #[test]
    fn encode_test_1() {
        assert_eq!(encode_rail_fence_cipher("WEAREDISCOVEREDFLEEATONCE", 3), "WECRLTEERDSOEEFEAOCAIVDEN");
    }

    #[test]
    fn encode_test_2() {
        assert_eq!(encode_rail_fence_cipher("Hello, World!", 3), "Hoo!el,Wrdl l");
    }

    #[test]
    fn decode_test_1() {
        assert_eq!(decode_rail_fence_cipher("WECRLTEERDSOEEFEAOCAIVDEN", 3), "WEAREDISCOVEREDFLEEATONCE");
    }

    #[test]
    fn decode_test_2() {
        assert_eq!(decode_rail_fence_cipher("Hoo!el,Wrdl l", 3), "Hello, World!");
    }
}
