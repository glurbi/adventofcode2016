#[macro_use]
extern crate nom;

use nom::line_ending;
use nom::types::CompleteByteSlice;

use Dir::{ U, D, R, L };

const INPUT: &'static str = include_str!("day2.txt");

#[derive(Debug)]
enum Dir { U, D, R, L }

named!(match_up<CompleteByteSlice, Dir>, map!(char!('U'), |_| U));
named!(match_down<CompleteByteSlice, Dir>, map!(char!('D'), |_| D));
named!(match_left<CompleteByteSlice, Dir>, map!(char!('L'), |_| L));
named!(match_right<CompleteByteSlice, Dir>, map!(char!('R'), |_| R));

named!(match_one_dir<CompleteByteSlice, Dir>,
    alt!(
        match_up |
        match_down |
        match_left |
        match_right
    )
);

named!(match_dirs<CompleteByteSlice, Vec<Dir>>,
    many0!(match_one_dir)
);

named!(parse_line<CompleteByteSlice, Vec<Dir>>,
    terminated!(
        match_dirs,
        opt!(line_ending)
    )
);

named!(parse_input<CompleteByteSlice, Vec<Vec<Dir>>>,
    many0!(parse_line)
);

fn main() {
    let vv = parse_input(CompleteByteSlice(INPUT.as_bytes())).ok().unwrap().1;
    let mut res = String::from("");
    let mut digit = 5;

    let map_up = [ 0, 1, 2, 3, 1, 2, 3, 4, 5, 6 ];
    let map_down = [ 0, 4, 5, 6, 7, 8, 9, 7, 8, 9 ];
    let map_left = [ 0, 1, 1, 2, 4, 4, 5, 7, 7, 8 ];
    let map_right = [ 0, 2, 3, 3, 5, 6, 6, 8, 9, 9 ];

    for v in vv.iter() {
        for dir in v.iter() {
            digit = match dir {
                U => map_up[digit],
                D => map_down[digit],
                L => map_left[digit],
                R => map_right[digit],
            }
        }
        res.push_str(&digit.to_string());
    }

    println!("{}", res);
}
