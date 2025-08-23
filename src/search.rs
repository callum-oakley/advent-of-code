use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashSet, VecDeque},
    hash::Hash,
    ops::Add,
};

use num::PrimInt;

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

/// Prunes the search space whenever we encounter a state with a hash key we've already seen.
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

/// Prunes the search space whenever we encounter a state we've already seen.
pub fn id_filter<S: Clone + Eq + Hash>() -> impl FnMut(&S) -> bool {
    hash_filter(Clone::clone)
}

/// Doesn't prune the search space at all.
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
    std::iter::from_fn(move || {
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

/// Search a state space for a lowest cost solution given: a cost function to evaluate a given
/// state, and a bound function to give a lower bound on the cost of the best state reachable from a
/// given state.
pub fn branch_and_bound_min<S, A, F, C, B, O>(
    start: S,
    adjacent: A,
    mut filter: F,
    cost: C,
    bound: B,
) -> S
where
    S: Clone,
    A: FnMut(&S, &mut dyn FnMut(S)),
    F: FnMut(&S) -> bool,
    C: Fn(&S) -> O,
    B: Fn(&S) -> O,
    O: Ord,
{
    let mut best_final_state = start.clone();
    let mut best_cost = cost(&best_final_state);
    dijkstra(
        start,
        adjacent,
        |state| {
            if !filter(state) {
                return false;
            }
            let cost = cost(state);
            if cost < best_cost {
                best_cost = cost;
                best_final_state = state.clone();
            }
            // The bound is the best cost we could hope for from this state, so if it isn't an
            // improvement on the best cost we've already seen, there's no reason to explore this
            // branch.
            bound(state) < best_cost
        },
        &cost,
    )
    .for_each(drop);
    best_final_state
}

/// Search a state space for a highest score solution given: a score function to evaluate a given
/// state, and a bound function to give an upper bound on the score of the best state reachable from
/// a given state.
pub fn branch_and_bound_max<S, A, F, C, B, O>(
    start: S,
    adjacent: A,
    filter: F,
    score: C,
    bound: B,
) -> S
where
    S: Clone,
    A: FnMut(&S, &mut dyn FnMut(S)),
    F: FnMut(&S) -> bool,
    C: Fn(&S) -> O,
    B: Fn(&S) -> O,
    O: Ord,
{
    branch_and_bound_min(
        start,
        adjacent,
        filter,
        |state| Reverse(score(state)),
        |state| Reverse(bound(state)),
    )
}

/// Finds the smallest value where pred is true, assuming that it is false for all lower values, and
/// true for all higher, given a low and high initial bound.
pub fn binary<N, F>(mut low: N, mut high: N, mut pred: F) -> N
where
    N: PrimInt,
    F: FnMut(N) -> bool,
{
    assert!(!pred(low));
    assert!(pred(high));
    while high - low > N::one() {
        let mid = (high + low) / N::from(2).unwrap();
        if pred(mid) {
            high = mid;
        } else {
            low = mid;
        }
    }
    high
}

/// Finds the smallest value where pred is true, assuming that it is false for all lower values, and
/// true for all higher, given only a low initial bound.
pub fn exponential<N, F>(mut low: N, mut pred: F) -> N
where
    N: PrimInt,
    F: FnMut(N) -> bool,
{
    assert!(!pred(low));
    let mut size = N::one();
    while !pred(low + size) {
        low = low + size;
        size = size * N::from(2).unwrap();
    }

    binary(low, low + size, pred)
}
