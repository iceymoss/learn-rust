

fn g(i: i32) -> Result<i32, bool> {
    let t = check(i);
    return match t {
        Ok(i) => Ok(i),
        Err(b) => Err(b)
    };
}
fn check(i: i32) -> Result<i32, bool> {
    if i >= 0 {
        Ok(i)
    } else {
        Err(false)
    }
}

fn main() {
    let r = g(-1);
    if let Ok(v) = r {
        println!("Ok: f(-1) = {}", v);
    } else {
        println!("Err");
    }
}