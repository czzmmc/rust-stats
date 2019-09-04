use std::prelude::v1::*;
use stats::{Frequencies,MinMax,Commute,OnlineStats,merge_all};

pub fn ranked() {
        let mut counts = Frequencies::new();
        counts.extend(vec![1usize, 1, 2, 2, 2, 2, 2, 3, 4, 4, 4].into_iter());
        assert_eq!(counts.most_frequent()[0], (&2, 5));
        assert_eq!(counts.least_frequent()[0], (&3, 1));
    }

pub fn minmax() {
        let minmax: MinMax<u32> =
            vec![1u32, 4, 2, 3, 10].into_iter().collect();
        assert_eq!(minmax.min(), Some(&1u32));
        assert_eq!(minmax.max(), Some(&10u32));
    }


pub fn minmax_merge_empty() {
        let mut mx1: MinMax<u32> = vec![1, 4, 2, 3, 10].into_iter().collect();
        assert_eq!(mx1.min(), Some(&1u32));
        assert_eq!(mx1.max(), Some(&10u32));

        mx1.merge(MinMax::default());
        assert_eq!(mx1.min(), Some(&1u32));
        assert_eq!(mx1.max(), Some(&10u32));
    }

pub fn stddev() {
        // TODO: Convert this to a quickcheck test.
        let expected = OnlineStats::from_slice(&[1usize, 2, 3, 2, 4, 6]);

        let var1 = OnlineStats::from_slice(&[1usize, 2, 3]);
        let var2 = OnlineStats::from_slice(&[2usize, 4, 6]);
        let mut got = var1;
        got.merge(var2);
        assert_eq!(expected.stddev(), got.stddev());
    }

pub fn stddev_many() {
        // TODO: Convert this to a quickcheck test.
        let expected = OnlineStats::from_slice(
            &[1usize, 2, 3, 2, 4, 6, 3, 6, 9]);

        let vars = vec![
            OnlineStats::from_slice(&[1usize, 2, 3]),
            OnlineStats::from_slice(&[2usize, 4, 6]),
            OnlineStats::from_slice(&[3usize, 6, 9]),
        ];
        assert_eq!(expected.stddev(),
                   merge_all(vars.into_iter()).unwrap().stddev());
    }