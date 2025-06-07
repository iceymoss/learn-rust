// 可增长字符串结构体
struct TextBuffer {
    content: String,
    version: u32,
}

impl TextBuffer {
    // 可变借用函数：添加文本
    fn append(&mut self, text: &str) {
        self.content.push_str(text);
        self.version += 1;
    }

    // 可变借用函数：清除内容
    fn clear(&mut self) {
        self.content.clear();
        self.version = 0;
    }

    // 不可变借用函数：获取内容
    fn get_content(&self) -> &str {
        &self.content
    }

    // 获取版本信息: 获取版本号
    fn get_version(&self) -> u32 {
        self.version //u32内部实现了Copy的trait在栈是直接copy的
    }
}

fn main() {
    let mut buffer = TextBuffer {
        content: String::new(),
        version: 1,
    };

    buffer.append("Hello");
    buffer.append(", world!");

    println!("Content: {}, version: {}", buffer.get_content(), buffer.get_version());

    buffer.clear();
    println!("After clear: {}", buffer.get_content()); // ""
}