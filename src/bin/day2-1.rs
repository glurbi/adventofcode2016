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
    terminated!(many0!(parse_line), eof!())
);

fn main() {
    let vv = parse_input(CompleteByteSlice(INPUT.as_bytes()));
}
