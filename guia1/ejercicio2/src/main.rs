use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;
use std::iter::FromIterator;


//Refactorizar separando en funciones luego de ver ownership y lifetimes
fn main() -> io::Result<()> {
    let file = File::open("contar_palabras.txt")?;
    let reader = BufReader::new(file);
    let mut frequencies: HashMap<String, u32> = HashMap::new();

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let line = line.to_lowercase();
                let words : Vec<&str> = line.split(" ").collect();
                for word in words {
                    let stat = frequencies.entry(word.to_string()).or_insert(0);
                    *stat += 1;
                }
            }
            Err(_) => (),
        }
    }

    let mut frequencies = Vec::from_iter(frequencies.iter());

    frequencies.sort_by(|a, b| (b.1).cmp(&(a.1)));

    for element in frequencies.iter() {
        println!("{} -> {}", element.0, element.1);
    }

    Ok(())
}
