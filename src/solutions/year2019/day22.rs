use num::{BigInt, traits::Euclid};

// We can represent each step of the shuffle as a linear map
//
//     card -> a * card + b
//
// where card is the index of a given card in the deck. Linear maps are closed under composition, so
// in fact the whole shuffle is a linear map.
struct LinearMap<const M: i64>(BigInt, BigInt);

fn parse<const M: i64>(input: &str) -> impl Iterator<Item = LinearMap<M>> + '_ {
    input.lines().map(|line| {
        if line == "deal into new stack" {
            LinearMap(BigInt::from(-1), BigInt::from(-1))
        } else if line.starts_with("cut ") {
            LinearMap(
                BigInt::from(1),
                -line
                    .strip_prefix("cut ")
                    .unwrap()
                    .parse::<BigInt>()
                    .unwrap(),
            )
        } else if line.starts_with("deal with increment ") {
            LinearMap(
                line.strip_prefix("deal with increment ")
                    .unwrap()
                    .parse()
                    .unwrap(),
                BigInt::from(0),
            )
        } else {
            unreachable!()
        }
    })
}

impl<const M: i64> LinearMap<M> {
    fn compose(&self, other: &Self) -> Self {
        Self(
            (self.0.clone() * other.0.clone()).rem_euclid(&BigInt::from(M)),
            (self.1.clone() * other.0.clone() + other.1.clone()).rem_euclid(&BigInt::from(M)),
        )
    }

    // Relies on Fermat's Little Theorem, only valid for prime M.
    fn invert(&self) -> Self {
        let inv_0 = self.0.modpow(&BigInt::from(M - 2), &BigInt::from(M));
        Self(
            inv_0.clone(),
            (-inv_0 * self.1.clone()).rem_euclid(&BigInt::from(M)),
        )
    }

    fn pow(&self, exponent: i64) -> Self {
        if exponent == 0 {
            Self(BigInt::from(1), BigInt::from(0))
        } else if exponent < 0 {
            self.invert().pow(-exponent)
        } else if exponent % 2 == 0 {
            self.compose(self).pow(exponent / 2)
        } else {
            self.compose(&self.pow(exponent - 1))
        }
    }

    fn apply(&self, card: i64) -> BigInt {
        (self.0.clone() * card + self.1.clone()).rem_euclid(&BigInt::from(M))
    }
}

pub fn part1(input: &str) -> BigInt {
    parse::<10007>(input)
        .reduce(|a, b| LinearMap::compose(&a, &b))
        .unwrap()
        .apply(2019)
}

pub fn part2(input: &str) -> BigInt {
    parse::<119_315_717_514_047>(input)
        .reduce(|a, b| LinearMap::compose(&a, &b))
        .unwrap()
        .pow(-101_741_582_076_661)
        .apply(2020)
}

pub fn tests() {
    fn assert_shuffle(input: &str, result: [i64; 10]) {
        let shuffle = parse::<10>(input)
            .reduce(|a, b| LinearMap::compose(&a, &b))
            .unwrap();
        for card in 0..10 {
            assert_eq!(
                usize::try_from(shuffle.apply(card)).unwrap(),
                result.iter().position(|&c| c == card).unwrap(),
            );
        }
    }

    assert_shuffle("deal into new stack", [9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
    assert_shuffle("cut 3", [3, 4, 5, 6, 7, 8, 9, 0, 1, 2]);
    assert_shuffle("cut -4", [6, 7, 8, 9, 0, 1, 2, 3, 4, 5]);
    assert_shuffle("deal with increment 3", [0, 7, 4, 1, 8, 5, 2, 9, 6, 3]);
    assert_shuffle(
        "deal with increment 7\ndeal into new stack\ndeal into new stack",
        [0, 3, 6, 9, 2, 5, 8, 1, 4, 7],
    );
    assert_shuffle(
        "cut 6\ndeal with increment 7\ndeal into new stack",
        [3, 0, 7, 4, 1, 8, 5, 2, 9, 6],
    );
    assert_shuffle(
        "deal with increment 7\ndeal with increment 9\ncut -2",
        [6, 3, 0, 7, 4, 1, 8, 5, 2, 9],
    );
    assert_shuffle(
        &[
            "deal into new stack",
            "cut -2",
            "deal with increment 7",
            "cut 8",
            "cut -4",
            "deal with increment 7",
            "cut 3",
            "deal with increment 9",
            "deal with increment 3",
            "cut -1",
        ]
        .join("\n"),
        [9, 2, 5, 8, 1, 4, 7, 0, 3, 6],
    );
}
