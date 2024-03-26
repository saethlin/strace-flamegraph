#[derive(Default)]
struct Sample {
    weight: u64,
    stack: Vec<String>,
}

fn main() {
    let mut parsed = Vec::new();
    let mut sample = Sample::default();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let fields: Vec<_> = line.split(' ').collect();
        if line.starts_with(" > ") {
            // If we didn't see a header yet, skip this frame.
            if sample.weight == 0  {
                continue;
            }
            let mut symbol = line.splitn(2, '(').nth(1).unwrap();
            symbol = symbol.rsplitn(2, ')').nth(1).unwrap();
            let mut symbol = symbol.replace(';', ":");
            if symbol.contains('+') {
                symbol = symbol.rsplitn(2, '+').nth(1).unwrap().to_string();
            }
            sample.stack.push(symbol.to_string());
            continue;
        }
        if fields[1].starts_with("mmap(") {
            if sample.weight != 0 {
                // This is the start of a new sample. We need to push the old sample and refresh it.
                parsed.push(std::mem::take(&mut sample));
            }
            let size = fields[2];
            let size = size[..size.len() - 1].parse::<u64>().unwrap();
            sample.weight = size;
            continue;
        }
        // Something else. Purge state.
        sample = Sample::default();
    }

    for p in parsed {
        let mut line = String::new();
        for func in p.stack.iter().rev() {
            line.push_str(func);
            line.push(';');
        }
        line.pop();
        println!("{line} {}", p.weight);
    }
}
