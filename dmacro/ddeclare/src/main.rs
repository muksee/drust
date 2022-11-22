macro_rules! make_public {
    (
        $(#[$meta:meta])*
        $vis:vis struct $struct_name: ident {
            $(#[$field_meta:meta])*
            $($field_vis:vis $field_name:ident : $field_type:ty),*$(,)+
        }
    ) => {
        $(#[$meta])*
        pub struct $struct_name {
            $(#[$field_meta])*
            $(pub $field_name: $field_type,)*
        }
    };
}

macro_rules! sum {
    ($x: expr, $y: expr) => {
        $x + $y
    };
}

fn main() {
    make_public!(
        #[derive(Debug)]
        struct Person {
            name: String,
            age: u64,
            addr: String,
        }
    );

    sum!(2, 3);
}
