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

fn main() {
    println!("Hello, world!");
    println!("identity! {}",  identity(33));
    println!("const! {}",  constt("Ho!", 333));
    println!("apply! {}",  apply(identity, "Ba!"));
    println!("thrush! {}",  trush("Ba!", identity));
    println!("flip! {}",  flip(|x:u32, _:&str| { x }, "a", 1));
    println!("compose! {}",  compose(|x:u32 | { x }, |t: &str| {t.parse().unwrap()}, "66"));
}
