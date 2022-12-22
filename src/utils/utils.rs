type SplitFn<T> = fn(T) -> bool;

// Split an iterator into segments
pub struct SplitIter<T> where T: Clone {
    source: Box<dyn Iterator<Item=T>>,
    check: SplitFn<T>,
}

impl<T> SplitIter<T> where T: Clone {
    pub fn new(source: Box<dyn Iterator<Item=T>>, check: SplitFn<T>) -> SplitIter<T> {
        SplitIter { source: source, check: check }
    }
}

impl<T> Iterator for SplitIter<T> where T: Clone {
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
}
