use std::io;

fn exercise6() {
    // let mut prev_key = "";

    // for line in io::stdin().lines() {
    //     let s = line.unwrap();
    //     let data: Vec<&str> = s.split("\t").collect();
    //     if prev_key.len() == 0 {
    //         prev_key = data[0];
    //     }
    // }
}

fn main() {
    let mut accounting = vec!["Alice", "Ben"];
    let mut count = 0;
    
    loop {
        let mut add_input_ptr = "anhkhoi".as_ptr();
        let mut add_input = "anhkhoi";
        println!("pointer lan thu {}:{:?}",{ count += 1; count },add_input_ptr);
        accounting.push(add_input);
        if accounting.len() >= 10 {
            break;
        }
    }

    println!("{:?}",accounting);
}
