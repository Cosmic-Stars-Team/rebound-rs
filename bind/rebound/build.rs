use std::env;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

use flate2::read::GzDecoder;
use reqwest::blocking::Client;
use tar::Archive;
use tempfile::Builder;

const SOURCES: &[&str] = &[
    "rebound.c",
    "tree.c",
    "particle.c",
    "gravity.c",
    "integrator.c",
    "integrator_whfast.c",
    "integrator_whfast512.c",
    "integrator_saba.c",
    "integrator_ias15.c",
    "integrator_sei.c",
    "integrator_bs.c",
    "integrator_leapfrog.c",
    "integrator_mercurius.c",
    "integrator_trace.c",
    "integrator_eos.c",
    "boundary.c",
    "input.c",
    "binarydiff.c",
    "output.c",
    "collision.c",
    "communication_mpi.c",
    "display.c",
    "tools.c",
    "rotations.c",
    "derivatives.c",
    "simulationarchive.c",
    "glad.c",
    "integrator_janus.c",
    "transformations.c",
    "fmemopen.c",
    "server.c",
    "frequency_analysis.c",
];

fn env_flag(name: &str) -> bool {
    matches!(
        env::var(name).ok().as_deref().map(str::trim),
        Some("1") | Some("true") | Some("TRUE") | Some("yes") | Some("on")
    )
}

fn env_flag_with_default(name: &str, default: bool) -> bool {
    match env::var(name) {
        Ok(v) => {
            let v = v.trim().to_ascii_lowercase();
            if v.is_empty() {
                default
            } else {
                !matches!(v.as_str(), "0" | "false" | "no" | "off")
            }
        }
        Err(_) => default,
    }
}

fn rebound_version() -> String {
    env::var("CARGO_PKG_VERSION").expect("CARGO_PKG_VERSION not set")
}

fn find_libomp_prefix() -> Option<PathBuf> {
    if let Ok(prefix) = env::var("LIBOMP_PREFIX") {
        let path = PathBuf::from(prefix);
        if path.exists() {
            return Some(path);
        }
    }
    let candidates = [
        Path::new("/opt/homebrew/opt/libomp"),
        Path::new("/usr/local/opt/libomp"),
    ];
    candidates
        .into_iter()
        .find(|p| p.exists())
        .map(Path::to_path_buf)
}

fn vendored_source_matches(target_dir: &Path, version: &str) -> bool {
    let src_dir = target_dir.join("src");
    let license_file = target_dir.join("LICENSE");
    let version_file = target_dir.join(".version");

    src_dir.is_dir()
        && license_file.is_file()
        && version_file.is_file()
        && fs::read_to_string(version_file)
            .map(|current| current.trim() == version)
            .unwrap_or(false)
}

fn vendor(manifest_dir: &Path, version: &str) -> Result<(), Box<dyn Error>> {
    let target_dir = manifest_dir.join("c_src");
    if vendored_source_matches(&target_dir, version) {
        return Ok(());
    }

    let url = format!("https://github.com/hannorein/rebound/archive/refs/tags/{version}.tar.gz");
    println!("cargo:warning=Vendoring REBOUND v{version} from {url}");

    let client = Client::builder()
        .timeout(Duration::from_secs(300))
        .build()?;
    let response = client
        .get(&url)
        .header("user-agent", "rebound-bind-build-script")
        .send()?
        .error_for_status()?;

    let extract_dir = Builder::new()
        .prefix("rebound-extract-")
        .tempdir_in(manifest_dir)?;
    let stage_dir = Builder::new()
        .prefix("c_src-stage-")
        .tempdir_in(manifest_dir)?;

    let decoder = GzDecoder::new(response);
    let mut archive = Archive::new(decoder);
    archive.unpack(extract_dir.path())?;

    let archive_root = extract_dir.path().join(format!("rebound-{version}"));
    let source_src_dir = archive_root.join("src");
    let source_license_file = archive_root.join("LICENSE");
    if !source_src_dir.is_dir() {
        return Err(format!(
            "downloaded archive for REBOUND v{version} is missing src/: {}",
            source_src_dir.display()
        )
        .into());
    }
    if !source_license_file.is_file() {
        return Err(format!(
            "downloaded archive for REBOUND v{version} is missing LICENSE: {}",
            source_license_file.display()
        )
        .into());
    }

    fs::rename(&source_src_dir, stage_dir.path().join("src"))?;
    fs::rename(&source_license_file, stage_dir.path().join("LICENSE"))?;
    fs::write(stage_dir.path().join(".version"), format!("{version}\n"))?;

    let backup_dir = if target_dir.exists() {
        let backup_dir = Builder::new()
            .prefix("c_src-backup-")
            .tempdir_in(manifest_dir)?;
        fs::rename(&target_dir, backup_dir.path().join("c_src"))?;
        Some(backup_dir)
    } else {
        None
    };

    if let Err(error) = fs::rename(stage_dir.path(), &target_dir) {
        if let Some(backup_dir) = backup_dir.as_ref() {
            let backup_target = backup_dir.path().join("c_src");
            if backup_target.exists() {
                let _ = fs::rename(backup_target, &target_dir);
            }
        }
        return Err(error.into());
    }

    std::mem::forget(stage_dir);
    Ok(())
}

fn build(manifest_dir: &Path, version: &str) {
    let vendor_dir = manifest_dir.join("c_src");
    let c_src_dir = vendor_dir.join("src");
    let rebound_header = c_src_dir.join("rebound.h");

    println!("cargo:rerun-if-changed={}", vendor_dir.display());
    println!(
        "cargo:rerun-if-changed={}",
        vendor_dir.join(".version").display()
    );
    println!("cargo:rerun-if-env-changed=CARGO_PKG_VERSION");
    println!("cargo:rerun-if-env-changed=OPENGL");
    println!("cargo:rerun-if-env-changed=OPENMP");
    println!("cargo:rerun-if-env-changed=OPENMPCLANG");
    println!("cargo:rerun-if-env-changed=MPI");
    println!("cargo:rerun-if-env-changed=AVX512");
    println!("cargo:rerun-if-env-changed=QUADRUPOLE");
    println!("cargo:rerun-if-env-changed=PROFILING");
    println!("cargo:rerun-if-env-changed=FFTW");
    println!("cargo:rerun-if-env-changed=SERVER");
    println!("cargo:rerun-if-env-changed=CC");
    println!("cargo:rerun-if-env-changed=LIBOMP_PREFIX");
    println!("cargo:rerun-if-env-changed=MPI_CLANG_ARGS");

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let is_windows = target_os == "windows";
    let is_linux = target_os == "linux";
    let is_macos = target_os == "macos";

    let mut opengl = env_flag("OPENGL");
    let openmp = env_flag("OPENMP");
    let openmp_clang = env_flag("OPENMPCLANG");
    let mpi = env_flag("MPI");
    let avx512 = env_flag("AVX512");
    let quadrupole = env_flag("QUADRUPOLE");
    let profiling = env_flag("PROFILING");
    let fftw = env_flag("FFTW");
    let server = env_flag_with_default("SERVER", true);

    if is_windows {
        if opengl {
            println!(
                "cargo:warning=OpenGL not supported on Windows in upstream Makefile, disabling OPENGL."
            );
            opengl = false;
        }
        if mpi {
            panic!("MPI currently not supported on Windows. Please set MPI=0.");
        }
        if openmp || openmp_clang {
            panic!("OPENMP currently not supported on Windows. Please set OPENMP=0.");
        }
        if avx512 {
            panic!("AVX512 currently not supported on Windows. Please set AVX512=0.");
        }
    }

    let mut cc_build = cc::Build::new();
    cc_build.include(&c_src_dir);

    for source in SOURCES {
        cc_build.file(c_src_dir.join(source));
    }

    cc_build.define("_GNU_SOURCE", None);
    cc_build.define("GITHASH", Some(version));

    if is_windows {
        cc_build.flag_if_supported("/Ox");
        cc_build.flag_if_supported("/fp:precise");
    } else {
        cc_build.flag_if_supported("-std=c99");
        cc_build.flag_if_supported("-Wpointer-arith");
        cc_build.flag_if_supported("-fPIC");
        cc_build.flag_if_supported("-Wall");
        cc_build.flag_if_supported("-g");
    }

    if is_linux {
        println!("cargo:rustc-link-lib=m");
        println!("cargo:rustc-link-lib=rt");
    }
    if is_macos {
        cc_build.define("_APPLE", None);
        cc_build.include("/usr/local/include");
        println!("cargo:rustc-link-search=native=/usr/local/lib");
    }

    let mut bindgen_defines: Vec<String> = vec![format!("GITHASH={version}")];

    if mpi {
        if env::var_os("CC").is_none() {
            cc_build.compiler("mpicc");
        }
        cc_build.define("MPI", None);
        bindgen_defines.push("MPI".to_owned());
    }
    if fftw {
        cc_build.define("FFTW", None);
        bindgen_defines.push("FFTW".to_owned());
        println!("cargo:rustc-link-lib=fftw3");
    }
    if server {
        cc_build.define("SERVER", None);
        bindgen_defines.push("SERVER".to_owned());
    }
    if opengl {
        cc_build.define("OPENGL", None);
        bindgen_defines.push("OPENGL".to_owned());
        if is_macos {
            cc_build.include("/opt/homebrew/include");
            println!("cargo:rustc-link-search=native=/opt/homebrew/lib");
            println!("cargo:rustc-link-lib=glfw");
            println!("cargo:rustc-link-lib=framework=Cocoa");
            println!("cargo:rustc-link-lib=framework=OpenGL");
            println!("cargo:rustc-link-lib=framework=IOKit");
            println!("cargo:rustc-link-lib=framework=CoreVideo");
        } else {
            println!("cargo:rustc-link-lib=glfw");
        }
    }
    if avx512 {
        cc_build.define("AVX512", None);
        bindgen_defines.push("AVX512".to_owned());
    }
    if quadrupole {
        cc_build.define("QUADRUPOLE", None);
        bindgen_defines.push("QUADRUPOLE".to_owned());
    }
    if profiling {
        cc_build.define("PROFILING", None);
        bindgen_defines.push("PROFILING".to_owned());
    }

    if openmp {
        cc_build.define("OPENMP", None);
        bindgen_defines.push("OPENMP".to_owned());
        if env::var("CC").ok().as_deref() == Some("icc") {
            cc_build.flag_if_supported("-openmp");
            println!("cargo:rustc-link-arg=-openmp");
        } else {
            cc_build.flag_if_supported("-fopenmp");
            println!("cargo:rustc-link-arg=-fopenmp");
        }
    } else if openmp_clang {
        cc_build.define("OPENMP", None);
        bindgen_defines.push("OPENMP".to_owned());
        cc_build.flag_if_supported("-Xpreprocessor");
        cc_build.flag_if_supported("-fopenmp");
        if let Some(prefix) = find_libomp_prefix() {
            let include_dir = prefix.join("include");
            let lib_dir = prefix.join("lib");
            if include_dir.exists() {
                cc_build.include(include_dir);
            }
            if lib_dir.exists() {
                println!("cargo:rustc-link-search=native={}", lib_dir.display());
            }
        } else {
            println!(
                "cargo:warning=OPENMPCLANG=1 but no LIBOMP_PREFIX found. Set LIBOMP_PREFIX to your libomp install prefix."
            );
        }
        println!("cargo:rustc-link-lib=omp");
    } else if !is_windows {
        cc_build.flag_if_supported("-Wno-unknown-pragmas");
    }

    cc_build.compile("rebound");

    let mut bindgen_builder = bindgen::Builder::default()
        .header(rebound_header.display().to_string())
        .clang_arg(format!("-I{}", c_src_dir.display()))
        .allowlist_item("^(reb_|REB_).*$")
        .allowlist_item("^_reb_.*$")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()));

    for define in bindgen_defines {
        bindgen_builder = bindgen_builder.clang_arg(format!("-D{define}"));
    }
    if mpi {
        if let Ok(extra) = env::var("MPI_CLANG_ARGS") {
            for arg in extra.split_whitespace() {
                bindgen_builder = bindgen_builder.clang_arg(arg);
            }
        } else {
            println!(
                "cargo:warning=MPI=1 but MPI_CLANG_ARGS is unset; set it if bindgen cannot find mpi.h."
            );
        }
    }

    let bindings = bindgen_builder
        .generate()
        .expect("Unable to generate bindings");

    let out_path = manifest_dir.join("src").join("bindings_gen.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}

fn main() {
    let manifest_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));
    let version = rebound_version();

    vendor(&manifest_dir, &version).unwrap_or_else(|error| {
        panic!("failed to vendor REBOUND v{version}: {error}");
    });
    build(&manifest_dir, &version);
}
