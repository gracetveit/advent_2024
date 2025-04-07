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
pub fn sol() {
    println!("Hello World!");
}