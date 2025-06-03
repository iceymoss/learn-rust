trait Descriptive {
    fn describe(&self) -> String;
}

struct Person {
    name: String,
    age: u8,
}

impl Descriptive for Person {
    fn describe(&self) -> String {
        format!("{} is {} years old.", self.name, self.age)
    }
}

struct Dog {
    name: String,
    cooler: String,
    age: u8,
}

impl Descriptive for Dog {
    fn describe(&self) -> String {
        format!("{} is {} years old, it cooler is {}", self.name, self.age, self.cooler)
    }
}


fn output(object: impl Descriptive) {
    println!("{}", object.describe());
}

fn output_two<T: Descriptive>(arg1: T, arg2: T) {
    println!("{}", arg1.describe());
    println!("{}", arg2.describe());
}

fn main() {
    let p = Person {
        name: String::from("Ubuntu"),
        age: 18,
    };
    let s = p.describe();
    println!("{}", s);
    println!("name:{}", p.name);

    let dog = Dog {
        name: String::from("xiaohei"),
        cooler: String::from("blue"),
        age: 1,
    };

    output(dog);

    output_two(dog, dog);
}