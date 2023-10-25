#![feature(test)]
extern crate line_adjustment;
extern crate test;

use test::Bencher;

const CASES:&[(&str, u32)] = &[
            ("", 5),
            ("test", 5),
            ("Lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua", 12,
             ),
            ("Lorem     ipsum    dolor", 17),
				("Lorem  ipsum       dolor", 18),
        ];

// 1,056,950 ns/iter (+/- 7,296) on single thread on M1
#[bench]
fn transform_utf(b: &mut Bencher) {
    use line_adjustment::transform;

    let cases = CASES;

    b.iter(|| {
        for _ in 0..1000 {
            for &(input, line_width) in cases {
                let _ = transform(input, line_width);
            }
        }
    });
}

// 975,087 ns/iter (+/- 5,091) on single thread on M1
#[bench]
fn transform_ascii(b: &mut Bencher) {
    use line_adjustment::ascii::transform;

    let cases = CASES;

    b.iter(|| {
        for _ in 0..1000 {
            for &(input, line_width) in cases {
                let _ = transform(input, line_width);
            }
        }
    });
}
