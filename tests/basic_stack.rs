use abin::StackBin;

#[test]
pub fn stack_empty() {
    StackBin::try_from(&[]).expect("Empty must be stack-allocated");
}

#[test]
pub fn stack_max_ok() {
    let slice = [15u8; StackBin::max_len()];
    StackBin::try_from(&slice).expect("Max len must be stack-allocated");
}

#[test]
pub fn stack_too_large() {
    let slice = [15u8; StackBin::max_len() + 1];
    // this was a bit too long, should no longer be possible to stack-allocated this.
    assert!(StackBin::try_from(&slice).is_none());
}

