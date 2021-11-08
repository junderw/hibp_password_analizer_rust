use std::{
    collections::HashMap,
    env,
    fs::File,
    io::prelude::*,
    io::{self, BufReader},
};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Need only one arg!");
    }
    let mut i = 0usize;
    let mut target_index = 0usize;
    let target_list: Vec<usize> = (1..=100000).rev().collect();
    let mut keys: Vec<usize> = Vec::new();
    let mut important_numbers: HashMap<usize, usize> = HashMap::new();

    let f = File::open(&args[1])?;
    let f = BufReader::new(f);

    let mut f2 = File::create("output.csv")?;

    for line in f.lines() {
        i += 1;

        let line = line?;
        let mut parts = line.split(':');
        let _ = &parts.next();
        let count = parts.next().unwrap().parse::<usize>().unwrap();

        if count < target_list[target_index] {
            keys.push(target_list[target_index]);

            while count < target_list[target_index] - 1 {
                target_index += 1;
                keys.push(target_list[target_index]);
                important_numbers.insert(target_list[target_index], i);
            }
            target_index += 1;
        }
        important_numbers.insert(target_list[target_index], i);
        if i % 10000000 == 0 {
            println!("{}", i);
        }
    }
    keys.push(target_list[target_index]);
    keys.reverse();
    f2.write_all(format!("{},{}\n", "pwnedCount", "numberOfPasswords").as_bytes())?;
    for k in &keys {
        f2.write_all(format!("{},{}\n", k, important_numbers[k]).as_bytes())?;
    }

    Ok(())
}
