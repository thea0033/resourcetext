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
pub fn to_result<T, U>(o: Option<T>, e: U) -> Result<T, U> {
    if let Some(val) = o {
        Ok(val)
    } else {
        Err(e)
    }
}
/// Takes a result and a function that converts a possible error variant into something new.
/// Useful whenever you want to change Result<usize, io::Error> to Result<usize, String> or something.
pub fn result_compat<T, U, V, P>(o: Result<T, U>, mut e: P) -> Result<T, V>
where
    P: FnMut(U) -> V, {
    match o {
        Ok(val) => Ok(val),
        Err(val) => Err(e(val)),
    }
}
