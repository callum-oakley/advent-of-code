use crate::sexp::Value;

fn parse(input: &str) -> impl Iterator<Item = Value> {
    input.trim().lines().map(|line| line.parse().unwrap())
}

fn pair(a: &Value, b: &Value) -> Value {
    Value::from([a, b])
}

fn explode(value: &Value) -> Option<Value> {
    fn add_leftmost(value: &Value, carry: i32) -> Value {
        if value.is_int() {
            Value::int(value.unint() + carry)
        } else {
            pair(&add_leftmost(&value[0], carry), &value[1])
        }
    }

    fn add_rightmost(value: &Value, carry: i32) -> Value {
        if value.is_int() {
            Value::int(value.unint() + carry)
        } else {
            pair(&value[0], &add_rightmost(&value[1], carry))
        }
    }

    fn go(value: &Value, depth: u8) -> Option<(i32, Value, i32)> {
        if value.is_int() {
            None
        } else if depth == 4 {
            Some((value[0].unint(), Value::int(0), value[1].unint()))
        } else {
            go(&value[0], depth + 1)
                .map(|(left_carry, v, right_carry)| {
                    (
                        left_carry,
                        pair(&v, &add_leftmost(&value[1], right_carry)),
                        0,
                    )
                })
                .or_else(|| {
                    go(&value[1], depth + 1).map(|(left_carry, v, right_carry)| {
                        (
                            0,
                            pair(&add_rightmost(&value[0], left_carry), &v),
                            right_carry,
                        )
                    })
                })
        }
    }

    go(value, 0).map(|(_, value, _)| value)
}

fn split(value: &Value) -> Option<Value> {
    if value.is_int() {
        let int = value.unint();
        if int >= 10 {
            Some(pair(&Value::int(int / 2), &Value::int(int - int / 2)))
        } else {
            None
        }
    } else if let Some(v) = split(&value[0]) {
        Some(pair(&v, &value[1]))
    } else {
        split(&value[0])
            .map(|v| pair(&v, &value[1]))
            .or_else(|| split(&value[1]).map(|v| pair(&value[0], &v)))
    }
}

fn magnitude(value: &Value) -> i32 {
    if value.is_int() {
        value.unint()
    } else {
        3 * magnitude(&value[0]) + 2 * magnitude(&value[1])
    }
}

fn add(a: &Value, b: &Value) -> Value {
    let mut value = pair(a, b);
    while let Some(v) = explode(&value).or_else(|| split(&value)) {
        value = v;
    }
    value
}

pub fn part1(input: &str) -> i32 {
    magnitude(&parse(input).reduce(|a, b| add(&a, &b)).unwrap())
}

pub fn part2(input: &str) -> i32 {
    let numbers: Vec<_> = parse(input).collect();
    numbers
        .iter()
        .flat_map(|a| numbers.iter().map(|b| magnitude(&add(a, b))))
        .max()
        .unwrap()
}

pub fn tests() {
    let example = "
        [[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
        [[[5,[2,8]],4],[5,[[9,9],0]]]
        [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
        [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
        [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
        [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
        [[[[5,4],[7,7]],8],[[8,3],8]]
        [[9,3],[[9,9],[6,[4,9]]]]
        [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
        [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
    ";
    assert_eq!(part1(example), 4140);
    assert_eq!(part2(example), 3993);
}
