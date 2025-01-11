fn main() {
    crate::inner::my_fun();
}

mod inner {
    ///
    ///
    /// # example
    ///
    /// ```
    /// crate::inner::my_fun();
    /// ```
    pub fn my_fun() {
        println!("i am test in doc");
    }
}
