use rolling_norm::Series;

fn main() {
    let mut rolling = Series::from([1.0, 2.0, 3.0]);
    // The latest value is at index 0.
    assert_eq!(rolling[0], 3.0);
    assert_eq!(rolling[1], 2.0);
    assert_eq!(rolling[2], 1.0);
    println!("The most recent value is {}.", rolling.curr());
    assert_eq!(rolling.curr(), rolling[0]);

    println!("The mean is {}.", rolling.mean());
    println!("The variance is {}.", rolling.var());
    println!("The standard derivation is {}.", rolling.stdev());
    println!("The mean is {}.", rolling.mean());

    // When inserting a new value, the oldest value
    // is removed and the values below are recomputed.
    rolling.insert(3.0);
    assert_eq!(rolling[0], 3.0);
    assert_eq!(rolling[1], 3.0);
    assert_eq!(rolling[2], 2.0);

    println!("The new mean is {}.", rolling.mean());
    println!("The new variance is {}.", rolling.var());
    println!("The new standard derivation is {}.", rolling.stdev());
    println!("The new mean is {}.", rolling.mean());
}
