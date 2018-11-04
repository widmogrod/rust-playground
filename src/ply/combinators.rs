
fn identity<A>(x: A) -> A {
    x
}

fn constt<A,B>(a: A, _:B) -> A {
    a
}

fn apply<A,B, F>(f: F, a: A) -> B where F: Fn(A) -> B{
    f(a)
}

fn trush<A,B,F>(a: A, f:F) -> B where F: Fn(A) -> B {
    f(a)
}

fn flip<A,B,C,F>(f: F, b:B, a:A ) -> C where F: Fn(A,B) -> C {
    f(a,b)
}

fn compose<A,B,C,F,G>(f: F, g: G, a:A) -> C where F: Fn(B) -> C, G: Fn(A) -> B {
    f(g(a))
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
