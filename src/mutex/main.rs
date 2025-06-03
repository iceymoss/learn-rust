use std::sync::Arc;
use tokio::sync::Mutex; // 互斥锁
use tokio::sync::RwLock; // 读写锁
use std::sync::atomic::AtomicU32; // 原子操作

#[tokio::main]
async fn main() {
    let db: Vec<u32> = vec![1,2,3,4,5,6,7,8,9,10];
    let arc_db = Arc::new(Mutex::new(db));  // 加锁
    let arc_db2 = arc_db.clone();
    let arc_db3 = arc_db.clone();

    let task_a = tokio::task::spawn(async move {
        let mut db = arc_db.lock().await;  // 获取锁
        db[4] = 50;
        assert_eq!(db[4], 50);             // 校验值
    });
    let task_b = tokio::task::spawn(async move {
        let mut db = arc_db2.lock().await;  // 获取锁
        db[4] = 100;
        assert_eq!(db[4], 100);            // 校验值
    });
    _ = task_a.await.unwrap();
    _ = task_b.await.unwrap();

    println!("{:?}", arc_db3.lock().await);  // 获取锁

    mutex_test();
    atomic_test();
}


async fn mutex_test() {
    let lock = RwLock::new(5);
    // 多个读锁可以同时存在
    {
        let r1 = lock.read().await;
        let r2 = lock.read().await;
        assert_eq!(*r1, 5);
        assert_eq!(*r2, 5);
    } // 在这一句结束时，两个读锁都释放掉了

    // 同时只能存在一个写锁
    {
        let mut w = lock.write().await;
        *w += 1;
        assert_eq!(*w, 6);
    } // 在这一句结束时，写锁释放掉了
}

fn atomic_test() {
    // 创建
    let atomic_forty_two = AtomicU32::new(42);
    let arc_data = Arc::new(atomic_forty_two);

    let mut some_var = AtomicU32::new(10);
    // 更新
    *some_var.get_mut() = 5;
    assert_eq!(*some_var.get_mut(), 5);
}