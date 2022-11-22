fn main() {}

#[cfg(test)]
mod tests {
    use std::ptr;

    #[test]
    fn test_swap() {
        let mut x = [1, 2, 3, 4];
        let (x1, x2) = x.split_at_mut(2);
        let x1 = x1.as_mut_ptr().cast::<[u32; 2]>();
        let x2 = x2.as_mut_ptr().cast::<[u32; 2]>();

        unsafe {
            ptr::swap(x1, x2);
        }

        assert_eq!(x, [3, 4, 1, 2]);
    }
    fn test_swap_nonoverlapping() {}
}
