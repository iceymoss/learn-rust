use chrono::Local;

pub fn say_time () -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>> {
    Box::pin(async {
        println!("执行任务!");
        let now = Local::now().timestamp_millis();
        println!("time:{}", now);
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    })
}