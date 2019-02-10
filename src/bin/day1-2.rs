use std::collections::HashSet;
use std::process;

fn main() {
    
    let input = "R5, L2, L1, R1, R3, R3, L3, R3, R4, L2, R4, L4, R4, R3, L2, L1, L1, R2, R4, R4, L4, R3, L2, R1, L4, R1, R3, L5, L4, L5, R3, L3, L1, L1, R4, R2, R2, L1, L4, R191, R5, L2, R46, R3, L1, R74, L2, R2, R187, R3, R4, R1, L4, L4, L2, R4, L5, R4, R3, L2, L1, R3, R3, R3, R1, R1, L4, R4, R1, R5, R2, R1, R3, L4, L2, L2, R1, L3, R1, R3, L5, L3, R5, R3, R4, L1, R3, R2, R1, R2, L4, L1, L1, R3, L3, R4, L2, L4, L5, L5, L4, R2, R5, L4, R4, L2, R3, L4, L3, L5, R5, L4, L2, R3, R5, R5, L1, L4, R3, L1, R2, L5, L1, R4, L1, R5, R1, L4, L4, L4, R4, R3, L5, R1, L3, R4, R3, L2, L1, R1, R2, R2, R2, L1, L1, L2, L5, L3, L1";
    //let input = "R8, R4, R4, R8";
    let v: Vec<&str> = input.split(',').collect();
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut d = 0; // 0 = N, 1 = E, 2 = S, 3 = W

    let mut hs = HashSet::new();

    for s in v.iter() {
        let s = s.trim();
        let dir = &s[..1];
        match dir {
            "L" => d = (d + 3) % 4,
            "R" => d = (d + 1) % 4,
            _ => panic!("AAAAAAAAH!")
        }
        let dist = s[1..].parse::<i32>().unwrap();
        let mov = match d {
            0 => (0,1),
            1 => (1,0),
            2 => (0,-1),
            3 => (-1,0),
            _ => panic!("AAAAAAAAH!")
        };
        for _ in 0..dist {
            x += mov.0;
            y += mov.1;
            if hs.contains(&(x,y)) {
                println!("{:?} visited twice!!", (x,y));
                println!("{:?}", x.abs() + y.abs());
                process::exit(0);
            }
            hs.insert((x,y));
        }
    }

    println!("Nothing visited twice...");
}

