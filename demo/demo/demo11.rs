
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// 不要使用 clone，使用 copy 的方式替代
fn main() {
    // let x = (1, 2, (), "hello".to_string());
    // let y= (x.0, x.1, x.2, x.3);
    // println!("{:?}, {:?}", x, y);

    //
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    let user2 = User {
        active: false,
        username: String::from("someusername456"),
        email: String::from("another@example.com"),
        sign_in_count: 10
    };
    // println!("{}", user1.active);
    // // // 下面这行会报错
    // println!("{:?}", user1);


    let arr: [User; 2] = [user1, user2];
    for u in 0..= arr.len() - 1 {
        println!("{}", arr[u].email);
    }

    for i in arr {
        println!("{}", i.email);
    }

}
