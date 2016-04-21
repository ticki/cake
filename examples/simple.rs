#[macro_use]
extern crate cake;

build! {
    start(sodium, libstd) => cmd!(in "src"; "ls"),
    sodium(libstd, libextra) => println!("yay"),
    libstd() => println!("libstd"),
    libextra() => cmd!("ls"),
}
