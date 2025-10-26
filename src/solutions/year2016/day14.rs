use std::io::{Cursor, Seek, Write};

use md5::{Digest, Md5};

enum State {
    None,
    Triple(u8),
    MatchingPair,
}

// For each index, track the first triple digit (if any) and then if there is
// also a quintuple digit scan back and look for matching triples. We find
// quintuples much less frequently than triples so this saves a lot of work
// compared to scanning forwards looking for quintuples for every triple.
fn part_(stretch_rounds: usize, input: &str) -> usize {
    let mut count = 0;
    let mut states = Vec::new();

    let mut hasher = Md5::new();
    let mut buffer = Cursor::new(Vec::new());
    // The hash is always exactly 32 hex digits, so we can use a fixed buffer.
    let mut hash = Cursor::new([0; 32]);
    for i in 0.. {
        write!(buffer, "{input}{i}").unwrap();
        hasher.update(buffer.get_ref());
        buffer.rewind().unwrap();
        write!(hash, "{:x}", hasher.finalize_reset()).unwrap();

        for _ in 0..stretch_rounds {
            hasher.update(hash.get_ref());
            hash.rewind().unwrap();
            write!(hash, "{:x}", hasher.finalize_reset()).unwrap();
        }

        states.push(State::None);

        // NOTE "Only consider the first such triplet in a hash"
        'triple: for triple in hash.get_ref().windows(3) {
            for digit in 0..16 {
                let hex = char::from_digit(digit.into(), 16).unwrap() as u8;
                if triple.iter().all(|b| *b == hex) {
                    states[i] = State::Triple(digit);
                    break 'triple;
                }
            }
        }

        for quintuple in hash.get_ref().windows(5) {
            for digit in 0..16 {
                let hex = char::from_digit(digit.into(), 16).unwrap() as u8;
                if quintuple.iter().all(|b| *b == hex) {
                    for state in &mut states[i.saturating_sub(1000)..i] {
                        if let State::Triple(triple) = state
                            && *triple == digit {
                                // We can't count this yet, because we may yet
                                // discover another lower index which satisfies
                                // the criteria.
                                *state = State::MatchingPair;
                            }
                    }
                }
            }
        }

        hash.rewind().unwrap();

        if i >= 1000 {
            let j = i - 1000;
            // States before this can't be changed any more, so if it's a
            // MatchingPair, it's the next key.
            if let State::MatchingPair = states[j] {
                count += 1;
                if count == 64 {
                    return j;
                }
            }
        }
    }
    unreachable!()
}

pub fn part1(input: &str) -> usize {
    part_(0, input)
}

pub fn part2(input: &str) -> usize {
    part_(2016, input)
}

pub fn tests() {
    assert_eq!(part1("abc"), 22728);
    assert_eq!(part2("abc"), 22551);
}
