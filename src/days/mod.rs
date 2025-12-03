macro_rules! pub_mod {
    ($name:ident) => {
        pub mod $name;
        pub use $name::*;
    };
}

pub_mod!(day01);
pub_mod!(day02);
pub_mod!(day03);