#[macro_use]
extern crate cake;

const LS_FLAGS: &'static [&'static str] = &["-a", "/"];

build! {
    start(sodium, libstd) => cmd!("ls"; in "src"),
    sodium(libstd, libextra) => println!("yay"),
    libstd() => println!("libstd"),
    libextra(run, other) => cmd!("ls"; where LAL = "2"),
    other(run) => cmd!("ls", LS_FLAGS),
    run() => println!("check"),
}
