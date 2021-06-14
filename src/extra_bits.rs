///"filters" the input (index), moving it based on the filter.
/// For example:
/// ```
/// assert_eq!(filter(2,&vec![true, true, false, false, true]), 4);
/// assert_eq!(filter(1,&vec![true, true, false, false, true]), 1);
/// let true_vec:Vec<bool> = vec![true; 100];
/// for i in 0..100 {
/// assert_eq!(filter(i, &true_vec), i);
/// }
/// ```
pub fn filter(mut input: usize, filter: &Vec<bool>) -> usize {
    for (i, item) in filter.iter().enumerate() {
        if *item {
            if input == 0 {
                return i;
            }
            input -= 1;
        }
    }
    panic!("The option selected was too high!");
}
