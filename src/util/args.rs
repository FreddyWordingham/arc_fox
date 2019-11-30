//! Args macro.

/// Import command line arguments as a requested type.
#[macro_export]
macro_rules! args {
    ($( $name:ident : $type:ty ); +) => {
        $(let $name;)*
        {
            info!("Command line arguments:");
            let args: Vec<String> = std::env::args().collect();
            let mut args_iter = args.iter();
            $(
                $name = (*args_iter.next().expect(
                    &format!("Command line argument <{}> missing.", stringify!($name)))).parse::<$type>().expect(
                    &format!("Unable to parse <{}> into {}.", stringify!($name), stringify!($type))
                );
                arc::report!($name);
            )*
        }
    };
}
