fn main() {
    let mut a = [1, 2, 3];    // 一个整数数组

    let mut an_iter = a.iter();  // 转换成第一种迭代器

    assert_eq!(Some(&1), an_iter.next());
    assert_eq!(Some(&2), an_iter.next());
    assert_eq!(Some(&3), an_iter.next());
    assert_eq!(None, an_iter.next());

    let mut an_iter = a.iter_mut();  // 转换成第二种迭代器

    assert_eq!(Some(&mut 1), an_iter.next());
    assert_eq!(Some(&mut 2), an_iter.next());
    assert_eq!(Some(&mut 3), an_iter.next());
    assert_eq!(None, an_iter.next());
    //
    let mut an_iter = a.into_iter();  // 转换成第三种迭代器，并消耗掉a

    assert_eq!(Some(1), an_iter.next());
    assert_eq!(Some(2), an_iter.next());
    assert_eq!(Some(3), an_iter.next());
    assert_eq!(None, an_iter.next());

    // println!("{:?}", a);
}