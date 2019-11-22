#[doc(hidden)]
#[macro_export]
macro_rules! package_name {
    () => {
        env!("CARGO_PKG_NAME")
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! package_version {
    () => {
        env!("CARGO_PKG_VERSION")
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! package {
    () => {
        std::concat!(package_name!(), " v", package_version!())
    };
}
