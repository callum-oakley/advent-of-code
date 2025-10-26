fn dragon_fill(disk: usize, curve: &mut Vec<bool>) {
    while curve.len() < disk {
        let prev_len = curve.len();
        curve.push(false);
        for i in ((2 * prev_len + 1).saturating_sub(disk)..prev_len).rev() {
            curve.push(!curve[i]);
        }
    }
}

fn checksum(curve: &[bool]) -> String {
    let mut res_len = curve.len();
    let mut chunk_size = 1;
    while res_len.is_multiple_of(2) {
        res_len /= 2;
        chunk_size *= 2;
    }

    let mut res = String::with_capacity(res_len);
    for chunk in curve.chunks(chunk_size) {
        if chunk.iter().filter(|bit| **bit).count() % 2 == 0 {
            res.push('1');
        } else {
            res.push('0');
        }
    }
    res
}

fn part_(disk: usize, input: &str) -> String {
    let mut curve = Vec::with_capacity(disk);
    curve.extend(input.chars().map(|c| c == '1'));
    dragon_fill(disk, &mut curve);
    checksum(&curve)
}

pub fn part1(input: &str) -> String {
    part_(272, input)
}

pub fn part2(input: &str) -> String {
    part_(35_651_584, input)
}

pub fn tests() {
    assert_eq!(part_(20, "10000"), "01100");
}
