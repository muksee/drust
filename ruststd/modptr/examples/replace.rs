#![allow(unused)]
fn main() {}

#[cfg(test)]
mod tests {
    use std::ptr;

    #[test]
    fn test_replace() {
        let mut rust = ['r', 'u', 's', 't'];
        let b = unsafe { ptr::replace(&mut rust[0], 'b') };
        assert_eq!(b, 'r');
        assert_eq!(rust, ['b', 'u', 's', 't']);
    }
}
