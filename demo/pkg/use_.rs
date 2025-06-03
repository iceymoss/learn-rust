mod nation {
    pub mod government {
        pub fn govern() {
            println!("Govern");
        }
        pub fn govern_review() {
            println!("Govern_Review");
        }
    }
    pub mod reviewment {
        pub fn review() {
            println!("Review");
        }
    }
    pub fn govern() {
        println!("nation.Govern");
    }

    pub mod count {
        pub fn job_count() {
            println!("job_count");
        }
    }

    // 修复：使用 self:: 指定当前模块的子模块
    pub use self::count::job_count; // ✅ 正确路径
}

use crate::nation::government::govern;
use crate::nation::reviewment::review;
use crate::nation::govern as nation_govern;

fn main() {
    govern();
    review();
    nation_govern();
    nation::job_count();
}