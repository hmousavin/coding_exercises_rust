// fn get_uniques(mut list:Vec<u32>) -> Vec<u32> {
//     if list.is_empty() {
//         return vec![];
//     }
    
//     list.dedup();
//     list
// }

fn get_uniques<T: PartialEq>(mut list:Vec<T>) -> Vec<T> {
    if list.is_empty() {
        return vec![];
    }
    
    list.dedup(); 
    list
}

fn main() {
    let list: Vec<u32> = vec![];
    assert_eq!(get_uniques(list), vec![]);
    
    let list = vec![1, 1, 3];
    assert_eq!(get_uniques(list), vec![1,3]);

    let list = vec![1, 2, 3, 4, 4, 4];
    assert_eq!(get_uniques(list), vec![1, 2, 3, 4]);

    // let list = vec!['a', 'b', 'c', 'b'];
    // assert_eq!(get_uniques(list), vec!['a', 'b', 'c']);

    let list = vec![1.4, 2.1, 3.8, 4.5, 4.8, 2.1];
    assert_eq!(get_uniques(list), vec![1.4, 2.1, 3.8, 4.5, 4.8]);
}

// extra credit:
// 1. use generics
// 2. retain the order ()