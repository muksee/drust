fn main() {}

#[cfg(test)]
mod tests {
    use std::ptr;

    #[test]
    fn test_slice_from_raw_parts() {
        let x = &mut [5, 6, 7];
        let raw_pointer = x.as_mut_ptr();
        let len = x.len();

        let slice = unsafe {
            let s = ptr::slice_from_raw_parts(raw_pointer, len);
            &*s
        };

        assert_eq!(slice[0], 5);
    }

    #[test]
    fn test_slice_from_raw_parts_mut() {
        let mut x = &mut [5, 6, 7];
        let raw_pointer = x.as_mut_ptr();
        let len = x.len();

        let slice = unsafe {
            let s = ptr::slice_from_raw_parts_mut(raw_pointer, len);
            &mut *s
        };
        slice[0] = 10;
        assert_eq!(slice[0], 10);
    }
}
