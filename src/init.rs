//! Initialisation functions.

/// Quickly import the command line arguments as their requested type.
#[macro_export]
macro_rules! args {
    ($($name:ident : $type:ty), +) => {
        let args: Vec<String> = std::env::args().collect();
        let mut args_iter = args.iter();
        $(
            let $name = (*args_iter.next().unwrap()).parse::<$type>().expect(
                &format!("Unable to parse <{}> into {}.", stringify!($name), stringify!($type))
            );
        )*
    };
}
