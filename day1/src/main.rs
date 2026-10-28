use std::env; 
use std::fs;

fn sort3(arr: &mut [u32; 3]) {
    if arr[0] > arr[1] { arr.swap(0,1); }
    if arr[1] > arr[2] { arr.swap(1,2); }
    if arr[0] > arr[1] { arr.swap(0,1); }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("In file {}", file_path);
    
    let contents = fs::read_to_string(file_path)
        .expect("while reading {file_path}");

    let parts = contents.split('\n');

    let mut top3: [u32; 3] = [0; 3];
    let mut sum: u32 = 0;

    for part in parts {
        if part == "" {
            if sum > top3[0] {
                top3[0] = sum;
                sort3(&mut top3);
            }
            sum = 0;
        } else {
            let num: u32 = part.parse().unwrap();
            sum += num;
        }
    }

    // Last elf
    if sum > top3[0] {
        top3[0] = sum;
        sort3(&mut top3);
    }

    let mut total_carry = 0;
    for i in 0..3 {
        total_carry += top3[i];
        println!("Elf {} carrying {} calories", i + 1, top3[i]);
    }

    println!("Maximum calories carried by top 3 elves is {}", total_carry);

    // Functional way of doing things
    let contents = fs::read_to_string(file_path)
        .expect("while reading {file_path}")
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .collect::<Vec<_>>();
    
    let max_elf: u64 = contents
        .split(|v| v.is_none())
        .map(|group| group.iter().map(|x| x.unwrap()).sum::<u64>())
        .max()
        .unwrap();

    println!("Max Elf: {max_elf}");
}
