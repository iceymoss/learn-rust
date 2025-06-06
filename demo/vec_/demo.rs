fn main() {
    let mut subject_list = vec![]; //声明一个vec, 这里可以不指明数据类型，在具体使用时，编译器会自动推断
    subject_list.push("math");
    println!("subject_list: {:?}", subject_list);



    let mut vector: Vec<i32> = vec![1, 2, 4, 8];
    vector.push(16);
    vector.push(32);
    vector.push(64);
    println!("{:?}", vector);

    let mut member_list: Vec<&str> = vec!["iceymoss", "kuk", "taks"];
    member_list.push("lak"); // push元素
    member_list.remove(0); // 移除索引为0的元素
    println!("{:?}", member_list);
    println!("{}", member_list[2]); // 索引访问
}