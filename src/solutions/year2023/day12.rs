use std::collections::HashMap;

fn parse(input: &str) -> impl Iterator<Item = (Vec<u8>, Vec<usize>)> + '_ {
    input.lines().map(|line| {
        let (springs, groups) = line.split_once(' ').unwrap();
        (
            Vec::from(springs),
            groups.split(',').map(|n| n.parse().unwrap()).collect(),
        )
    })
}

fn count_arrangements(
    cache: &mut HashMap<(usize, usize), usize>,
    springs: &[u8],
    groups: &[usize],
) -> usize {
    fn count_arrangements_inner(
        cache: &mut HashMap<(usize, usize), usize>,
        springs: &[u8],
        groups: &[usize],
    ) -> usize {
        if springs.is_empty() && groups.is_empty() {
            1
        } else if springs.is_empty() {
            0
        } else {
            match springs[0] {
                b'.' => count_arrangements(cache, &springs[1..], groups),
                b'#' => count_arrangements_starting_group(cache, springs, groups),
                b'?' => {
                    count_arrangements(cache, &springs[1..], groups)
                        + count_arrangements_starting_group(cache, springs, groups)
                }
                _ => unreachable!(),
            }
        }
    }

    fn count_arrangements_starting_group(
        cache: &mut HashMap<(usize, usize), usize>,
        springs: &[u8],
        groups: &[usize],
    ) -> usize {
        if groups.is_empty() || springs.len() < groups[0] || springs[1..groups[0]].contains(&b'.') {
            0
        } else if springs.len() == groups[0] {
            count_arrangements(cache, &springs[groups[0]..], &groups[1..])
        } else if springs[groups[0]] != b'#' {
            count_arrangements(cache, &springs[groups[0] + 1..], &groups[1..])
        } else {
            0
        }
    }

    let key = (springs.len(), groups.len());
    if let Some(&res) = cache.get(&key) {
        return res;
    }
    let res = count_arrangements_inner(cache, springs, groups);
    cache.insert(key, res);
    res
}

pub fn part1(input: &str) -> usize {
    parse(input)
        .map(|(springs, groups)| count_arrangements(&mut HashMap::new(), &springs, &groups))
        .sum()
}

pub fn part2(input: &str) -> usize {
    parse(input)
        .map(|(springs, groups)| {
            let mut unfolded = springs.clone();
            for _ in 0..4 {
                unfolded.push(b'?');
                unfolded.extend_from_slice(&springs);
            }
            count_arrangements(&mut HashMap::new(), &unfolded, &groups.repeat(5))
        })
        .sum()
}

pub fn tests() {
    let example = [
        "???.### 1,1,3",
        ".??..??...?##. 1,1,3",
        "?#?#?#?#?#?#?#? 1,3,1,6",
        "????.#...#... 4,1,1",
        "????.######..#####. 1,6,5",
        "?###???????? 3,2,1",
    ]
    .join("\n");
    assert_eq!(part1(&example), 21);
    assert_eq!(part2(&example), 525_152);
}
