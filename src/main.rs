use std::io;

fn main() {
    let mut buf = String::new();

    let usr_num = loop {
        io::stdin().read_line(&mut buf).expect("Error reading line");

        match buf.trim().parse::<u32>() {
            Err(e) => eprintln!("{e}"),
            Ok(n) => break n,
        }

        buf.clear();
    };

    let mut num = usr_num;
    let mut history: Vec<u32> = Vec::new();

    let is_happy = loop {
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
    };

    if is_happy {
        println!("\n`{usr_num}` is happy :)");
    } else {
        println!("\n`{usr_num}` is not happy :(");
    }
}
