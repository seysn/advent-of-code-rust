trait Password {
    fn increment(&self) -> String;
    fn is_valid(&self) -> bool;
    fn next(&self) -> String;
}

impl Password for String {
    fn increment(&self) -> String {
        let mut next_string = String::new();

        let mut inc = true;
        for ch in self.chars().rev() {
            if !inc {
                next_string.push(ch);
                continue;
            }

            if ch == 'z' {
                next_string.push('a');
                continue;
            }

            let byte = match ch as u8 {
                b @ (105 | 108 | 111) => b + 2,
                b => b + 1,
            };
            next_string.push(byte as char);
            inc = false;
        }

        next_string.chars().rev().collect()
    }

    fn is_valid(&self) -> bool {
        for ch in ['i', 'o', 'l'] {
            if self.contains(ch) {
                return false;
            }
        }

        let mut has_inc_chars = false;
        for ch in self.as_bytes().windows(3) {
            let (a, b, c) = (ch[0], ch[1], ch[2]);
            if a + 1 == b && b + 1 == c {
                has_inc_chars = true;
                break;
            }
        }

        if !has_inc_chars {
            return false;
        }

        let mut pairs = 0;
        let mut skip_next = false;
        for ch in self.as_bytes().windows(2) {
            if skip_next {
                skip_next = false;
                continue;
            }

            let (a, b) = (ch[0], ch[1]);
            if a == b {
                pairs += 1;
                skip_next = true;
            }
        }

        pairs >= 2
    }

    fn next(&self) -> String {
        let mut s = String::new();
        let mut found_letter = false;
        for &byte in self.as_bytes() {
            if found_letter {
                s.push('a');
            } else if byte == 105 || byte == 108 || byte == 111 {
                found_letter = true;
                s.push((byte + 1) as char);
            } else {
                s.push(byte as char);
            }
        }

        let mut s = self.increment();
        while !s.is_valid() {
            s = s.increment();
        }

        s
    }
}

pub fn parse_input(input: &str) -> String {
    String::from(input)
}

pub fn part1(input: &str) -> String {
    input.to_string().next()
}

pub fn part2(input: &str) -> String {
    input.to_string().next().next()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increment() {
        assert_eq!("xx".to_string().increment(), "xy".to_string());
        assert_eq!("xy".to_string().increment(), "xz".to_string());
        assert_eq!("xz".to_string().increment(), "ya".to_string());
        assert_eq!("zyz".to_string().increment(), "zza".to_string());
        assert_eq!("azzz".to_string().increment(), "baaa".to_string());
    }

    #[test]
    fn test_valid() {
        assert!("aabbxyz".to_string().is_valid());
        assert!("abcdffaa".to_string().is_valid());
        assert!("ghjaabcc".to_string().is_valid());
        assert!(!"hijklmmn".to_string().is_valid());
        assert!(!"abbceffg".to_string().is_valid());
        assert!(!"abbcegjk".to_string().is_valid());
        assert!(!"aabbijk".to_string().is_valid());
    }

    #[test]
    fn test_next() {
        assert_eq!("abcdefgh".to_string().next(), "abcdffaa".to_string());
        assert_eq!("ghijklmn".to_string().next(), "ghjaabcc".to_string());
    }
}