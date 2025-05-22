#![cfg_attr(windows, feature(abi_vectorcall))]

mod exception;
mod pdf;
mod php_stream;

use crate::exception::LoadException;
use crate::pdf::Document;
use ext_php_rs::prelude::*;

fn build_module(builder: ModuleBuilder) -> ModuleBuilder {
    builder.class::<Document>().class::<LoadException>()
}

#[doc(hidden)]
#[unsafe(no_mangle)]
extern "C" fn get_module() -> *mut ::ext_php_rs::zend::ModuleEntry {
    static __EXT_PHP_RS_MODULE_STARTUP: ::ext_php_rs::internal::ModuleStartupMutex =
        ::ext_php_rs::internal::MODULE_STARTUP_INIT;

    extern "C" fn ext_php_rs_startup(ty: i32, mod_num: i32) -> i32 {
        __EXT_PHP_RS_MODULE_STARTUP
            .lock()
            .take()
            .inspect(|_| ::ext_php_rs::internal::ext_php_rs_startup())
            .expect("Module startup function has already been called.")
            .startup(ty, mod_num)
            .map(|_| 0)
            .unwrap_or(1)
    }

    let builder = build_module(ModuleBuilder::new("lopdf", env!("CARGO_PKG_VERSION")))
        .startup_function(ext_php_rs_startup);

    match builder.try_into() {
        Ok((entry, startup)) => {
            __EXT_PHP_RS_MODULE_STARTUP.lock().replace(startup);
            entry.into_raw()
        }
        Err(e) => panic!("Failed to build PHP module: {:?}", e),
    }
}

#[cfg(debug_assertions)]
#[unsafe(no_mangle)]
pub extern "C" fn ext_php_rs_describe_module() -> ::ext_php_rs::describe::Description {
    use ::ext_php_rs::describe::*;

    Description::new(build_module(ModuleBuilder::new("lopdf", env!("CARGO_PKG_VERSION"))).into())
}

#[cfg(test)]
mod integration {
    use std::env;

    use std::process::Command;
    use std::sync::Once;

    static BUILD: Once = Once::new();

    fn setup() {
        BUILD.call_once(|| {
            assert!(
                Command::new("cargo")
                    .arg("build")
                    .output()
                    .expect("failed to build extension")
                    .status
                    .success()
            );
        });
    }

    pub fn run_php(file: &str) -> bool {
        setup();
        let mut path = env::current_dir().expect("Could not get cwd");
        path.push("target");
        path.push("debug");
        path.push(if std::env::consts::DLL_EXTENSION == "dll" {
            "lopdf"
        } else {
            "liblopdf"
        });
        path.set_extension(std::env::consts::DLL_EXTENSION);
        let output = Command::new("php")
            .arg(format!("-dextension={}", path.to_str().unwrap()))
            .arg("-dassert.active=1")
            .arg("-dassert.exception=1")
            .arg("-dzend.assertions=1")
            .arg(format!("tests/integration/{}", file))
            .output()
            .expect("failed to run php file");
        if output.status.success() {
            true
        } else {
            panic!(
                "
                status: {}
                stdout: {}
                stderr: {}
                ",
                output.status,
                String::from_utf8(output.stdout).unwrap(),
                String::from_utf8(output.stderr).unwrap()
            );
        }
    }

    #[test]
    pub fn test_include_external() {
        assert!(run_php("0001-load-file.php"));
    }
}
