fn add_one(i: i32) -> i32 {
    i + 1
}

fn times_two(i: i32) -> i32 {
    i * 2
}

fn do_this(i: i32, f: fn(i32) -> i32) -> i32 {
    f(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_function_pointers() {
        let mut v = 1;

        v = add_one(v);
        assert_eq!(v, 2);

        v = times_two(v);
        assert_eq!(v, 4);

        v = do_this(v, add_one);
        assert_eq!(v, 5);

        v = do_this(v, times_two);
        assert_eq!(v, 10);

        v = do_this(v, |i| i * 10); // using fn we can also pass a closure
        assert_eq!(v, 100);
    }
}
