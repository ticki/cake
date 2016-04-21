pub extern crate rayon;

/// Execute a command.
///
/// Each argument after the first, denoting the executable, represents a respective argument passed
/// to the executable.  Optionally, you can specify the working directory by prepending `in "blah";
/// ` to the argument list.
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
/// cmd!(in "/"; "ls", ".");
/// ```
#[macro_export]
macro_rules! cmd {
    (in $cd:expr; $cmd:expr $(, $arg:expr)*) => {{
        use std::process;

        if !process::Command::new($cmd)
            $(
                .arg($arg)
            )*
            .current_dir($cd)
            .stdout(process::Stdio::inherit())
            .stderr(process::Stdio::inherit())
            .status().unwrap().success() {
            return Err(());
        }
    }};
    ($cmd:expr $(, $arg:expr)*) => {{
        use std::process;

        if !process::Command::new($cmd)
            $(
                .arg($arg)
            )*
            .stdout(process::Stdio::inherit())
            .stderr(process::Stdio::inherit())
            .status().unwrap().success() {
            return Err(());
        }
    }};
}

/// Evaluate N expressions in parallel.
///
/// This uses work-stealing as back end.
#[macro_export]
macro_rules! par {
    () => {};
    ($a:expr) => { $a };
    ($a:expr, $($rest:expr),*) => {
        $crate::rayon::join(|| $a, || par!($($rest),*))
    };
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
        use std::env;
        use std::sync::atomic::{self, AtomicBool};

        #[derive(Default)]
        struct CakeBuild {
            $(
                $name: AtomicBool
            ),*
        }

        impl CakeBuild {
            $(
                fn $name(&self) -> Result<(), ()> {
                    par!(
                        $(
                            if !self.$dep.load(atomic::Ordering::SeqCst) {
                                self.$dep().expect(concat!("recipe, ", stringify!($dep), ", failed to build."));
                            }
                        ),*
                    );

                    self.$name.store(true, atomic::Ordering::SeqCst);

                    $cont;

                    Ok(())
                }
            )*

            fn run_recipe(&self, cmd: &str) -> Result<(), ()> {
                match cmd {
                    $(
                        stringify!($name) => self.$name(),
                    )*
                    _ => Err(()),
                }

            }
        }

        fn main() {
            let build = CakeBuild::default();

            for i in env::args().skip(1) {
                if build.run_recipe(&i).is_err() {
                    panic!("recipe, {}, failed/not found.", i)
                }
            }
        }
    };
}
