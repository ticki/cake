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
        use std::sync::atomic::{self, AtomicBool};

        #[derive(Default)]
        struct CakeBuild {
            $(
                $name: AtomicBool
            ),*
        }

        $(
            fn $name(cake_build: &CakeBuild) -> Result<(), ()> {
                par!(
                    $(
                        if !cake_build.$dep.load(atomic::Ordering::SeqCst) {
                            $dep(&cake_build).expect(concat!("recipe, ", stringify!($dep), ", failed to build."));
                        }
                    ),*
                );

                cake_build.$name.store(true, atomic::Ordering::SeqCst);

                $cont;

                Ok(())
            }
        )*

        fn main() {
            let cake_build = CakeBuild::default();

            start(&cake_build).expect("start recipe failed to build.");
        }
    };
}
