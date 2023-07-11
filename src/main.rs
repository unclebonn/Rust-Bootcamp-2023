#[derive(Debug)]

enum MessageOne {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
   
    fn show_message(msg: MessageOne) {
        println!("{:?}", msg);
    }
    
    
        let msgs = [
            MessageOne::Quit,
            MessageOne::Move { x: 1, y: 3 },
            MessageOne::ChangeColor(255, 255, 0),
        ];
    
        for msg in msgs {
            show_message(msg)
        }
    
    
}
