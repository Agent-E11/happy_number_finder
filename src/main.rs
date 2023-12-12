use std::io;
use std::time::Instant;

fn main() {
    let mut buf = String::new();

    let num_count = loop {
        println!("\nHow many happy numbers do you want?:");
        io::stdin().read_line(&mut buf).expect("Error reading line");

        match buf.trim().parse::<u32>() {
            Err(e) => eprintln!("{e}"),
            Ok(n) => break n,
        }

        buf.clear();
    };

    let mut happy_nums: Vec<u32> = Vec::new();
    let mut curr_num = 0;
    
    let start = Instant::now();

    while happy_nums.len() < num_count as usize {
        if is_happy(curr_num) {
            happy_nums.push(curr_num);
        }
        curr_num += 1;
    }

    println!("Happy numbers: {:?}", happy_nums);
    println!("Time elapsed: {}", start.elapsed().as_micros());
}

fn is_happy(usr_num: u32) -> bool {
    let mut num = usr_num;
    let mut history: Vec<u32> = Vec::new();

    loop {
        let new_num: u32 = num.to_string().chars()
            .map(|d| d.to_digit(10).unwrap().pow(2))
            .sum();

        if new_num == 1 {
            break true;
        } else if history.contains(&new_num) {
            break false;
        } else {
            num = new_num;
            history.push(new_num);
        }
    }
}
