use std::cell::RefCell;
use std::iter::Iterator;

use crate::hgd::{rhyper, Coins};
use crate::ope::ValueRange;

pub fn sample_hgd(
    in_range: ValueRange,
    out_range: ValueRange,
    nsample: i32,
    seed_coins: Box<dyn Iterator<Item = bool> + 'static>,
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

    let seed_coins: Coins = Box::new(seed_coins);
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
    seed_coins: Box<RefCell<dyn Iterator<Item = bool> + 'static>>,
) -> i32 {
    let mut cur_range = in_range;
    assert!(cur_range.size() != 0, "Range size must not be zero");

    let mut seed_coins_ref_mut = seed_coins.borrow_mut();

    while cur_range.size() > 1 {
        let mid = (cur_range.start + cur_range.end) / 2;
        match seed_coins_ref_mut.next() {
            // Utilisez cette référence mutable ici
            Some(false) => cur_range.end = mid,
            Some(true) => cur_range.start = mid + 1,
            None => panic!("Not enough coins"),
        }
    }

    assert!(cur_range.size() == 1, "Range size should be 1");
    cur_range.start
}
