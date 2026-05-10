use std::env;
use std::path::{Path, PathBuf};

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

fn rebound_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

fn find_libomp_prefix() -> Option<PathBuf> {
    let candidates = [
        Path::new("/opt/homebrew/opt/libomp"),
        Path::new("/usr/local/opt/libomp"),
    ];
    candidates
        .into_iter()
        .find(|p| p.exists())
        .map(Path::to_path_buf)
}

fn build(manifest_dir: &Path, version: &str) {
    let rebound_src_dir = manifest_dir.join("rebound").join("src");
    let rebound_header = rebound_src_dir.join("rebound.h");

    if !rebound_header.is_file() {
        panic!(
            "missing REBOUND C sources at {}; expected bind/rebound/src/rebound.h",
            rebound_header.display()
        );
    }

    println!("cargo:rerun-if-changed={}", rebound_src_dir.display());

    let is_windows = cfg!(target_os = "windows");
    let is_linux = cfg!(target_os = "linux");
    let is_macos = cfg!(target_os = "macos");

    let mut opengl = cfg!(feature = "opengl");
    let openmp = cfg!(feature = "openmp");
    let openmp_clang = cfg!(feature = "openmp-clang");
    let mpi = cfg!(feature = "mpi");
    let avx512 = cfg!(feature = "avx512");
    let quadrupole = cfg!(feature = "quadrupole");
    let profiling = cfg!(feature = "profiling");
    let fftw = cfg!(feature = "fftw");
    let server = cfg!(feature = "server");

    if is_windows {
        if opengl {
            println!(
                "cargo:warning=OpenGL not supported on Windows in upstream Makefile, disabling OPENGL."
            );
            opengl = false;
        }
        if mpi {
            panic!("MPI currently not supported on Windows. Disable the `mpi` feature.");
        }
        if openmp || openmp_clang {
            panic!("OPENMP currently not supported on Windows. Disable the `openmp` feature.");
        }
        if avx512 {
            panic!("AVX512 currently not supported on Windows. Disable the `avx512` feature.");
        }
    }

    let mut cc_build = cc::Build::new();
    cc_build.include(&rebound_src_dir);

    for source in SOURCES {
        cc_build.file(rebound_src_dir.join(source));
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
        cc_build.flag_if_supported("-fopenmp");
        cc_build.flag_if_supported("-openmp");
        println!("cargo:rustc-link-arg=-fopenmp");
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
                "cargo:warning=`openmp-clang` feature enabled but no Homebrew libomp prefix was found."
            );
        }
        println!("cargo:rustc-link-lib=omp");
    } else if !is_windows {
        cc_build.flag_if_supported("-Wno-unknown-pragmas");
    }

    cc_build.compile("rebound");

    let mut bindgen_builder = bindgen::Builder::default()
        .header(rebound_header.display().to_string())
        .clang_arg(format!("-I{}", rebound_src_dir.display()))
        .allowlist_item("^(reb_|REB_).*$")
        .allowlist_item("^_reb_.*$")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()));

    for define in bindgen_defines {
        bindgen_builder = bindgen_builder.clang_arg(format!("-D{define}"));
    }

    let bindings = bindgen_builder
        .generate()
        .expect("Unable to generate bindings");

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));
    let out_path = out_dir.join("bindings_gen.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}

fn main() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let version = rebound_version();

    build(&manifest_dir, version);
}
