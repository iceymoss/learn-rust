pub trait Sport {
    type SportType;

    fn play(&self,  st: Self::SportType);
}

struct Football;
pub enum SportType {
    Land,
    Water,
}

impl Sport for Football {
    type SportType = SportType;  // 这里故意取相同的名字，不同的名字也是可以的
    fn play(&self,  st: Self::SportType){}  // 方法中用到了关联类型
}

struct PingPong;

pub enum SportType1 {
    SmallBall,
    Double
}

impl Sport for PingPong {
    type SportType = SportType1;
    fn play(&self, st: Self::SportType) {
    }
}


fn main() {
    let f = Football;
    f.play(SportType::Land);
}