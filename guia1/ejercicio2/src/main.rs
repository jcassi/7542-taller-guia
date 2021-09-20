use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;
use std::iter::FromIterator;

fn main() -> io::Result<()> {
    let file = File::open("contar_palabras.txt")?;
    let reader = BufReader::new(file);
    
    let frequencies = process_lines(reader);
    let mut frequencies = Vec::from_iter(frequencies.iter());
    frequencies.sort_by(|a, b| (b.1).cmp(&(a.1)));

    print_frequencies(frequencies);

    Ok(())
}

fn process_lines(reader: BufReader<File>) -> HashMap<String, u32>{
    let mut frequencies: HashMap<String, u32> = HashMap::new();

    for line in reader.lines() {
        match line {
            Ok(line) => process_line(line, &mut frequencies),
            Err(_) => (),
        }
    }
    frequencies
}

fn print_frequencies(frequencies: Vec<(&String, &u32)>) {
    for element in frequencies.iter() {
        println!("{} -> {}", element.0, element.1);
    }
}

fn process_line(linee: String, frequencies: &mut HashMap<String, u32>) {
    let line = linee.to_lowercase();
    let words : Vec<&str> = line.split(" ").collect();
    for word in words {
        let stat = frequencies.entry(word.to_string()).or_insert(0);
        *stat += 1;
    }
}
