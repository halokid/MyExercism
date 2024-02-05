use std::collections::HashMap;
use std::thread;

fn count_chars(text: &[&str]) -> HashMap<char, usize> {
    let mut map = HashMap::new();
    text.iter()
        .flat_map(|&line| line.chars())
        .filter(|ch| ch.is_alphabetic())
        .filter_map(|ch| ch.to_lowercase().next())
        .for_each(|ch| *map.entry(ch).or_default() += 1);
    map
}

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    match input.len() {
        0 => HashMap::new(),
        l if l < 300 => count_chars(input),
        l => {
            thread::scope(|s| {
                let mut map = HashMap::new();
                let mut threads = Vec::with_capacity(worker_count);
                for chunk in input.chunks(l / worker_count + 1) {
                    threads.push(s.spawn(|| count_chars(chunk)));
                }
                for thread in threads {
                    thread.join().unwrap().iter().for_each(|(&ch, &count)| {
                        map.entry(ch).and_modify(|c| *c += count).or_insert(count);
                    });
                }
                map
            })
        }
    }
}


