struct Atype;
struct Btype;
struct Ctype;

trait TraitA {}

impl TraitA for Atype {}
impl TraitA for Btype {}
impl TraitA for Ctype {}

// trait object。形式上，就是在 trait 名前加 dyn 关键字修饰，在这个例子里就是 dyn TraitA。dyn TraitName 本身就是一种类型，它和 TraitName 这个 trait 相关，但是它们不同，dyn TraitName 是一个独立的类型。
// fn doit(i: u32) -> impl TraitA {
//     if i == 0 {
//         let a = Atype;
//         return a;          // 这里用return语句直接从函数返回
//     } else if i == 1 {
//         let b = Btype;
//         return b;
//     } else {
//         let c = Ctype;
//         return c;
//     }
// }


// 编译失败：它说 dyn TraitA 编译时尺寸未知。dyn trait 确实不是一个固定尺寸类型。然后给出了第一个建议：你可以用 impl TraitA 来解决，前提是所有分支返回同一类型。随后给出了第二个建议，你可以用 Box 把 dyn TraitA 包起来。
// fn doit(i: u32) -> dyn TraitA { // 注意这里的返回类型换成了 dyn TraitA
//     if i == 0 {
//         let a = Atype;
//         return a
//     } else if i == 1 {
//         let b = Btype;
//         return b
//     } else {
//         let c = Ctype;
//         return c
//     }
// }

// Box 的作用是可以保证获得里面值的所有权，必要的时候会进行内存的复制，比如把栈上的值复制到堆中去。一旦值到了堆中，就很容易掌握到它的所有权。
fn doit(i: u32) -> Box<dyn TraitA> {
    if i == 0 {
        let a = Atype;
        Box::new(a)
    } else if i == 1 {
        let b = Atype;
        Box::new(b)
    } else {
        let c = Atype;
        Box::new(c)
    }
}

fn main() {
    doit(0);
}