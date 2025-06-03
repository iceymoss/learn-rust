trait TraitA {}
trait TraitB {}
trait TraitC {}

struct A;
struct B;
struct C;

impl TraitA for A {}
impl TraitB for A {}
impl TraitC for A {}  // 对类型A实现了TraitA, TraitB, TraitC
impl TraitB for B {}
impl TraitC for B {}  // 对类型B实现了TraitB, TraitC
impl TraitC for C {}  // 对类型C实现了TraitC

// 7个版本的doit() 函数
fn doit1<T: TraitA + TraitB + TraitC>(t: T) {}
fn doit2<T: TraitA + TraitB>(t: T) {}
fn doit3<T: TraitA + TraitC>(t: T) {}
fn doit4<T: TraitB + TraitC>(t: T) {}
fn doit5<T: TraitA>(t: T) {}
fn doit6<T: TraitB>(t: T) {}
fn doit7<T: TraitC>(t: T) {}

fn main() {
    doit1(A);
    doit2(A);
    doit3(A);
    doit4(A);
    doit5(A);
    doit6(A);
    doit7(A);  // A的实例能用在所有7个函数版本中

    doit4(B);
    doit6(B);
    doit7(B);  // B的实例只能用在3个函数版本中

    doit7(C);  // C的实例只能用在1个函数版本中
}