use std::collections::BTreeMap;

use async_trait::async_trait;

#[async_trait]
pub trait FilterResources<T>
where
    T: Ord + Copy,
{
    /// get non unique: [3, 2, 5, 1, 5, 7, 2, 1] -> [1, 2, 5]
    fn non_unique(&self, input: Vec<T>) -> Vec<T> {
        let mut resp_map = BTreeMap::<T, u32>::new();

        for v in input {
            match resp_map.get_mut(&v) {
                Some(v) => {
                    *v += 1;
                }
                None => {
                    resp_map.insert(v, 1);
                }
            }
        }

        resp_map.iter().filter(|v| v.1 > &1).map(|v| *v.0).collect()
    }
}

pub struct Filter;

#[async_trait]
impl FilterResources<u32> for Filter {}

#[cfg(test)]
mod tests {
    use super::FilterResources;

    use rstest::rstest;

    #[rstest]
    #[case(vec![3, 2, 5, 1, 5, 7, 2, 1], vec![1, 2, 5])]
    #[case(vec![5, 7, 7], vec![7])]
    fn non_unique(#[case] input: Vec<u32>, #[case] output: Vec<u32>) {
        let filter = super::Filter;

        assert_eq!(filter.non_unique(input), output);
    }
}
