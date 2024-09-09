fn main() {
    let mut first_list = vec!["Todd", "amy"];
    sort_inplace(&mut first_list);
    assert_eq!(first_list, vec!["amy", "Todd"]);

    let mut second_list = vec!["Jax", "britney", "yan"];
    sort_inplace(&mut second_list);
    assert_eq!(second_list, vec!["britney", "Jax", "yan"]);
}

fn sort_inplace(list: &mut Vec<&str>) {
    list.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
}