trait Foo {
    type Bar;
}

fn foo<T: Foo>()
where
    T::Baa: std::fmt::Debug,
    //~^ ERROR associated type `Baa` not found for `T`
{
}

fn main() {}
