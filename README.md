:cake: Cake
===========

Cake is a simple, Rustic build tool, which is configured through the advanced macro system of Rust, making it very flexible and expressive.

Features & advantages
---------------------

1. A sane and obvious syntax.
2. Fast parallel builds through work-stealing.
3. Ahead of time compilation.
4. Efficient dependency resolution.

An example
----------

```rust
#[macro_use]
extern crate cake;

build! {
    start(sodium, libstd) => cmd!("ls"; in "src"),
    sodium(libstd, libextra) => println!("yay"),
    libstd() => println!("libstd"),
    libextra(run) => cmd!("ls"; where LAL = "2"),
    run() => println!("check"),
}
```

The syntax
----------

The build is declared through the `build!` macro, which, when invoked, expands to the main function. The `build!` macro takes a block, containing a match like syntax:

```rust
recipe(dependencies...) => instructions
```

The first denotes the name of the build recipe. `dependencies`, delimited by `()` and splited by commas, denotes what build recipe this recipe depends on, i.e., requires to build.

A recipe can be failed by returning `Err(())`, e.g. using `try!()` on a result.

For the extra helper macros, see the rendered docs.
