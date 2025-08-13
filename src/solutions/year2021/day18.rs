use crate::sexp::{Inner, Value};

fn parse(input: &str) -> impl Iterator<Item = Value> {
    input.trim().lines().map(|line| line.parse().unwrap())
}

fn pair(a: Value, b: Value) -> Value {
    Value::vec(vec![a, b])
}

fn explode(value: &Value) -> Option<Value> {
    fn add_leftmost(value: &Value, carry: i32) -> Value {
        match value.as_inner() {
            Inner::Int(int) => Value::int(int + carry),
            Inner::Vec(values) => pair(add_leftmost(&values[0], carry), values[1].clone()),
        }
    }

    fn add_rightmost(value: &Value, carry: i32) -> Value {
        match value.as_inner() {
            Inner::Int(int) => Value::int(int + carry),
            Inner::Vec(values) => pair(values[0].clone(), add_rightmost(&values[1], carry)),
        }
    }

    fn go(value: &Value, depth: u8) -> Option<(i32, Value, i32)> {
        match value.as_inner() {
            Inner::Int(_) => None,
            Inner::Vec(values) => {
                if depth == 4 {
                    Some((
                        values[0].as_int().unwrap(),
                        Value::int(0),
                        values[1].as_int().unwrap(),
                    ))
                } else {
                    go(&values[0], depth + 1)
                        .map(|(left_carry, v, right_carry)| {
                            (
                                left_carry,
                                pair(v, add_leftmost(&values[1], right_carry)),
                                0,
                            )
                        })
                        .or_else(|| {
                            go(&values[1], depth + 1).map(|(left_carry, v, right_carry)| {
                                (
                                    0,
                                    pair(add_rightmost(&values[0], left_carry), v),
                                    right_carry,
                                )
                            })
                        })
                }
            }
        }
    }

    go(value, 0).map(|(_, value, _)| value)
}

fn split(value: &Value) -> Option<Value> {
    match value.as_inner() {
        Inner::Int(int) => {
            if *int >= 10 {
                Some(pair(Value::int(int / 2), Value::int(int - int / 2)))
            } else {
                None
            }
        }
        Inner::Vec(values) => {
            if let Some(v) = split(&values[0]) {
                Some(pair(v, values[1].clone()))
            } else {
                split(&values[0])
                    .map(|v| pair(v, values[1].clone()))
                    .or_else(|| split(&values[1]).map(|v| pair(values[0].clone(), v)))
            }
        }
    }
}

fn magnitude(value: &Value) -> i32 {
    match value.as_inner() {
        Inner::Int(int) => *int,
        Inner::Vec(values) => 3 * magnitude(&values[0]) + 2 * magnitude(&values[1]),
    }
}

fn add(a: Value, b: Value) -> Value {
    let mut value = pair(a, b);
    while let Some(v) = explode(&value).or_else(|| split(&value)) {
        value = v;
    }
    value
}

pub fn part1(input: &str) -> i32 {
    magnitude(&parse(input).reduce(add).unwrap())
}

pub fn part2(input: &str) -> i32 {
    let values: Vec<_> = parse(input).collect();
    values
        .iter()
        .flat_map(|a| values.iter().map(|b| magnitude(&add(a.clone(), b.clone()))))
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
