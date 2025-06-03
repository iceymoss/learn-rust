
mod nation {                  // 定义模块 nation
    pub mod government {      // 子模块 government（公开）
        pub fn govern() {}    // 公开函数
    }

    mod congress {            // 子模块 congress（私有）
        pub fn legislate() {} // 函数虽标记为 pub，但受模块可见性限制
    }

    mod court {               // 子模块 court（私有）
        fn judicial() {       // 私有函数
            super::congress::legislate(); // 调用兄弟模块的函数
        }
    }
}

fn main() {
    nation::government::govern();
    nation::congress::legislate();
}