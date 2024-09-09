fn get_type<T>(a: T) {
    // type()
}

fn main() {
    let a = ":p";
    assert_eq!(get_type(a), "string");
    
    let b = 123;
    assert_eq!(get_type(list), vec![1,3]);

    let c = "این میتونه هر چیزی باشه";
    assert_eq!(get_type(list), vec![1, 2, 3, 4]);

    let list = vec![1.4, 2.1, 3.8, 4.5, 4.8, 2.1];
    assert_eq!(get_type(list), vec![1.4, 2.1, 3.8, 4.5, 4.8]);
}