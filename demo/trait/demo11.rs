struct Atype;
struct Btype;
struct Ctype;

trait TraitA {
    fn new() -> Self;    // TraitA中定义了new()函数
}

impl TraitA for Atype {
    fn new() -> Atype {
        Atype
    }
}

impl TraitA for Btype {
    fn new() -> Btype {
        Btype
    }
}

impl TraitA for Ctype {
    fn new() -> Ctype {
        Ctype
    }
}

fn doit<T: TraitA>() -> T {
    T::new()
}

fn main() {
    let a: Atype = doit::<Atype>();
    let b: Btype = doit::<Btype>();
    let c: Ctype = doit::<Ctype>();
}