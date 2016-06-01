#[macro_use]
extern crate cake;

build! {
    start(sodium, libstd) => cmd!("ls"; in "src"),
    sodium(libstd, libextra) => println!("yay"),
    libstd() => println!("libstd"),
    libextra(run) => cmd!("ls"; where LAL = "2"),
    run() => println!("check"),
}
