extern crate playground;
use playground::ply::combinators::*;

#[test]
fn test_identity() {
    assert_eq!(identity(33), 33);
}

#[test]
fn test_constt() {
    assert_eq!(constt("Ho!", 333), "Ho!");
}
#[test]
fn test_apply() {
    assert_eq!(apply(identity, "Ba!"), "Ba!");
}
#[test]
fn test_trush() {
    assert_eq!(trush("Ba!", identity), "Ba!");
}

#[test]
fn test_flip() {
    assert_eq!(flip(|x:u32, _:&str| { x }, "a", 1), 1);
}

#[test]
fn test_compose() {
    assert_eq!(compose(|x:u32 | { x }, |t: &str| {t.parse().unwrap()}, "66"), 66);
}
