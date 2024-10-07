// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.


fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    let s = input.as_bytes();
    let len = input.len();

    let mut start = 0;
    for i in 0..len {
        if s[i] != b' ' {
            start = i;
            break;
        }
    };

    let mut end = len - 1;
    for i in (0..len).rev() {
        if s[i] != b' ' {
            end = i;
            break;
        }
    };

    let str = String::from(&input[start..=end]);

    str
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    let mut s = String::from(input);
    s.insert_str(s.len(), " world!");
    s
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    let mut s1 = String::from(input);
    let s2 = s1.replace("cars", "balloons");
    s2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
