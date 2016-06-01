pub extern crate rayon;

/// Execute a command.
///
/// Each argument after the first, denoting the executable, represents a respective argument passed
/// to the executable.  Optionally, you can specify the working directory by prepending `; in
/// "blah" ` to the argument list. Any arbitrary number of environment variables can be declared
/// through `where VAR = VAL`, delimited by `;`.
///
/// Examples
/// ========
///
/// To run a command in the current working directory:
///
/// ```rust
/// cmd!("ls", "/"); // 'ls /'
/// ```
///
/// If we want to run this in a custom working directory, we can do:
///
/// ```rust
/// cmd!("ls", "."; in "/");
/// ```
///
/// In case we want to set environment variables, we do:
///
/// ```rust
/// cmd!("ls", "."; where BLAH = "/"; where BLUH = "!");
/// ```
#[macro_export]
macro_rules! cmd {
    ($cmd:expr $(, $arg:expr)*; in $cd:expr $(; where $var:ident = $val:expr)*) => {{
        use std::process;

        if !process::Command::new($cmd)
            $(
                .arg($arg)
            )*
            $(
                .env(stringify!($var), $val)
            )*
            .current_dir($cd)
            .stdout(process::Stdio::inherit())
            .stderr(process::Stdio::inherit())
            .status().is_err() {
            return Err(());
        }
    }};
    ($cmd:expr $(, $arg:expr)* $(; where $var:ident = $val:expr)*) => {{
        use std::process;

        if !process::Command::new($cmd)
            $(
                .arg($arg)
            )*
            $(
                .env(stringify!($var), $val)
            )*
            .stdout(process::Stdio::inherit())
            .stderr(process::Stdio::inherit())
            .status().is_err() {
            return Err(());
        }
    }};
}

/// Evaluate N expressions in parallel.
///
/// This uses work-stealing as back end.
#[macro_export]
macro_rules! par {
    () => {
        Ok(())
    };
    ($a:expr) => {{
        try!($a);
        Ok(())
    }};
    ($a:expr, $($rest:expr),*) => {{
        let (a, b) = $crate::rayon::join(|| $a, || par!($($rest),*));
        try!(a);
        try!(b);
        Ok(())
    }};
}

/// Define a build.
///
/// This macro takes a block, which resembles the `match` syntax. Left to each of the arms are the
/// recipe's (compilation unit) name, which is followed by the names of the recipes it depends on,
/// delimited by `()`.  Right to the arm is the recipe's instructions are placed. This is simply
/// normal Rust code, defining the recipe.
///
/// Additionally, this expands to the main function, which handles arguments as well.
///
/// Examples
/// ========
///
/// ```rust
/// #[macro_use]
/// extern crate cake;
///
/// build! {
///     start(sodium, libstd) => cmd!("ls"),
///     sodium(libstd, libextra) => println!("yay"),
///     libstd() => println!("libstd"),
///     libextra() => println!("libextra"),
/// }
/// ```
#[macro_export]
macro_rules! build {
    { $($name:ident($($dep:ident),*) => $cont:expr,)* } => {
        use std::io::Write;
        use std::sync::Mutex;
        use std::{env, io, process};

        enum Error {
            Failed(&'static str),
            NotFound,
        }

        #[derive(Default)]
        struct CakeBuild {
            $(
                $name: Mutex<bool>
            ),*
        }

        fn unit(name: &str) {
            let mut stdout = io::stdout();
            writeln!(stdout, "== Running recipe {} ==", name).unwrap();
        }

        impl CakeBuild {
            $(
                fn $name(&self) -> Result<(), Error> {
                    fn inner() -> Result<(), ()> {
                        unit(stringify!($name));
                        $cont;
                        Ok(())
                    }

                    try!(par!(
                        $(
                            {
                                let mut lock = self.$dep.lock().unwrap();
                                if !*lock {
                                    try!(self.$dep());
                                }

                                *lock = true;

                                Ok(())
                            }
                        ),*
                    ));

                    inner().map_err(|_| Error::Failed(stringify!($name)))
                }
            )*

            fn run_recipe(&self, cmd: &str) -> Result<(), Error> {
                match cmd {
                    $(
                        stringify!($name) => {
                            let res = self.$name();
                            *self.$name.lock().unwrap() = true;
                            res
                        }
                    )*
                    _ => Err(Error::NotFound),
                }
            }
        }

        fn main() {
            let build = CakeBuild::default();
            let stderr = io::stderr();

            let mut run = false;
            for i in env::args().skip(1) {
                run = true;

                if let Err(x) = build.run_recipe(&i) {
                    let mut stderr = stderr.lock();
                    match x {
                        Error::NotFound => writeln!(stderr, "recipe, {}, not found.", i),
                        Error::Failed(name) => writeln!(stderr, "recipe, {}, failed.", name),
                    }.unwrap();
                    stderr.flush().unwrap();
                    process::exit(1);
                }
            }

            let mut stderr = stderr.lock();
            if !run {
                writeln!(stderr, "No argument given. Aborting.").unwrap();
            }
        }
    };
}
