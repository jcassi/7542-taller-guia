use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;
use std::thread;

fn main() -> io::Result<()> {
    let paths: Vec<Vec<String>> = vec!["contar1.txt".to_owned(), "contar2.txt".to_owned(), "contar3.txt".to_owned(), 
                                        "contar4.txt".to_owned()].chunks(2).map(|s| s.into()).collect();
    
    let mut thread_handles = vec![];    
    spawn_threads(paths, &mut thread_handles);

    let mut frequencies: Vec<HashMap<String, u32>> = vec![];
    for handle in thread_handles {
        frequencies.push(handle.join().unwrap());        
    }

    let frequencies = frequencies
            .into_iter()
            .reduce(|a, b| merge_hashmap_counter(a, b))
            .unwrap();
    let frequencies = hashmap_to_vec(frequencies);
    print_frequencies(frequencies);
    Ok(())
}

fn hashmap_to_vec(map: HashMap<String, u32>) -> Vec<(String, u32)> {
    let mut frequencies: Vec<(String, u32)> = map
                                                .into_iter()
                                                .collect();
    frequencies.sort_by(|a, b| (b.1).cmp(&(a.1)));
    frequencies
}

fn spawn_threads(paths: Vec<Vec<String>>, handles: &mut Vec<thread::JoinHandle<HashMap<String, u32>>>) {
    for chunk in paths {
        handles.push(
            thread::spawn(move || {
                process_files(chunk)})
        );
    }
}

fn process_files(paths: Vec<String>) -> HashMap<String, u32>{
    let files = paths
        .iter()
        .flat_map(File::open)
        .map(BufReader::new)
        .map(process_file)
        .reduce(|a, b| merge_hashmap_counter(a, b))
        .unwrap();
    files
}

fn merge_hashmap_counter(map1: HashMap<String, u32>, mut map2: HashMap<String, u32>) -> HashMap<String, u32>{
    let mut map: HashMap<String, u32> = HashMap::new();
    for (key, value) in map1 {
        if map2.contains_key(&key) {
            map.insert((*key).to_string(), value + map2.get(&key).unwrap());
            map2.remove(&key);
        } else {
            map.insert((*key).to_string(), 1);
        }
    }
    for (key, value) in map2 {
        map.insert((*key).to_string(), value);
    }
    map
}


fn process_file(reader: BufReader<File>) -> HashMap<String, u32>{
    let mut frequencies: HashMap<String, u32> = HashMap::new();

    for line in reader.lines() {
        match line {
            Ok(line) => process_line(line, &mut frequencies),
            Err(_) => (),
        }
    }
    frequencies
}


fn print_frequencies(frequencies: Vec<(String, u32)>) {
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