mod list;

use crate::list::List;

fn check_fold_id() {
    let result = List::from_iter(0..10)
        .fold_left(List::empty(), |b, a| List::cons(a, b))
        .reverse();
    assert_eq!(List::from_iter(0..10), result)
}

fn check_fold_right_id() {
    let result = List::from_iter(0..10)
        .fold_right(List::empty(), |a, b| List::cons(a, b));
    assert_eq!(List::from_iter(0..10), result)
}

fn check_fold_sum() {
    let result = List::from_iter(0..10)
        .fold_left(0, |b, a| a + b);
    assert_eq!(45, result)
}

fn check_fold_right_sum() {
    let result = List::from_iter(0..10)
        .fold_right(0, |a, b| a + b);
    assert_eq!(45, result)
}

fn check_map_id() {
    let result = List::from_iter(0..10).map(|a| a);
    assert_eq!(List::from_iter(0..10), result)
}

fn check_map_add() {
    let result = List::from_iter(0..10).map(|a| a + 1);
    assert_eq!(List::from_iter(1..11), result)
}

fn check_flat_map_id() {
    let result = List::from_iter(0..10)
        .flat_map(|a| List::cons(a, List::empty()));
    assert_eq!(List::from_iter(0..10), result)
}

fn check_flat_map_add() {
    let result = List::from_iter(0..10)
        .flat_map(|a| List::cons(a + 1, List::empty()));
    assert_eq!(List::from_iter(1..11), result)
}

fn check_reverse() {
    let result = List::from_iter(0..10).reverse();
    assert_eq!(List::from_iter((0..10).rev()), result)
}

fn check_head_option() {
    let result = List::from_iter(0..10).head_option();
    assert_eq!(0, result.unwrap())
}

fn check_tail() {
    let result = List::from_iter(0..10).tail();
    assert_eq!(List::from_iter(1..10), result)
}

fn check_append() {
    let result = List::from_iter(0..10).append(10);
    assert_eq!(List::from_iter(0..11), result)
}

fn check_prepend_list() {
    let result = List::from_iter(10..20)
        .prepend_list(List::from_iter(0..10));
    assert_eq!(List::from_iter(0..20), result)
}

fn check_filter() {
    let result = List::from_iter(0..10)
        .filter(|n| n % 2 == 0);
    assert_eq!(List::from_iter((0..10).step_by(2)), result)
}

fn main() {
    check_fold_id();
    check_fold_sum();
    check_map_id();
    check_map_add();
    check_flat_map_id();
    check_flat_map_add();
    check_reverse();
    check_head_option();
    check_tail();
    check_fold_right_id();
    check_fold_right_sum();
    check_append();
    check_prepend_list();
    check_filter();
}
