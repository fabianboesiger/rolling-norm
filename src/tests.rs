use super::*;

#[test]
fn from_buf() {
    let rolling = Series::from([2.0, 4.0, 6.0]);
    approx_eq(rolling.curr(), 6.0);
    approx_eq(rolling.mean(), 4.0);
    approx_eq(rolling.var(), 2.66666);
    approx_eq(rolling.stdev(), 1.63299);
    approx_eq(rolling.norm(), (6.0 - 4.0) / 1.63299);
}

#[test]
fn sum() {
    let rolling = Series::from([2.0, 4.0, 6.0]);
    assert_eq!(rolling.sum(), 12.0);
}

#[test]
fn all_zero() {
    let rolling = Series::from([0.0, 0.0, 0.0]);
    approx_eq(rolling.curr(), 0.0);
    approx_eq(rolling.mean(), 0.0);
    approx_eq(rolling.var(), 0.0);
    approx_eq(rolling.stdev(), 0.0);
    approx_eq(rolling.norm(), 0.0);
}

#[test]
fn mean_stdev() {
    let mut rolling = Series::<f64, 3>::new();
    rolling.insert(2.0);
    rolling.insert(4.0);
    rolling.insert(6.0);
    approx_eq(rolling.curr(), 6.0);
    approx_eq(rolling.mean(), 4.0);
    approx_eq(rolling.stdev(), 1.63299);
    rolling.insert(8.0);
    approx_eq(rolling.curr(), 8.0);
    approx_eq(rolling.mean(), 6.0);
    approx_eq(rolling.stdev(), 1.63299);
    rolling.insert(8.0);
    rolling.insert(8.0);
    approx_eq(rolling.curr(), 8.0);
    approx_eq(rolling.mean(), 8.0);
    approx_eq(rolling.stdev(), 0.0);
}

#[test]
fn index() {
    let mut rolling = Series::<f64, 3>::new();
    rolling.insert(2.0);
    rolling.insert(4.0);
    rolling.insert(6.0);
    assert_eq!(rolling[0], 6.0);
    assert_eq!(rolling[1], 4.0);
    assert_eq!(rolling[2], 2.0);
}

fn approx_eq<R: Real + std::fmt::Debug>(a: R, b: R) {
    assert!(
        (a - b).abs() < R::from(0.01).unwrap(),
        "Numbers {:?} and {:?} aren't roughly equal.",
        a,
        b
    );
}
