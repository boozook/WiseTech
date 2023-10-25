pub mod ascii;

/// Line semi-span representation.
#[derive(Debug, Default)]
struct Line {
    /// Range: `(start, len)`
    words: (usize, usize),
    /// Length in chars
    length: usize,
}

impl Line {
    /// Writes words from `src` to `output` aligned and with padding, with length limited to `line_width`.
    fn render(self, src: &[&str], output: &mut String, line_width: usize) {
        let words = &src[self.words.0..(self.words.0 + self.words.1)];
        let last = words.len() - 1;

        // Minimal width of line as sum of `self.length` and `{num of words} - 1`
        // (means num of separators/spaces between words).
        let width = self.length + words.len().saturating_sub(1);

        let (padding, mut extra) = crate::padding(words.len(), width, line_width);
        words.into_iter().enumerate().for_each(|(i, word)| {
            output.push_str(word);
            // Add padding for single or non-last only:
            if i != last || last == 0 {
                for _ in 0..padding {
                    output.push(' ');
                }
            }
            if extra != 0 {
                output.push(' ');
                extra -= 1;
            }
        });
    }
}

/// Dummy implementation over UTF-8 strings without proper grapheme clusters handling, but it tries to do it.
///
/// `line_width` is length in __bytes__.
///
/// Note, there's no validation using `is_char_boundary` or something like that.
pub fn transform(input: &str, line_width: u32) -> String {
    if input.trim().is_empty() || line_width == 0 {
        return input.to_string();
    }

    // Preallocated vector of words (str-slices)
    let mut words = Vec::with_capacity(input.len() / line_width as usize);

    // Current line accumulator:
    let mut current_line = Line::default();

    // Result:
    let mut output = String::new();

    crate::align_words(
        input.split_whitespace(),
        &mut words,
        &mut current_line,
        line_width as _,
        &mut output,
    );

    // In case of just one line or last line, we haven't met line-break after that, so didn't render that line.
    // Render current (last) line:
    current_line.render(&words, &mut output, line_width as _);

    output
}

fn align_words<'s, I: Iterator<Item = &'s str>>(
    source: I,
    words: &mut Vec<&'s str>,
    line: &mut Line,
    line_width: usize,
    output: &mut String,
) {
    source.for_each(|word| {
        let length = word.len() + line.length;
        let width = length + line.words.1;

        words.push(word);

        // Line break and recalculate min line len:
        if width > line_width as usize {
            // Start new line
            let next = Line {
                words: (words.len() - 1, 1),
                length: word.len(),
                //  width: word.len(),
            };
            let prev = std::mem::replace(line, next);

            // Render current line:
            prev.render(&words, output, line_width as _);
            output.push('\n');
        } else {
            line.words.1 += 1;
            line.length = length;
            // line.width = width;
        }
    });
}

/// Calculates padding needed to align string line according to limited width.
/// - `words` is num of words in the line.
/// - `separated` is sum of lengths of words separated by one char
///   (space character between words).
/// - `max` is maximal width of the line.
///
/// Returns `(padding, extra padding)` tuple, where
/// - `padding` is number of spaces to add between words __for each pair__,
/// - `extra padding` additional spaces to add as described in the task, from left to right.
fn padding(words: usize, separated: usize, max: usize) -> (usize, usize) {
    let diff = max - separated;

    // Zero-cases:
    if words == 0 {
        return (0, 0);
    } else if diff == 0 {
        return ((words != 1) as _, 0);
    }
    // Special corner cases:
    else if diff < words {
        return (1, diff);
    } else if words == 1 {
        return (diff, 0);
    } else if words == 2 {
        return (diff + 1, 0);
    }

    let mut padding = words.saturating_sub(1) / diff;
    let mut extra = words % diff;

    if words.saturating_sub(1) == 1 {
        extra += padding;
        padding = 0;
    }

    debug_assert_eq!(
        (separated - words.saturating_sub(1)) + (words.saturating_sub(1) * padding) + extra,
        max
    );

    (padding, extra)
}

#[cfg(test)]
mod tests {
    use super::transform;

    #[test]
    fn simple() {
        let test_cases = [
            ("", 5, ""),
            ("test", 5, "test "),
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

    #[test]
    fn padding() {
        use super::padding;

        let cases = [
            (0, 0, 5, (0, 0)),
            (1, 4, 6, (2, 0)),
            (1, 5, 5, (0, 0)),
            (3, 17, 17, (1, 0)),
            (3, 17, 18, (1, 1)),
            (2, 9, 12, (4, 0)),
        ];

        for (words, width, line_width, expected) in cases.into_iter() {
            println!("test: w={words},\tl={width},\tm={line_width},\texpected: {expected:?}");
            assert_eq!(padding(words, width, line_width), expected);
        }
    }
}
