#[macro_use]
extern crate cake;

build! {
    start(sodium, libstd) => cmd!("ls"),
    sodium(libstd, libextra) => println!("yay"),
    libstd() => println!("libstd"),
    libextra() => println!("libextra"),
}
