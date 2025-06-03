trait TraitA {
    type Mytype;
}

fn doit<T: TraitA>(a: T::Mytype) {}  // 这里在函数中使用了关联类型

struct TypeA;
impl TraitA for TypeA {
    type Mytype = String;  // 具化关联类型为String
}

struct TypeB;
impl TraitA for TypeB {
    type Mytype = i32;
}

struct TypeC;

impl TraitA for TypeC {
    type Mytype = bool;
}

fn main() {
    doit::<TypeA>("abc".to_string());  // 给Rustc小助手喂信息：T具化为TypeA
    doit::<TypeB>(101);
    doit::<TypeC>(true);
}