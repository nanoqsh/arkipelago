macro_rules! export {
    { $( $name:ident ),* $( , )? } => { $(
        mod $name;
        pub(crate) use $name::src as $name;
    )* };
}

pub(crate) use export;
