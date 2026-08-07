pub fn reorder_from_center(items: &[usize]) -> Vec<usize> {
    let len = items.len();
    if len <= 1 {
        return items.to_vec();
    }
    let mid = len / 2;
    let left: Vec<usize> = items[..mid].iter().rev().copied().collect();  // [1, 0]
    let right: Vec<usize> = items[mid..].iter().copied().collect();       // [2, 3]

    let mut result = Vec::with_capacity(len);
    result.extend(right);
    result.extend(left);
    result
}