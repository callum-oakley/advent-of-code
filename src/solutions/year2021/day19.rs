use std::{collections::HashMap, sync::LazyLock};

use crate::{freqs::Freqs, grid::Vector3, uniq::Uniq};

struct Scanner {
    position: Vector3,
    beacons: Vec<Vector3>,
    // Frequencies of distances between each pair of beacons.
    fingerprint: HashMap<i64, usize>,
}

impl Scanner {
    fn new(beacons: Vec<Vector3>) -> Self {
        Self {
            position: Vector3::zeros(),
            fingerprint: crate::combinatorics::combinations(2, &beacons)
                .map(|pair| (pair[0] - pair[1]).abs().sum())
                .freqs(),
            beacons,
        }
    }
}

static ROTATIONS: LazyLock<[nalgebra::Matrix3<i64>; 24]> = LazyLock::new(|| {
    let i = nalgebra::matrix![1, 0, 0; 0, 1, 0; 0, 0, 1];
    let x = nalgebra::matrix![1, 0, 0; 0, 0, -1; 0, 1, 0];
    let y = nalgebra::matrix![0, 0, 1; 0, 1, 0; -1, 0, 0];
    [
        i,
        x,
        y,
        x * x,
        x * y,
        y * x,
        y * y,
        x * x * x,
        x * x * y,
        x * y * x,
        x * y * y,
        y * x * x,
        y * y * x,
        y * y * y,
        x * x * x * y,
        x * x * y * x,
        x * x * y * y,
        x * y * x * x,
        x * y * y * y,
        y * x * x * x,
        y * y * y * x,
        x * x * x * y * x,
        x * y * x * x * x,
        x * y * y * y * x,
    ]
});

fn parse(input: &str) -> impl Iterator<Item = Scanner> {
    input.trim().split("\n\n").map(|scanner| {
        Scanner::new(
            scanner
                .lines()
                .skip(1)
                .map(crate::cast::str_to_vector3)
                .collect(),
        )
    })
}

// The distances between pairs of beacons doesn't depend on the position or orientation of the
// scanner, so if two scanners share 12 beacons, they must share 12 choose 2 = 66 distances.
fn fingerprints_match(a: &HashMap<i64, usize>, b: &HashMap<i64, usize>) -> bool {
    a.iter()
        .filter_map(|(dist, freq_a)| b.get(dist).map(|freq_b| freq_a.min(freq_b)))
        .sum::<usize>()
        >= 66
}

fn offset(beacons0: &[Vector3], beacons1: &[Vector3]) -> Option<Vector3> {
    beacons0
        .iter()
        .flat_map(|&b0| beacons1.iter().map(move |&b1| b0 - b1))
        .freqs()
        .iter()
        .find(|&(_, &freq)| freq >= 12)
        .map(|(&offset, _)| offset)
}

fn fix(fixed_scanners: &[Scanner], floating_scanners: &mut Vec<Scanner>) -> Option<Scanner> {
    for fixed_scanner in fixed_scanners {
        for (i, floating_scanner) in floating_scanners.iter().enumerate() {
            if !fingerprints_match(&fixed_scanner.fingerprint, &floating_scanner.fingerprint) {
                continue;
            }
            for rotation in ROTATIONS.iter() {
                let floating_beacons: Vec<Vector3> = floating_scanner
                    .beacons
                    .iter()
                    .map(|b| rotation * b)
                    .collect();
                if let Some(offset) = offset(&fixed_scanner.beacons, &floating_beacons) {
                    let mut scanner = floating_scanners.swap_remove(i);
                    scanner.position = offset;
                    scanner.beacons = floating_beacons.iter().map(|b| b + offset).collect();
                    return Some(scanner);
                }
            }
        }
    }
    None
}

fn part_(input: &str) -> Vec<Scanner> {
    let mut scanners = parse(input);
    let mut fixed_scanners: Vec<Scanner> = vec![scanners.next().unwrap()];
    let mut floating_scanners: Vec<Scanner> = scanners.collect();
    while let Some(scanner) = fix(&fixed_scanners, &mut floating_scanners) {
        fixed_scanners.push(scanner);
    }
    assert!(floating_scanners.is_empty());
    fixed_scanners
}

pub fn part1(input: &str) -> usize {
    part_(input)
        .into_iter()
        .flat_map(|s| s.beacons)
        .uniq()
        .count()
}

pub fn part2(input: &str) -> i64 {
    let scanners: Vec<Vector3> = part_(input).iter().map(|s| s.position).collect();
    scanners
        .iter()
        .flat_map(|a| scanners.iter().map(move |b| (a - b).abs().sum()))
        .max()
        .unwrap()
}

#[expect(clippy::too_many_lines)]
pub fn tests() {
    let example = "
        --- scanner 0 ---
        404,-588,-901
        528,-643,409
        -838,591,734
        390,-675,-793
        -537,-823,-458
        -485,-357,347
        -345,-311,381
        -661,-816,-575
        -876,649,763
        -618,-824,-621
        553,345,-567
        474,580,667
        -447,-329,318
        -584,868,-557
        544,-627,-890
        564,392,-477
        455,729,728
        -892,524,684
        -689,845,-530
        423,-701,434
        7,-33,-71
        630,319,-379
        443,580,662
        -789,900,-551
        459,-707,401

        --- scanner 1 ---
        686,422,578
        605,423,415
        515,917,-361
        -336,658,858
        95,138,22
        -476,619,847
        -340,-569,-846
        567,-361,727
        -460,603,-452
        669,-402,600
        729,430,532
        -500,-761,534
        -322,571,750
        -466,-666,-811
        -429,-592,574
        -355,545,-477
        703,-491,-529
        -328,-685,520
        413,935,-424
        -391,539,-444
        586,-435,557
        -364,-763,-893
        807,-499,-711
        755,-354,-619
        553,889,-390

        --- scanner 2 ---
        649,640,665
        682,-795,504
        -784,533,-524
        -644,584,-595
        -588,-843,648
        -30,6,44
        -674,560,763
        500,723,-460
        609,671,-379
        -555,-800,653
        -675,-892,-343
        697,-426,-610
        578,704,681
        493,664,-388
        -671,-858,530
        -667,343,800
        571,-461,-707
        -138,-166,112
        -889,563,-600
        646,-828,498
        640,759,510
        -630,509,768
        -681,-892,-333
        673,-379,-804
        -742,-814,-386
        577,-820,562

        --- scanner 3 ---
        -589,542,597
        605,-692,669
        -500,565,-823
        -660,373,557
        -458,-679,-417
        -488,449,543
        -626,468,-788
        338,-750,-386
        528,-832,-391
        562,-778,733
        -938,-730,414
        543,643,-506
        -524,371,-870
        407,773,750
        -104,29,83
        378,-903,-323
        -778,-728,485
        426,699,580
        -438,-605,-362
        -469,-447,-387
        509,732,623
        647,635,-688
        -868,-804,481
        614,-800,639
        595,780,-596

        --- scanner 4 ---
        727,592,562
        -293,-554,779
        441,611,-461
        -714,465,-776
        -743,427,-804
        -660,-479,-426
        832,-632,460
        927,-485,-438
        408,393,-506
        466,436,-512
        110,16,151
        -258,-428,682
        -393,719,612
        -211,-452,876
        808,-476,-593
        -575,615,604
        -485,667,467
        -680,325,-822
        -627,-443,-432
        872,-547,-609
        833,512,582
        807,604,487
        839,-516,451
        891,-625,532
        -652,-548,-490
        30,-46,-14
    ";
    assert_eq!(part1(example), 79);
    assert_eq!(part2(example), 3621);
}
