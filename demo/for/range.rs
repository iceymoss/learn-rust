
fn main() {
    // for i in 1..=1000 {
    //     if i > 101 {
    //         break;
    //     }
    //     println!("{}", i);
    // }
    //
    // for j in 'a'..='z' {
    //     if j == 'y' {
    //         break;
    //     }
    //     println!("{}", j);
    // }


    // 转移所有权
    let mut list = vec![10, 2, 111, 34, 12, 43];

    {
        for i in &list { // 不可变借用
            println!("{}", i);
        }

        // let l = list;
    }

    {
        for i in &mut list { // 可变借用
            if *i == 2 {
                *i = 1000
            }
        }
    }

    let list1 = list.clone();
    let mut list2 = list.clone();

    {
        for i in list { // 会移动所有权
            println!("{}", i);
        }
    }

    // let l = list;


    let res_list = select_sort(list1);
    println!("{:?}", res_list);

    select_sort1(&mut list2);
    println!("{:?}", list2);


}

// select_sort 所有权移动的写法
fn select_sort(mut list: Vec<i32>) -> Vec<i32> {
    for i in 0..=list.len() - 1 {
        for j in i+1..=list.len() - 1 {
            if list[j] > list[i] {
                let temp = list[i];
                list[i] = list[j];
                list[j] = temp;
            }
        }
    }
    list.clone()
}

// 可变引用写法
fn select_sort1(list: &mut Vec<i32>) {
    for i in 0..= list.len() -1 {
        for j in i+1..=list.len() - 1 {
            if list[j] > list[i] {
                let temp = list[i];
                list[i] = list[j];
                list[j] = temp;
            }
        }
    }
}