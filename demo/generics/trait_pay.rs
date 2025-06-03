use std::time::SystemTime;
use std::time::UNIX_EPOCH;


trait Payer {
    fn pay(&mut self, address: String, count: f64) -> Result<String, Error>;
}

#[derive(Debug)]
enum Error {
    EmptyAmount,
    ToAddressBaned,
    InsufficientBalance,
    TransferAmountIllegal,
    OtherErr
}

impl Error {
    fn print_err(&self) {
        match self {
            Error::EmptyAmount => {
                println!("EmptyAmount");
            }
            Error::ToAddressBaned => {
                println!("To address banked");
            }
            Error::InsufficientBalance => {
                println!("InsufficientBalance");
            }
            Error::TransferAmountIllegal => {
                println!("TransferAmountIllegal");
            }
            _ => {
                println!("Error: {:?}", self);
            }
        }
    }
}

#[warn(unused_parens)]
struct AliPay {
    amount: f64,
    address: String,
    last_tx_time: u64,
    fees: f64
}

impl AliPay {
    fn new(amount: f64, address: String) -> AliPay {
        AliPay {
            fees: 0.01,
            amount,
            address,
            last_tx_time: SystemTime::now().duration_since(UNIX_EPOCH).expect("系统时间早于 1970-01-01，这几乎不可能发生").as_secs(),
        }
    }
}

impl Payer for AliPay {
    fn pay(&mut self, address: String, count: f64) -> Result<String, Error> {
        println!("====================欢迎使用AliPay=====================");
        println!("==========={}====>{}===========", self.address, address);
        if count <= 0 as f64 {
            return Err(Error::TransferAmountIllegal);
        }

        let fees = self.amount * self.fees;
        let total = self.amount - (fees + count);

        if total < 0 as f64 {
            return Err(Error::InsufficientBalance);
        }

        let reduce_total = fees + count as f64;
        self.amount -= reduce_total;
        self.last_tx_time =SystemTime::now().duration_since(UNIX_EPOCH).expect("系统时间早于 1970-01-01，这几乎不可能发生").as_secs();

        //todo: 做转账处理
        let tx_id= "tx-alibaba".to_string() + self.last_tx_time.to_string().as_str();

        Ok(tx_id)
    }
}

struct TencentPay {
    amount: f64,
    address: String,
    last_tx_time: u64,
    fees: f64
}

impl TencentPay {
    fn new(amount: f64, address: String) -> TencentPay {
        TencentPay {
            amount,
            address,
            fees: 0.05,
            last_tx_time: SystemTime::now().duration_since(UNIX_EPOCH).expect("系统时间早于 1970-01-01，这几乎不可能发生").as_secs(),
        }
    }

}

impl Payer for TencentPay {
    fn pay(&mut self, address: String, count: f64) -> Result<String, Error> {
        println!("====================TencentPay====================");
        println!("==========={}====>{}===========", self.address, address);
        if count <= 0 as f64 {
            return Err(Error::TransferAmountIllegal);
        }

        let fees = self.amount * self.fees;
        let total = self.amount - (fees + count);

        if total < 0 as f64 {
            return Err(Error::InsufficientBalance);
        }

        let reduce_total = fees + count as f64;
        self.amount -= reduce_total;
        self.last_tx_time =SystemTime::now().duration_since(UNIX_EPOCH).expect("系统时间早于 1970-01-01，这几乎不可能发生").as_secs();

        //todo: 做转账处理

        let tx_id= "tx-tencent".to_string() + self.last_tx_time.to_string().as_str();

        Ok(tx_id)
    }
}

// fn process_payment<T: Payer>(payer: &mut T, address: &str, amount: f64) -> Result<String, Error> {
//     payer.pay(address.to_string(), amount)
// }

// 修改为 Trait 对象版本 ✅
fn process_payment(payer: &mut dyn Payer, address: &str, amount: f64) -> Result<String, Error> {
    payer.pay(address.to_string(), amount)
}

fn main() {
    // let mut ali_user = AliPay::new(100 as f64, String::from("12RBASGASBFSA"));
    // match ali_user.pay("sadfisahfisf".to_string(), 50.0) {
    //     Ok(tx_id) => {
    //         println!("ok tx_id: {}", tx_id);
    //     }
    //     Err(e) => {
    //         e.print_err();
    //     }
    // }
    //
    // println!("ali_user:{}", ali_user.amount);
    //
    // let mut ten_user = TencentPay::new(100 as f64, String::from("tx-dashdfiashfd"));
    // match ten_user.pay("sadfisahfisf".to_string(), 50.0) {
    //     Ok(tx_id) => {
    //         println!("ok tx_id: {}", tx_id);
    //     }
    //     Err(e) => {
    //         e.print_err();
    //     }
    // }
    //
    // println!("ten_user:{}", ten_user.amount);

    let mut ali_user1 = AliPay::new(100 as f64, String::from("12RBASGASBFSA"));
    let mut ten_user1 = TencentPay::new(100 as f64, String::from("tx-dashdfiashfd"));
    // 创建 trait 对象集合
    let mut payers: Vec<&mut dyn Payer> = vec![&mut ali_user1, &mut ten_user1];
    for pay in payers {
        // 使用trait + 泛型
        process_payment(pay, "sadfisahfisf", 50.0).unwrap();
    }

    println!("tencent：{}", ten_user1.amount);
    println!("ali：{}", ali_user1.amount);
}
