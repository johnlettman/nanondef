use nanondef::Range;

#[test]
fn test_len() {
    let r = Range::new(2, 5);
    assert_eq!(r.len(), 3);

    let r = Range::new(0, 0);
    assert_eq!(r.len(), 0);

    let r = Range::new(10, 10);
    assert_eq!(r.len(), 0);
}

#[test]
fn test_display() {
    let r = Range::new(3, 6);
    assert_eq!(format!("{}", r), "3..6");
}

#[test]
fn test_ord() {
    let r1 = Range::new(0, 10);
    let r2 = Range::new(5, 10);
    assert!(r1 > r2);

    let r1 = Range::new(6, 8);
    let r2 = Range::new(20, 41);
    assert!(r1 < r2);

    let r1 = Range::new(20, 30);
    let r2 = Range::new(30, 40);
    assert_eq!(r1, r2);
}

#[test]
fn index_basic() {
    let data: [u8; 6] = [10, 20, 30, 40, 50, 60];
    let r = Range::new(1, 4);

    assert_eq!(&data[r], &[20, 30, 40]);
}

#[test]
fn index_empty_range() {
    let data: [u8; 4] = [1, 2, 3, 4];
    let r = Range::new(2, 2);

    assert_eq!(&data[r], &[]);
}

#[test]
fn index_full_range() {
    let data = [9, 8, 7, 6];
    let r = Range::new(0, data.len());

    assert_eq!(&data[r], &[9, 8, 7, 6]);
}

#[test]
fn index_mut_basic() {
    let mut data = [1, 2, 3, 4, 5];
    let r = Range { start: 1, end: 4 };

    data[r][0] = 20;
    data[r][1] = 30;
    data[r][2] = 40;

    assert_eq!(data, [1, 20, 30, 40, 5]);
}

#[test]
#[should_panic]
fn index_out_of_bounds_panics() {
    let data = [1, 2, 3];
    let r = Range { start: 1, end: 10 }; // too large

    // This should panic
    let _ = &data[r];
}
