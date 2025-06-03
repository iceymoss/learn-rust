use tokio::fs::File;
use tokio::io::AsyncWriteExt;     // 引入AsyncWriteExt trait

pub async fn write() -> std::io::Result<()> {
    let mut file = File::create("foo.txt").await.unwrap();  // 创建文件
    file.write_all(b"hello, world!").await.unwrap();        // 写入内容
    Ok(())
}