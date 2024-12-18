use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashSet, VecDeque},
    hash::Hash,
    iter,
    ops::Add,
};

trait Queue {
    type Item;

    fn push(&mut self, value: Self::Item);
    fn pop(&mut self) -> Option<Self::Item>;
}

/// Common or garden first-in-first-out queue.
impl<T> Queue for VecDeque<T> {
    type Item = T;

    fn push(&mut self, value: Self::Item) {
        VecDeque::push_back(self, value);
    }

    fn pop(&mut self) -> Option<Self::Item> {
        VecDeque::pop_front(self)
    }
}

struct CostValue<V, O> {
    cost: O,
    value: V,
}

impl<V, O: PartialEq> PartialEq for CostValue<V, O> {
    fn eq(&self, other: &Self) -> bool {
        self.cost.eq(&other.cost)
    }
}

impl<V, O: Eq> Eq for CostValue<V, O> {}

impl<V, O: PartialOrd> PartialOrd for CostValue<V, O> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.cost.partial_cmp(&other.cost)
    }
}

impl<V, O: Ord> Ord for CostValue<V, O> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
    }
}

struct CostHeap<V, C, O> {
    cost: C,
    binary_heap: BinaryHeap<Reverse<CostValue<V, O>>>,
}

/// Priority queue which pops the lowest cost item first.
impl<V, C, O> Queue for CostHeap<V, C, O>
where
    C: FnMut(&V) -> O,
    O: Ord,
{
    type Item = V;

    fn push(&mut self, value: Self::Item) {
        self.binary_heap.push(Reverse(CostValue {
            cost: (self.cost)(&value),
            value,
        }));
    }

    fn pop(&mut self) -> Option<Self::Item> {
        self.binary_heap.pop().map(|w| w.0.value)
    }
}

/// A filter that prunes the tree whenever we encounter a state with a hash key we've already seen.
pub fn hash_filter<S, H, K>(mut hash_key: H) -> impl FnMut(&S) -> bool
where
    H: FnMut(&S) -> K,
    K: Eq + Hash,
{
    let mut visited = HashSet::new();
    move |state| {
        let key = hash_key(state);
        if visited.contains(&key) {
            false
        } else {
            visited.insert(key);
            true
        }
    }
}

/// A filter that prunes the tree whenever we encounter a state we've already seen.
pub fn id_filter<S: Clone + Eq + Hash>() -> impl FnMut(&S) -> bool {
    hash_filter(Clone::clone)
}

/// A filter that doesn't prune the tree at all.
pub fn no_filter<S>(_: &S) -> bool {
    true
}

fn search<Q, S, A, F>(
    mut queue: Q,
    start: S,
    mut adjacent: A,
    mut filter: F,
) -> impl Iterator<Item = S>
where
    Q: Queue<Item = S>,
    A: FnMut(&S, &mut dyn FnMut(S)),
    F: FnMut(&S) -> bool,
{
    queue.push(start);
    iter::from_fn(move || {
        while let Some(state) = queue.pop() {
            if filter(&state) {
                adjacent(&state, &mut |a| queue.push(a));
                return Some(state);
            }
        }
        None
    })
}

/// Search a state space breadth first.
pub fn breadth_first<S, A, F>(start: S, adjacent: A, filter: F) -> impl Iterator<Item = S>
where
    A: FnMut(&S, &mut dyn FnMut(S)),
    F: FnMut(&S) -> bool,
{
    search(VecDeque::new(), start, adjacent, filter)
}

/// Search a state space min-cost first.
pub fn dijkstra<S, A, F, C, O>(start: S, adjacent: A, filter: F, cost: C) -> impl Iterator<Item = S>
where
    A: FnMut(&S, &mut dyn FnMut(S)),
    F: FnMut(&S) -> bool,
    C: FnMut(&S) -> O,
    O: Ord,
{
    search(
        CostHeap {
            cost,
            binary_heap: BinaryHeap::new(),
        },
        start,
        adjacent,
        filter,
    )
}

/// Search a state space min-cost-plus-heuristic first.
pub fn a_star<S, A, F, C, D, O>(
    start: S,
    adjacent: A,
    filter: F,
    mut cost: C,
    mut heuristic: D,
) -> impl Iterator<Item = S>
where
    A: FnMut(&S, &mut dyn FnMut(S)),
    F: FnMut(&S) -> bool,
    C: FnMut(&S) -> O,
    D: FnMut(&S) -> O,
    O: Add,
    O::Output: Ord,
{
    dijkstra(start, adjacent, filter, move |state| {
        cost(state) + heuristic(state)
    })
}

/// Like `slice::partition_point` but between two indices rather than on a slice.
pub fn binary<F>(mut low: usize, mut high: usize, mut pred: F) -> usize
where
    F: FnMut(usize) -> bool,
{
    assert!(pred(low));
    assert!(!pred(high));
    while high - low > 1 {
        let mid = (high + low) / 2;
        if pred(mid) {
            low = mid;
        } else {
            high = mid;
        }
    }
    high
}

pub fn exponential<F>(mut low: usize, mut pred: F) -> usize
where
    F: FnMut(usize) -> bool,
{
    assert!(pred(low));
    let mut size = 1;
    while pred(low + size) {
        low += size;
        size *= 2;
    }

    binary(low, low + size, pred)
}
