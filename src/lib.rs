pub extern crate rayon;

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

#[macro_export]
macro_rules! par {
    () => {};
    ($a:expr) => { $a };
    ($a:expr, $($rest:expr),*) => {
        $crate::rayon::join(|| $a, || par!($($rest),*))
    };
}

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
