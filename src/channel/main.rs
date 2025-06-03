use std::time::Duration;
use tokio::sync::{mpsc, oneshot};
use tokio::task;
use tokio::task::futures;
use tokio::time;

#[tokio::main]
async fn main() {
    let mut db: Vec<u32> = vec![1,2,3,4,5,6,7,8,9,10];
    
    // 创建channel，创建一个tx用于一个或者多个生产者push传输数据，rx用于一个消费者消费数据
    let (tx, mut rx) = mpsc::channel::<(u32, oneshot::Sender<bool>)>(100);

    // 这里clone的本质是进行引用计数
    let tx1 = tx.clone();
    let tx2 = tx.clone();

    // task::spawn 这里就已经将task放入runtime中开始执行了
    let task_a = task::spawn(async move {
        // 休息三秒
        time::sleep(Duration::from_secs(3)).await;
        
        // 创建一个用于同步状态的channel
        let (resp_tx, resp_rx) = oneshot::channel();
        
        // 将resp_tx的push生产权利交给tx的消费者
        if let Err(_) = tx1.send((50, resp_tx)).await {
            println!("receiver dropped");
            return;
        }
        
        // 监控tx的消费者是否完成了
        if let Ok(ret) = resp_rx.await {
            if ret {
                println!("task_a finished with success.");
            } else {
                println!("task_a finished with failure.");
            }
        } else {
            println!("oneshot sender dropped");
            return;
        }
    });
    let task_b = task::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        if let Err(_) = tx2.send((100, resp_tx)).await {
            println!("receiver dropped");
            return;
        }
        if let Ok(ret) = resp_rx.await {
            if ret {
                println!("task_b finished with success.");
            } else {
                println!("task_b finished with failure.");
            }
        } else {
            println!("oneshot sender dropped");
            return;
        }
    });

    let task_c = task::spawn(async move {
        while let Some((i, resp_tx)) = rx.recv().await {
            println!("got = {}", i);
            db[4] = i;
            println!("{:?}", db);
            
            // 处理完成，将消息状态同步给生产者
            resp_tx.send(true).unwrap();
        }
    });

    _ = task_a.await.unwrap();
    _ = task_b.await.unwrap();
    _ = task_c.await.unwrap();
}