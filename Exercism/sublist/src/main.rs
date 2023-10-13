#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list == second_list {
        return Comparison::Equal;
    }

    // Check if first_list is a sublist of second_list
    if is_sublist(first_list, second_list) {
        return Comparison::Sublist;
    }

    // Check if second_list is a sublist of first_list
    if is_sublist(second_list, first_list) {
        return Comparison::Superlist;
    }

    Comparison::Unequal
}

fn is_sublist<T: PartialEq>(smaller: &[T], larger: &[T]) -> bool {
    if smaller.is_empty() {
        return true;
    }

    // Iterate through 'larger' and check if any contiguous slice matches 'smaller'
    for window in larger.windows(smaller.len()) {
        if window == smaller {
            return true;
        }
    }

    false
}

fn main() {
    assert_eq!(sublist(&[1, 2, 3], &[1, 2, 3, 4]), Comparison::Sublist);
    assert_eq!(sublist(&[1, 2, 3, 4], &[1, 2, 3]), Comparison::Superlist);
    assert_eq!(sublist(&[1, 2, 3], &[1, 2, 3]), Comparison::Equal);
    assert_eq!(sublist(&[1, 2, 3], &[4, 5, 6]), Comparison::Unequal);
    println!("All tests passed!");
}