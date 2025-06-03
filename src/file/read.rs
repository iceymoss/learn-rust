use tokio::fs::File;
use tokio::io::AsyncReadExt;   // 引入AsyncReadExt trait

pub async fn read() -> std::io::Result<()> {
    let mut file = File::open("foo.txt").await.unwrap();  // 打开文件
    let mut contents = vec![];
    // 将文件内容读到contents动态数组里面，注意传入的是可变引用
    file.read_to_end(&mut contents).await.unwrap();
    println!("len = {}", contents.len());
    Ok(())
}
