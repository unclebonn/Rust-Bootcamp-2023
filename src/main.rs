fn reverse_collection<T>(collection: &mut[T]){
    collection.reverse();
}

fn main() {
    let mut arr_number = vec![2, 11, 3, 100, 5, 62];
    // let result = arr_number.reverse();
     let result = reverse_collection(&mut arr_number);
    println!("{:?}", result);
}
