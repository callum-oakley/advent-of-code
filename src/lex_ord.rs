use std::cmp::Ordering;

/// Wrapper implementing a [lexicographic order](https://en.wikipedia.org/wiki/Lexicographic_order)
/// for any `IntoIterator` yielding `Ord` items.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct LexOrd<T>(pub T);

impl<T> PartialOrd for LexOrd<T>
where
    T: Eq,
    for<'a> &'a T: IntoIterator,
    for<'a> <&'a T as IntoIterator>::Item: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for LexOrd<T>
where
    T: Eq,
    for<'a> &'a T: IntoIterator,
    for<'a> <&'a T as IntoIterator>::Item: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        let mut self_it = self.0.into_iter();
        let mut other_it = other.0.into_iter();
        loop {
            match (self_it.next(), other_it.next()) {
                (None, None) => return Ordering::Equal,
                (None, Some(_)) => return Ordering::Less,
                (Some(_), None) => return Ordering::Greater,
                (Some(a), Some(b)) => match a.cmp(&b) {
                    Ordering::Less => return Ordering::Less,
                    Ordering::Greater => return Ordering::Greater,
                    Ordering::Equal => {}
                },
            }
        }
    }
}
