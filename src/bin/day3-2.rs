#[macro_use]
extern crate nom;

use nom::{line_ending,space,digit};
use nom::types::CompleteStr;

const INPUT: &'static str = include_str!("day3.txt");

named!(integer<CompleteStr, u64>,
    map!(digit, |s| s.parse::<u64>().unwrap())
);

named!(three_triangles<CompleteStr, Vec<(u64, u64, u64)>>,
    do_parse!(
        space >>
        t11: integer >>
        space >>
        t21: integer >>
        space >>
        t31: integer >>
        line_ending >>
        space >>
        t12: integer >>
        space >>
        t22: integer >>
        space >>
        t32: integer >>
        line_ending >>
        space >>
        t13: integer >>
        space >>
        t23: integer >>
        space >>
        t33: integer >>
        line_ending >>
        (vec![(t11, t12, t13), (t21, t22, t23), (t31, t32, t33)])
    )
);


named!(all_triangles<CompleteStr, Vec<(u64, u64, u64)>>,
    map!(
        many0!(three_triangles),
        |vv| vv.iter().flat_map(|v| v.iter()).cloned().collect()
    )
);

fn main() {
    let v = all_triangles(CompleteStr(INPUT)).ok().unwrap().1;
    //println!("{:?}", v);
    let count = v.iter().filter(|t| t.1 + t.2 > t.0 && t.0 + t.1 > t.2 && t.2 + t.0 > t.1).count();
    println!("{}", count);
}
