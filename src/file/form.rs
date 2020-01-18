//! Form sub-module.

/// Concisely create an input form.
#[macro_export]
macro_rules! form {
    ($name:ident, $($field:ident : $type:ty); *) => {
        #[derive(Debug, serde::Deserialize, serde::Serialize)]
        pub struct $name {
            $( pub $field: $type, )*
        }

        impl arc::file::io::Save for $name {
            fn save(&self, path: &std::path::Path) {
                arc::file::io::as_json(self, path);
            }
        }

        impl arc::file::io::Load for $name {
            fn load(path: &std::path::Path) -> Self {
                arc::file::io::from_json(path)
            }
        }
    };
}
