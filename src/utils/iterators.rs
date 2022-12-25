use std::cmp::Ordering;

type VecIntoIter<T> = ::std::vec::IntoIter<T>;

pub trait IteratorExtensions : Iterator {

    fn sorted(self) -> VecIntoIter<Self::Item>
        where Self: Sized,
              Self::Item: Ord
    {
        let mut v = Vec::from_iter(self);
        v.sort();
        v.into_iter()
    }

    fn sorted_by<F>(self, cmp: F) -> VecIntoIter<Self::Item>
        where Self: Sized,
              F: FnMut(&Self::Item, &Self::Item) -> Ordering,
    {
        let mut v = Vec::from_iter(self);
        v.sort_by(cmp);
        v.into_iter()
    }

    fn deduped(self) -> VecIntoIter<Self::Item>
        where Self: Sized,
              Self::Item: PartialEq,
    {
        let mut v = Vec::from_iter(self);
        v.dedup_by(|a,b| a==b);
        v.into_iter()
    }

    /// Returns an iterator that is just for the next N elements
    fn split_nth(self, count: usize) -> NthSplitter<Self>
        where Self: Sized,
              Self::Item: Clone,
    {
        NthSplitter { source: self, count }
    }
}

impl<T> IteratorExtensions for T where T: Iterator { }

pub struct NthSplitter<T>
    where T: Iterator
{
    pub source: T,
    pub count: usize,
}

impl<T,U> Iterator for NthSplitter<T>
    where T: Iterator<Item=U>,
{
    type Item = Vec<U>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result : Vec<U> = vec![];
        for _ in 1..=self.count {
            let item = self.source.next()?;
            result.push(item);
        }
        Some(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn split_nth() {
        let lines: Vec<i32> = vec![1,2,3,4,5,6];
        let mut iter = lines.into_iter().split_nth(3);
        let first = iter.next();
        let second = iter.next();
        let third = iter.next();

        assert_eq!(first, Some(vec![1,2,3]));
        assert_eq!(second, Some(vec![4,5,6]));
        assert_eq!(third, None);
    }
}
