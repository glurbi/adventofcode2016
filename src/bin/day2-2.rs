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

    let map_up = [ 0, 1, 2, 1, 4, 5, 2, 3, 4, 9, 6, 7, 8, 11 ];
    let map_down = [ 0, 3, 6, 7, 8, 5, 10, 11, 12, 9, 10, 13, 12, 13 ];
    let map_left = [ 0, 1, 2, 2, 3, 5, 5, 6, 7, 8, 10, 10, 11, 13 ];
    let map_right = [ 0, 1, 3, 4, 4, 6, 7, 8, 9, 9, 11, 12, 12, 13 ];

    for v in vv.iter() {
        for dir in v.iter() {
            digit = match dir {
                U => map_up[digit],
                D => map_down[digit],
                L => map_left[digit],
                R => map_right[digit],
            }
        }
        let c = (digit + (if digit >= 10 { 55 } else { 48  })) as u8 as char;
        res.push(c);
    }

    println!("{}", res);
}
