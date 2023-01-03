fn main() {
    let c = Closure {
        data: (100, 200),
        f: largest,
    };

    let r = c.call();

    println!("{r}");
}

struct Closure<F> {
    data: (i32, i32),
    f: F,
}

impl<F> Closure<F>
where
    for<'a> F: Fn(&'a (i32, i32)) -> &'a i32,
{
    fn call(&self) -> &i32 {
        (self.f)(&self.data)
    }
}

fn largest<'a>(data: &'a (i32, i32)) -> &'a i32 {
    if data.0 >= data.1 {
        &data.0
    } else {
        &data.1
    }
}
