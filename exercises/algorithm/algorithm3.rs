/*
    sort
    This problem requires you to implement a sorting algorithm
    you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T: std::marker::Copy + std::cmp::PartialOrd>(array: &mut [T]) {
    //TODO
    if array.len() < 2 {
        return;
    }
    let length = array.len();
    let mid = array.len() / 2;
    let (array_left, array_right) = array.split_at_mut(mid);

    sort(array_left);
    sort(array_right);

    let mut merged = Vec::with_capacity(length);
    let mut index_left = 0;
    let mut index_right = 0;
    while index_left < array_left.len() && index_right < array_right.len() {
        if array_left[index_left] > array_right[index_right] {
            merged.push(array_right[index_right]);
            index_right += 1;
        } else {
            merged.push(array_left[index_left]);
            index_left += 1;
        }
    }
    while index_left < array_left.len() {
        merged.push(array_left[index_left]);
        index_left += 1;
    }
    while index_right < array_right.len() {
        merged.push(array_right[index_right]);
        index_right += 1;
    }
    array.copy_from_slice(&merged);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
    #[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
    #[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}
