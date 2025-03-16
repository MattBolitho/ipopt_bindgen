//! Generates the C bindings for the Ipopt library at build time.

use std::{error::Error, fs::File, io::Write, path::PathBuf};

// On Windows, we use `cc` to try to detect the MSVC include directory and add it to the bindgen
// builder. On other platforms, this returns the input builder unchanged.

#[cfg(target_os = "windows")]
fn try_add_msvc_include_directory(builder: bindgen::Builder) -> bindgen::Builder {
    let target = std::env::var("TARGET").unwrap();
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

#[cfg(not(target_os = "windows"))]
fn try_add_msvc_include_directory(builder: bindgen::Builder) -> bindgen::Builder {
    builder
}

fn main() -> Result<(), Box<dyn Error>> {
    const IPOPT_BINDGEN_HEADER: &str = "IpoptBindgen.h";
    const DEFAULT_IPOPT_INCLUDE_PREFIX: &str = "coin-or/";

    // Skip building the bindings if we are on docs.rs, otherwise we will get build failures.
    if std::env::var("DOCS_RS").is_ok() {
        return Ok(());
    }

    let out_dir = std::env::var("OUT_DIR").unwrap();

    println!("cargo::rerun-if-env-changed=IPOPT_BINDGEN_INCLUDE_PREFIX");

    // Generate a file in the output directory that includes the Ipopt C interface header. The
    // IPOPT_BINDGEN_INCLUDE_PREFIX environment variable can be set to override the prefix from
    // which to include. The default value of "coin-or/" is chosen due
    // to being the default install prefix for Ipopt source builds.
    let mut include_prefix = std::env::var("IPOPT_BINDGEN_INCLUDE_PREFIX")
        .unwrap_or(DEFAULT_IPOPT_INCLUDE_PREFIX.into());
    if !include_prefix.ends_with('/') {
        include_prefix.push('/');
    }
    let include_statement = format!("#include <{include_prefix}IpStdCInterface.h>");
    let generated_header_path = format!("{out_dir}/{IPOPT_BINDGEN_HEADER}");
    let mut file = File::create(&generated_header_path)?;
    file.write_all(include_statement.as_bytes())?;

    let bindings = try_add_msvc_include_directory(bindgen::Builder::default())
        .header(generated_header_path)
        .generate()
        .expect("Failed to generate Ipopt bindings.");

    let out_path = PathBuf::from(out_dir);
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Failed to write Ipopt bindings.");

    println!("cargo:rustc-link-lib=ipopt");

    Ok(())
}
