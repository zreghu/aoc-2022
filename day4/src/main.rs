use std::env;
use std::fs;

fn main() {
    let file_name = env::args().nth(1)
        .expect("file name should be first argument");

    let contents = fs::read_to_string(file_name)
        .expect("could not find {file_name}"); 

    let bounds = contents
        .lines()
        .map(|s| { 
            let b = s.split(",")
                .into_iter()
                .map(|s2| s2.split("-")
                     .into_iter()
                     .map(|s3| s3.parse().ok())
                     .collect::<Vec<Option<u32>>>())
                .filter_map( |v| match v[..] {
                    [Some(x), Some(y)] => Some((x, y)),
                    _ => None
                })
                .collect::<Vec<(u32, u32)>>();
            dbg!(b)
        });

    let contain_count = bounds
        .clone()
        .filter(|x| check_contains_bounds(x))
        .count();

    let overlap_count = bounds
        .filter(|x| check_overlap_bounds(x))
        .count();

    println!("Number of times work is redundant is {contain_count}");
    println!("Number of times bounds overlap is {overlap_count}");
}

fn check_contains_bounds(bound: &Vec<(u32, u32)>) -> bool {
    let (l1, r1, l2, r2) = match bound[..][..] {
        [(l1, r1), (l2, r2)] => (l1, r1, l2, r2),
        _ => return false
    };

    if (l1 <= l2 && r1 >= r2) || (l1 >= l2 && r1 <= r2) {
        return true;
    }

    return false;
}


fn check_overlap_bounds(bound: &Vec<(u32, u32)>) -> bool {
    let (l1, r1, l2, r2) = match bound[..][..] {
        [(l1, r1), (l2, r2)] => (l1, r1, l2, r2),
        _ => return false
    };

    if (l1 <= l2 && r1 >= l2) || (l1 >= l2 && l1 <= r2) {
        return true;
    }

    return false;
}
