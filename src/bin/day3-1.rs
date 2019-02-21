#[macro_use]
extern crate nom;

use nom::{line_ending,space,digit};
use nom::types::CompleteStr;

const INPUT: &'static str = include_str!("day3.txt");

named!(integer<CompleteStr, u64>,
    map!(digit, |s| s.parse::<u64>().unwrap())
);

named!(triangle<CompleteStr, (u64, u64, u64)>,
    do_parse!(
        space >>
        i1: integer >>
        space >>
        i2: integer >>
        space >>
        i3: integer >>
        (i1, i2, i3)
    )
);

named!(triangles<CompleteStr, Vec<(u64, u64, u64)>>,
    many0!(
        do_parse!(
            t: triangle >>
            opt!(line_ending) >>
            (t)
        )
    )
);

fn main() {
    let v = triangles(CompleteStr(INPUT)).ok().unwrap().1;
    let count = v.iter().filter(|t| t.1 + t.2 > t.0 && t.0 + t.1 > t.2 && t.2 + t.0 > t.1).count();
    println!("{}", count);
}
