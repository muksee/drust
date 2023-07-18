#[derive(Debug)]
struct My {
    inner: Box<i32>,
}
impl Drop for My {
    fn drop(&mut self) {
        println!("Drop {:p}", self.inner);
    }
}

fn main() {
    {
        let b = Box::new(My {
            inner: Box::new(100i32),
        });
        println!("b: {:?},{:p}", b, b.inner);

        let c = *b;
        println!("c: {:?},{:p}", c, c.inner);

        let d = My {
            inner: Box::new(200i32),
        };
        println!("d: {:?},{:p}", d, d.inner);
    }

    println!("======================");
}
