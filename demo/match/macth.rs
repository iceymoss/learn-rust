
enum Animal{
    Cat,
    Pig,
    Dog,
}

enum IpAddress {
    Ipv4,
    Ipv6,
}

enum Info {
    Name(String),
    Author(String),
    Date(String),
}

enum Coin {
    Btc(Info),
    Eth(Info)
}

enum Action {
    Say(String),
    MoveTo(i32, i32),
    ChangeColorRGB(u16, u16, u16),
}

fn get_action() {
    let actions = [
        Action::Say("Hello Rust".to_string()),
        Action::MoveTo(1,2),
        Action::ChangeColorRGB(255,255,0),
    ];
    for action in actions {
        match action {
            Action::Say(s) => {
                println!("{}", s);
            },
            Action::MoveTo(x, y) => {
                println!("point from (0, 0) move to ({}, {})", x, y);
            },
            Action::ChangeColorRGB(r, g, _) => {
                println!("change color into '(r:{}, g:{}, b:0)', 'b' has been ignored",
                         r, g,
                );
            }
        }
    }
}

fn get_coin_info(coin: &Coin) -> String {
    match coin {
        Coin::Btc(info) => {
            match info {
                // 模式匹配的另外一个重要功能是从模式中取出绑定的值
                Info::Name(_) => "BTC".to_string(),
                Info::Author(_) => "中本聪".to_string(),
                _ => "只提供名称和作者".to_string()
            }
        }
        Coin::Eth(info) => {
                match info {
                    Info::Name(_) => "ETH".to_string(),
                    Info::Author(_) => "V神".to_string(),
                    _ => "只提供名称和作者".to_string()
                }
            }
        _ => "未知币种".to_string()
    }
}


fn get_default_ip_address(ip: IpAddress) -> String {
    // 使用 match 表达式赋值
    let default_ip = match ip {
        IpAddress::Ipv4 => "127.0.0.1",
        IpAddress::Ipv6 => "::ffff:127.0.0.1",
        _ => ":::1",
    };
    return default_ip.to_string();
}

#[derive(Debug)]
enum Direction {
    East,
    West,
    North,
    South,
}

fn get_dire() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        //除了_通配符，用一个变量来承载其他情况也是可以的
        other => println!("other direction: {:?}", other),
    };
}

// get_animal_index 不可变借用
fn get_animal_index(animal: &Animal) -> i32 {
    match animal {
        Animal::Cat => {
            0
        }
        Animal::Dog => {
            1
        }
        Animal::Pig => {
            2
        }
    }
}

// get_animal_index 值移动版本
fn get_animal_index2(animal: Animal) -> i32 {
    match animal {
        Animal::Cat => 0,
        Animal::Dog => {
            return 1
        }
        Animal::Pig => {
            2
        }
    }
}

#[derive(Debug)]
enum MyEnum {
    Foo,
    Bar
}

fn my_enum() {
    // matches 宏 配
    let v = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo];
    v.iter().filter(|x| matches!(x, MyEnum::Foo));

    for i in &v {
        println!("{:?}", i);
    }

    let foo = 'f';
    assert!(matches!(foo, 'A'..='Z' | 'a'..='z'));

    let bar = Some(4);
    assert!(matches!(bar, Some(x) if x > 2));

}



fn main() {
    let am = Animal::Pig;
    match am {
        Animal::Cat => {
            println!("{}", "the is a cat");
        }
        Animal::Dog => {
            println!("{}", "the is a dog");
        }
        // Animal::Pig=> {
        //     println!("{}", "the is a pig");
        // }

        // match必须匹配所有，如果不想全覆盖，使用_ => {}
        // 类似 switch 中的 default
        _ => {
            println!("{}", "unknow");
        }
    }

    let index = get_animal_index(&am);
    println!("{}", index);
    let index2 = get_animal_index2(am);
    println!("{}", index2);

    // println!("{}", get_animal_index2(am));

    let default_ip = get_default_ip_address(IpAddress::Ipv4);
    println!("{}", default_ip);

    let coin = Coin::Btc(Info::Name("BTC".to_string()));
    let info = get_coin_info(&coin);
    println!("{}", info);

    get_action();
    get_dire();

    my_enum()








}