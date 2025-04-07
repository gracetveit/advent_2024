/// # Problem
/// ## Example input
/// ```
/// 3   4
/// 4   3
/// 2   5
/// 1   3
/// 3   9
/// 3   3
/// ```
/// Pair up the numbers and measure how far apart they are.
/// Pair up the *smallest number in the left list*  with the *smallest number in
/// the right list*, then the *second-smallest left number* with the *second-
/// smallest right number*, and so on.
/// 
/// With each pair, figure out *how far apart* the two numnbers are; you'll need
/// to *add up all of those distances.* For example, if you pair up a `3` from
/// the left list with a `7` from the right list, the distance apart is `4`;
/// if you pair up a `9` with a `3`, the distance apart is `6`.
/// 
/// # Approach
/// First, we need to read the input file. Using the `input_reader.rs`, we can
/// get a raw string. We can split the string by `\n`, and then seperately add
/// the first and last item from the sub-string to two seperate vectors, and 
/// then return those vectors.
/// 
/// Next we need to sort the vectors from smallest to largest, using one of
/// Rust's buit-in tools.
/// 
/// After we've sorted the vectors, maybe we can `reduce` them. First
/// finding the absolute value between the two, and then adding that to a
/// rolling sum.
pub fn day_01_solution(filename: &str) -> i32 {
    let (mut vec_a, mut vec_b) = day_01_parse(filename);
    vec_a.sort();
    vec_b.sort();
    let mut i = 0;
    let mut sum = 0;
    while i < vec_a.len() {
        sum = multi_reduce(vec_a[i], vec_b[i], sum);
        i += 1;
    }
    return sum;
}

fn day_01_parse(filename: &str) -> (Vec<i32>, Vec<i32>) {
    todo!()
}

#[test]
fn test_parse() {
    use super::solution::day_01_parse;
    let list_a = vec![3, 4, 2, 1, 3, 3];
    let list_b = vec![4, 3, 5, 3, 9, 3];

    let (input_list_a, input_list_b) = day_01_parse("inputs/day_01_test.txt");
    assert_eq!(list_a, input_list_a);
    assert_eq!(list_b, input_list_b);
}

fn multi_reduce(a: i32, b: i32, sum: i32) -> i32 {
    todo!()
}

#[test]
fn test_multi_reduce() {
    assert_eq!(multi_reduce(1, 2, 0), 1);
}