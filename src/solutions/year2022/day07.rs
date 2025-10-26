use std::collections::HashMap;

enum Fs<'a> {
    Dir(HashMap<&'a str, Fs<'a>>),
    File(usize),
}

impl<'a> Fs<'a> {
    fn insert(&mut self, path: &[&'a str], name: &'a str, fs: Self) {
        let mut dir = self.unwrap_dir();
        for component in path {
            dir = dir.get_mut(component).unwrap().unwrap_dir();
        }
        dir.insert(name, fs);
    }

    fn unwrap_dir(&mut self) -> &mut HashMap<&'a str, Fs<'a>> {
        match self {
            Fs::Dir(d) => d,
            Fs::File(_) => panic!("unwrap_dir on a file"),
        }
    }

    fn walk(&self) -> impl Iterator<Item = &Self> {
        crate::search::breadth_first(
            self,
            |fs, push| {
                if let Fs::Dir(d) = fs {
                    d.values().for_each(push);
                }
            },
            crate::search::no_filter,
        )
    }
}

fn parse(input: &str) -> Fs<'_> {
    let mut fs = Fs::Dir(HashMap::new());
    let mut cwd = Vec::new();
    let mut lines = input.lines().peekable();

    while let Some(line) = lines.next() {
        if line == "$ cd /" {
            cwd = Vec::new();
        } else if line == "$ cd .." {
            cwd.pop();
        } else if line.starts_with("$ cd ") {
            cwd.push(line.strip_prefix("$ cd ").unwrap());
        } else if line == "$ ls" {
            while lines.peek().is_some_and(|line| !line.starts_with('$')) {
                let (size, name) = lines.next().unwrap().split_once(' ').unwrap();
                if size == "dir" {
                    fs.insert(&cwd, name, Fs::Dir(HashMap::new()));
                } else {
                    fs.insert(&cwd, name, Fs::File(size.parse().unwrap()));
                }
            }
        } else {
            unreachable!()
        }
    }

    fs
}

fn total_size(fs: &Fs) -> usize {
    match fs {
        Fs::Dir(d) => d.values().map(total_size).sum(),
        Fs::File(size) => *size,
    }
}

pub fn part1(input: &str) -> usize {
    parse(input)
        .walk()
        .filter(|fs| matches!(fs, Fs::Dir(_)))
        .map(total_size)
        .filter(|&size| size <= 100_000)
        .sum()
}

pub fn part2(input: &str) -> usize {
    let fs = parse(input);
    let free = 70_000_000 - total_size(&fs);
    fs.walk()
        .filter(|fs| matches!(fs, Fs::Dir(_)))
        .map(total_size)
        .filter(|&size| free + size >= 30_000_000)
        .min()
        .unwrap()
}

pub fn tests() {
    let example = concat!(
        "$ cd /\n",
        "$ ls\n",
        "dir a\n",
        "14848514 b.txt\n",
        "8504156 c.dat\n",
        "dir d\n",
        "$ cd a\n",
        "$ ls\n",
        "dir e\n",
        "29116 f\n",
        "2557 g\n",
        "62596 h.lst\n",
        "$ cd e\n",
        "$ ls\n",
        "584 i\n",
        "$ cd ..\n",
        "$ cd ..\n",
        "$ cd d\n",
        "$ ls\n",
        "4060174 j\n",
        "8033020 d.log\n",
        "5626152 d.ext\n",
        "7214296 k\n",
    );
    assert_eq!(part1(example), 95437);
    assert_eq!(part2(example), 24_933_642);
}
