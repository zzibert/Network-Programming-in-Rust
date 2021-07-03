fn main() {
    let mut times = 2;
    {
        // This is in a new scope
        let mut borrow = |x| times += x;
        borrow(5);
    }
    assert_eq!(times, 7);

    let mut own = move |x| times += x;
    own(5);
    assert_eq!(times, 8);
}