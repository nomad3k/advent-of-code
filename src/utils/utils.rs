use std::path::{Path};
use std::fs::{File};
use std::io::{self, BufReader, BufRead, Lines};
use core::cmp::Ordering;

type VecIntoIter<T> = ::std::vec::IntoIter<T>;

pub trait MyIterator : Iterator {

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

    fn split_nth(self, count: usize) -> NthSplitter<Self>
        where Self: Sized,
              Self::Item: Clone,
    {
        NthSplitter { source: self, count }
    }
}

impl<T> MyIterator for T where T: Iterator { }

pub fn read_lines(filename: impl AsRef<Path>) -> io::Result<Lines<BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

type SplitFn<T> = fn(T) -> bool;

// -----------------------------------------------------------------------------
// NthSplitter
// -----------------------------------------------------------------------------

pub struct NthSplitter<T>
    where T: Iterator
{
    source: T,
    count: usize,
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

// -----------------------------------------------------------------------------
// Split an iterator into segments
// -----------------------------------------------------------------------------

pub struct SplitIter<T>
    where T: Clone
{
    source: Box<dyn Iterator<Item=T>>,
    check: SplitFn<T>,
}

impl<T> SplitIter<T>
    where T: Clone
{
    pub fn new(source: Box<dyn Iterator<Item=T>>, check: SplitFn<T>) -> SplitIter<T> {
        SplitIter { source, check }
    }
}

impl<T> Iterator for SplitIter<T> 
    where T: Clone
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Vec<T>> {
        if let Some(item) = self.source.next() {
            let mut res = vec![item];
            while let Some(item) = self.source.next() {
                if (self.check)(item.clone()) {
                    return Some(res);
                }
                res.push(item);
            }
            return Some(res);
        }
        None
    }
}

// Return the highest element of an iterator, or None if empty
pub fn highest(mut iter: impl Iterator<Item=i32>) -> Option<i32> {
    let first = iter.next()?;
    Some(iter.fold(first, |a, b| if a > b { a } else { b }))
}

// Return the topN elements of an iterator
pub fn top_n_values(values: impl Iterator<Item=i32>, len: usize) -> Vec<i32> {
    let mut top: Vec<i32> = Vec::new();
    for value in values {
        top.push(value);
        top.sort();
        if top.len() > len {
            top.remove(0);
        }
    }
    top
}

// Return the sum of an iterator of i32
pub fn sum(values: impl Iterator<Item=i32>) -> i32 {
    values.fold(0, |a,b| a+b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn highest_is_three() {
        let values = [1, 2, 3];
        let expected = 3;
        let actual = highest(values.into_iter());
        assert_eq!(actual, Some(expected));
    }

    #[test]
    fn highest_is_none() {
        let actual = highest([].into_iter());
        
        assert_eq!(actual, None);
    }

    #[test]
    fn split_none_returns_none() {
        let values: Vec<i32> = Vec::new();
        let actual = SplitIter::new(Box::new(values.into_iter()), |_| true).next();
        
        assert_eq!(actual, None);
    }

    #[test]
    fn split_works_as_expected() {
        let values = vec!["one", "two", "", "three", ""].into_iter();
        let mut splitter = SplitIter::new(Box::new(values), |s| s == "");
        let first = splitter.next();
        let second = splitter.next();
        let third = splitter.next();
        
        assert_eq!(first, Some(vec!["one", "two"]));
        assert_eq!(second, Some(vec!["three"]));
        assert_eq!(third, None);
    }

    #[test]
    fn top_n_values() {
        let values = vec![2, 5, 3, 9, 12, 4];
        let top = super::top_n_values(values.into_iter(), 3);

        assert_eq!(top, [5, 9, 12]);
    }

    #[test]
    fn sum() {
        let values = vec![1, 2, 3, 4, 5].into_iter();
        let actual = super::sum(values);

        assert_eq!(actual, 15);
    }

    #[test]
    fn split_nth() {
        let lines: Vec<i32> = vec![1,2,3,4,5,6];
        let mut iter = lines.into_iter().split_nth(3);
        let actual = iter.next();
        let expected = Some(vec![1,2,3]);

        assert_eq!(actual, expected);
    }
}
