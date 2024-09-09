fn median(mut list:Vec<f32>) -> Option<f32> {
    if list.is_empty() {
        return None;
    }

    list.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let length: usize = list.len();
    let middle = length / 2;

    let med: f32 = if length % 2 == 0 {
       (list[middle - 1] + list[middle]) / 2.0
    } else {
       list[middle]
    };

    Some(med)
}

fn main() {
    let list = vec![1.0, 4.0, 5.0];
    assert_eq!(median(list), Some(4.0));

    let list = vec![5.0, 1.0, 2.0, 4.0];
    assert_eq!(median(list), Some(3.0));

    let list = vec![];
    assert_eq!(median(list), None);
}