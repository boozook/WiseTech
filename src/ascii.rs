use crate::Line;
use std::borrow::Cow;

/// Same as [`crate::transform`] but little bit optimized.
pub fn transform(input: &str, line_width: u32) -> Cow<'_, str> {
    if input.trim().is_empty() || line_width == 0 {
        return Cow::Borrowed(input);
    }

    debug_assert!(input.is_ascii());

    // Preallocated vector of words (str-slices)
    let mut words = Vec::with_capacity(input.len() / line_width as usize);

    // Current line accumulator:
    let mut current_line = Line::default();

    // Result:
    let mut output = String::new();

    crate::align_words(
        input.split_ascii_whitespace(),
        &mut words,
        &mut current_line,
        line_width as _,
        &mut output,
    );

    // In case of just one line or last line, we haven't met line-break after that, so didn't render that line.
    // Render current (last) line:
    current_line.render(&words, &mut output, line_width as _);

    output.into()
}

#[cfg(test)]
mod tests {
    use super::transform;

    #[test]
    fn simple() {
        let test_cases = [
            ("", 5, ""),
            ("test", 5, "test "),
            ("Lorem     ipsum    dolor", 17, "Lorem ipsum dolor"),
            ("Lorem     ipsum    dolor", 18, "Lorem  ipsum dolor"),
            ("Lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua", 12,
             "Lorem  ipsum\ndolor    sit\namet        \nconsectetur \nadipiscing  \nelit  sed do\neiusmod     \ntempor      \nincididunt  \nut labore et\ndolore magna\naliqua      "),
            ("Lorem     ipsum    dolor", 17, "Lorem ipsum dolor"),
            ("Lorem     ipsum    dolor", 18, "Lorem  ipsum dolor"),
        ];

        for &(input, line_width, expected) in &test_cases {
            println!("input: '{}'", input);
            assert_eq!(transform(input, line_width), expected);
        }
    }
}
