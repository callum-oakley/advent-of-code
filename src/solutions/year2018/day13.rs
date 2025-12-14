use crate::{
    grid::{Grid, LEFT, RIGHT, Vector},
    part::Part,
};

enum Tile {
    Empty,
    Straight,
    TurnFSlash,
    TurnBSlash,
    Intersection,
}

struct Cart {
    pos: Vector,
    dir: Vector,
    turn_count: usize,
}

impl Cart {
    fn tick(&mut self, track: &Grid<Tile>) {
        self.pos += self.dir;
        match track[self.pos] {
            Tile::Empty => unreachable!(),
            Tile::Straight => {}
            Tile::TurnFSlash => {
                if self.dir.x == 0 {
                    self.dir = RIGHT * self.dir;
                } else {
                    self.dir = LEFT * self.dir;
                }
            }
            Tile::TurnBSlash => {
                if self.dir.x == 0 {
                    self.dir = LEFT * self.dir;
                } else {
                    self.dir = RIGHT * self.dir;
                }
            }
            Tile::Intersection => {
                match self.turn_count % 3 {
                    0 => {
                        self.dir = LEFT * self.dir;
                    }
                    2 => {
                        self.dir = RIGHT * self.dir;
                    }
                    _ => {}
                }
                self.turn_count += 1;
            }
        }
    }
}

fn parse(input: &str) -> (Grid<Tile>, Vec<Cart>) {
    let mut carts = Vec::new();
    let track = Grid::parse(input, |pos, c| match c {
        ' ' => Tile::Empty,
        '/' => Tile::TurnFSlash,
        '\\' => Tile::TurnBSlash,
        '+' => Tile::Intersection,
        '^' | 'v' | '<' | '>' => {
            carts.push(Cart {
                pos,
                dir: crate::cast::char_to_vector(c),
                turn_count: 0,
            });
            Tile::Straight
        }
        _ => Tile::Straight,
    });
    (track, carts)
}

fn tick(part: Part, track: &Grid<Tile>, carts: &mut Vec<Cart>) -> Option<Vector> {
    carts.sort_unstable_by_key(|c| [c.pos.y, c.pos.x]);

    let mut i = 0;
    while i < carts.len() {
        let pos = carts[i].pos + carts[i].dir;
        if let Some(j) = carts.iter().position(|c| c.pos == pos) {
            match part {
                Part::One => {
                    return Some(pos);
                }
                Part::Two => {
                    carts.remove(j);
                    if j < i {
                        i -= 1;
                    }
                    carts.remove(i);
                    continue;
                }
            }
        }

        carts[i].tick(track);
        i += 1;
    }

    None
}

pub fn part1(input: &str) -> String {
    let (track, mut carts) = parse(input);
    loop {
        if let Some(crash) = tick(Part::One, &track, &mut carts) {
            return format!("{},{}", crash.x, crash.y);
        }
    }
}

pub fn part2(input: &str) -> String {
    let (track, mut carts) = parse(input);
    while carts.len() >= 2 {
        tick(Part::Two, &track, &mut carts);
    }
    format!("{},{}", carts[0].pos.x, carts[0].pos.y)
}

pub fn tests() {
    assert_eq!(
        part1(
            &[
                r"/->-\        ",
                r"|   |  /----\",
                r"| /-+--+-\  |",
                r"| | |  | v  |",
                r"\-+-/  \-+--/",
                r"  \------/   ",
            ]
            .join("\n")
        ),
        "7,3",
    );
    assert_eq!(
        part2("/>-<\\  \n|   |  \n| /<+-\\\n| | | v\n\\>+</ |\n  |   ^\n  \\<->/"),
        "6,4",
    );
}
