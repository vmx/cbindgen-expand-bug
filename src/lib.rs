macro_rules! some_macro {
    () => {
        #[no_mangle]
        pub unsafe extern "C" fn hello() {}
    };
}

some_macro!();
