//! Generates the C bindings for the Ipopt library at build time.

use std::env;
use std::path::PathBuf;

/// On Windows, ues `cc` to try to detect the MSVC include directory and add
/// it to the bindgen builder. If this does not work, then the function will
/// not modify the input builder instance.
///
/// On other platforms, this is an identity operation that returns the input builder.
#[allow(dead_code)]
#[allow(unreachable_code)]
fn try_add_msvc_include_directory(builder: bindgen::Builder) -> bindgen::Builder {
    #[cfg(target_os = "windows")]
    {
        let target = env::var("TARGET").unwrap();
        let msvc_tool = cc::windows_registry::find_tool(&target, "cl.exe");
        if msvc_tool.is_none() {
            return builder;
        }

        // There presumably has to be a better way to do this :(
        //
        // We could reverse search via ancestors path up to the directory that by default would contain
        // all MSVC toolchains:
        // C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\MSVC\<version>\include
        // But that will cuase problems + long build times if for some reason the expected `include`
        // path is not found.
        //
        // We may also end up choosing a different version of MSVC if the include directory *does*
        // exist in a different version directory.
        //
        // So I am just leaving this as is for now. If the include path isn't there, then the
        // headers won't get found and the error message in the build will be pretty obvious.
        //
        // Adding the specific, desired system include path
        // to the INCLUDE environment variable is always possible too.
        let msvc_tool_include_path = format!(
            "{0}\\..\\..\\..\\..\\include",
            msvc_tool.unwrap().path().to_str().unwrap()
        );

        // Appends the MSVC include path to the bindgen builder and immediately returns it.
        return builder.clang_arg(format!("-isystem{}", msvc_tool_include_path));
    }

    builder
}

fn main() {
    // Skip building the bindings if we are on docs.rs, otherwise we will get build failures.
    if std::env::var("DOCS_RS").is_ok() {
        return;
    }

    println!("cargo:rustc-link-lib=ipopt");
    println!("cargo::rerun-if-changed=IpoptWrapper.h");

    let bindings = try_add_msvc_include_directory(bindgen::Builder::default())
        .header("IpoptWrapper.h")
        .generate()
        .expect("Unable to generate Ipopt bindings.");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write Ipopt bindings!");
}
