use std::collections::HashMap;

fn parse(input: &str) -> HashMap<&str, Vec<&str>> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (device, outputs) = line.trim().split_once(": ").unwrap();
            (device, outputs.split_whitespace().collect())
        })
        .collect()
}

fn count_paths<'a>(
    cache: &mut HashMap<(&'a str, &'a str), usize>,
    wires: &HashMap<&'a str, Vec<&'a str>>,
    from: &'a str,
    to: &'a str,
) -> usize {
    if let Some(&res) = cache.get(&(from, to)) {
        res
    } else {
        let res = if from == to {
            1
        } else if let Some(outputs) = wires.get(from) {
            outputs
                .iter()
                .map(|&output| count_paths(cache, wires, output, to))
                .sum()
        } else {
            0
        };
        cache.insert((from, to), res);
        res
    }
}

pub fn part1(input: &str) -> usize {
    count_paths(&mut HashMap::new(), &parse(input), "you", "out")
}

pub fn part2(input: &str) -> usize {
    let wires = parse(input);
    let mut cache = HashMap::new();
    [["svr", "dac", "fft", "out"], ["svr", "fft", "dac", "out"]]
        .iter()
        .map(|path| {
            path.windows(2)
                .map(|pair| count_paths(&mut cache, &wires, pair[0], pair[1]))
                .product::<usize>()
        })
        .sum()
}

pub fn tests() {
    let example1 = "
        aaa: you hhh
        you: bbb ccc
        bbb: ddd eee
        ccc: ddd eee fff
        ddd: ggg
        eee: out
        fff: out
        ggg: out
        hhh: ccc fff iii
        iii: out
    ";
    let example2 = "
        svr: aaa bbb
        aaa: fft
        fft: ccc
        bbb: tty
        tty: ccc
        ccc: ddd eee
        ddd: hub
        hub: fff
        eee: dac
        dac: fff
        fff: ggg hhh
        ggg: out
        hhh: out
    ";
    assert_eq!(part1(example1), 5);
    assert_eq!(part2(example2), 2);
}
