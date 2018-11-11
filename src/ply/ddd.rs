use std::marker::PhantomData;

#[derive(Debug, Clone, PartialEq, Copy)]
struct Item(u32, &'static str);

#[derive(Debug, PartialEq)]
struct PaymentMethod;

enum Payed{}
type UnpayedItems = (Vec<Item>, PhantomData<UnPayed>);

enum UnPayed{}
type PayedItems = (Vec<Item>, PhantomData<Payed>);

#[derive(Debug, PartialEq)]
enum ShoppingCart {
    EmptyCart,
    ActiveCart(UnpayedItems),
    PaidCart(PayedItems, PaymentMethod)
}

// enum CartErrorKind {
//     PaidCannotAdd,
// }

fn new_cart()  -> ShoppingCart {
    ShoppingCart::EmptyCart
}

fn items_in_cart(c: ShoppingCart)  -> usize {
    match c {
        ShoppingCart::EmptyCart => 0,
        ShoppingCart::ActiveCart((items, _)) => items.len(),
        ShoppingCart::PaidCart((items, _), _) => items.len(),
    }
}

// fn add_to_cart(c: ShoppingCart, i: Item) -> Result<ShoppingCart, CartErrorKind> {
fn add_to_cart(c: ShoppingCart, i: Item) -> ShoppingCart {
    match c {
       ShoppingCart::EmptyCart => add_to_cart(ShoppingCart::ActiveCart((Vec::new(), PhantomData)), i),
       ShoppingCart::ActiveCart((items, _)) => {
           let mut items2 = items.clone();
                   items2.push(i);
           ShoppingCart::ActiveCart((items, PhantomData))
       },
       ShoppingCart::PaidCart(_,_) => c
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn axioms<F>(add: F, x:Item) where F: Fn(ShoppingCart, Item) -> ShoppingCart {
        let a = new_cart();
        let b = new_cart();
        let a1 = add(a, x);
        let a2 = add(a1, x);
        let b1 = add(b, x);
        assert_eq!(a2, b1)
    }

    #[test]
    fn it_works() {
        let i = Item(33, "Phone");
        axioms(add_to_cart, i)
    }
}
