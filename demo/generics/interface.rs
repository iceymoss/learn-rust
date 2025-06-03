use std::collections::HashMap;

// 定义 Trade trait
trait Trade {
    fn sell(&mut self, address: &str, count: f64, coin: &str) -> bool;
    fn buy(&mut self, address: &str, count: f64, coin: &str) -> bool;
}

// 结构体需要生命周期标注（因为包含引用字段）
struct Okx<'a> {
    support_coin: [&'a str; 3],
    coin_count: HashMap<&'a str, f64>,
    fee: HashMap<&'a str, f64>,
}

impl<'a> Okx<'a> {
    // 初始化 Okx 结构体
    fn new() -> Self {
        let support_coin = ["btc", "eth", "sol"];
        let mut coin_count = HashMap::new();
        let mut fee = HashMap::new();

        // 初始化币种数量和手续费率
        for &coin in &support_coin {
            coin_count.insert(coin, 1000);
            fee.insert(coin, match coin {
                "btc" => 0.0002,
                "eth" => 0.001,
                "sol" => 0.001,
                _ => 0.0,
            });
        }

        Okx {
            support_coin,
            coin_count,
            fee,
        }
    }
}

// 实现 Trade trait
impl<'a> Trade for Okx<'a> {
    fn sell(&mut self, _address: &str, count: f64, coin: &str) -> bool {
        // 检查是否支持该币种
        if !self.support_coin.contains(&coin) {
            return false;
        }

        // 获取当前币种余额和手续费率
        let balance = self.coin_count.get_mut(coin).unwrap();
        let fee_rate = self.fee.get(coin).unwrap();

        // 检查余额是否足够
        if *balance < count {
            return false;
        }

        // 扣除手续费后的实际卖出数量
        let actual_sell = count * (1.0 - fee_rate);
        *balance -= actual_sell;

        true
    }

    fn buy(&mut self, _address: &str, count: f64, coin: &str) -> bool {
        // 检查是否支持该币种
        if !self.support_coin.contains(&coin) {
            return false;
        }

        // 获取当前币种余额和手续费率
        let balance = self.coin_count.get_mut(coin).unwrap();
        let fee_rate = self.fee.get(coin).unwrap();

        // 扣除手续费后的实际买入数量
        let actual_buy = count * (1.0 - fee_rate);
        *balance += actual_buy;

        true
    }
}

// 测试代码
fn main() {
    let mut okx = Okx::new();

    // 初始余额为 0，无法卖出
    assert!(!okx.sell("user1", 1.0, "btc"));

    // 买入 BTC
    assert!(okx.buy("user1", 100.0, "btc"));
    println!("BTC余额: {:?}", okx.coin_count.get("btc")); // 输出 99.98（扣除手续费）

    // 卖出 BTC
    assert!(okx.sell("user1", 50.0, "btc"));
    println!("BTC余额: {:?}", okx.coin_count.get("btc")); // 输出约 49.98
}