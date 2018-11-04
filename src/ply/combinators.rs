pub fn identity<A>(x: A) -> A {
    x
}

pub fn constt<A,B>(a: A, _:B) -> A {
    a
}

pub fn apply<A,B, F>(f: F, a: A) -> B where F: Fn(A) -> B{
    f(a)
}

pub fn trush<A,B,F>(a: A, f:F) -> B where F: Fn(A) -> B {
    f(a)
}

pub fn flip<A,B,C,F>(f: F, b:B, a:A ) -> C where F: Fn(A,B) -> C {
    f(a,b)
}

pub fn compose<A,B,C,F,G>(f: F, g: G, a:A) -> C where F: Fn(B) -> C, G: Fn(A) -> B {
    // Just to make function private
    let _ = prv();
    f(g(a))
}

fn prv() -> &'static str {
   identity("Priv")
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_priv() {
        assert_eq!(super::prv(),  "Priv")
    }
}
