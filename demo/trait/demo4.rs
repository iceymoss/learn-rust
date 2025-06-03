use std::fmt::Debug;

trait TraitA {
    type Item: Debug;  // 这里对关联类型添加了Debug约束
}

#[derive(Debug)]       // 这里在类型A上自动derive Debug约束
struct A;

struct B;

impl TraitA for B {
    type Item = A;  // 这里这个类型A已满足Debug约束
}