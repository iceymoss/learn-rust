enum Message {
    Hello { id: i32 },
}

fn main() {
    let msg = Message::Hello { id: 10 };

    match msg {
        Message::Hello { id: id_variable @ 3..=7 } => {
            println!("Found an id in range: {}", id_variable)
        },
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another1 range")
        },
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        },
    }

}