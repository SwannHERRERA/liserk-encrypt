use std::iter::Iterator;

use crate::hgd::rhyper;
use crate::test::ValueRange;

pub fn sample_hgd(
    in_range: ValueRange,
    out_range: ValueRange,
    nsample: i32,
    seed_coins: &mut impl Iterator<Item = i32>,
) -> i32 {
    let in_size = in_range.size();
    let out_size = out_range.size();
    assert!(in_size > 0 && out_size > 0, "Ranges must have positive size");
    assert!(
        in_size <= out_size,
        "Input range size must be less than or equal to output range size"
    );
    assert!(out_range.contains(nsample), "nsample must be within output range");

    let nsample_index = nsample - out_range.start + 1;
    if in_size == out_size {
        return in_range.start + nsample_index - 1;
    }

    // Placeholder for sampling from the hypergeometric distribution.
    // Replace this with a call to an actual statistical library.
    let in_sample_num =
        rhyper(nsample_index, in_size as f64, (out_size - in_size) as f64, seed_coins);

    if in_sample_num == 0 {
        in_range.start
    } else {
        let in_sample = in_range.start + in_sample_num - 1;
        assert!(in_range.contains(in_sample), "Sample not in input range");
        in_sample
    }
}

pub fn sample_uniform(
    in_range: ValueRange,
    seed_coins: &mut impl Iterator<Item = i32>,
) -> i32 {
    let mut cur_range = in_range;
    assert!(cur_range.size() != 0, "Range size must not be zero");

    while cur_range.size() > 1 {
        let mid = (cur_range.start + cur_range.end) / 2;
        match seed_coins.next() {
            Some(0) => cur_range.end = mid,
            Some(1) => cur_range.start = mid + 1,
            None => panic!("Not enough coins"),
            _ => panic!("Invalid coin"),
        }
    }

    assert!(cur_range.size() == 1, "Range size should be 1");
    cur_range.start
}
