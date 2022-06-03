use std::mem;

fn type_name_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}

fn main() {
    dbg!(type_name_of(2));
    dbg!(type_name_of(268.2111));

    dbg!(mem::size_of::<isize>());
}
