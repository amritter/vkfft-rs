extern crate bindgen;
extern crate cc;

use std::error::Error;
use std::path::PathBuf;

use bindgen::Bindings;

fn build_lib<LD, L, const N: usize, const M: usize>(
  library_dirs: LD,
  libraries: L,
  defines: &[(&str, &str); N],
  include_dirs: &[String; M],
) -> Result<(), Box<dyn Error>>
where
  LD: Iterator,
  LD::Item: AsRef<str>,
  L: Iterator,
  L::Item: AsRef<str>,
{
  let mut build = cc::Build::default();

  build.file("wrapper.cpp").flag("-w").cpp(true).std("c++17");

  for library_dir in library_dirs {
    build.flag(format!("-L{}", library_dir.as_ref()).as_str());
  }

  for library in libraries {
    build.flag(format!("-l{}", library.as_ref()).as_str());
  }

  build.cargo_metadata(true).static_flag(true);

  for (key, value) in defines.iter() {
    build.define(*key, Some(*value));
  }

  for include_dir in include_dirs.iter() {
    build.include(include_dir);
  }

  build.compile("vkfft");

  Ok(())
}

fn gen_wrapper<const N: usize, const M: usize>(
  defines: &[(&str, &str); N],
  include_dirs: &[String; M],
) -> Result<Bindings, Box<dyn Error>>
where
{
  let base_args = [];

  let defines: Vec<String> = defines
    .iter()
    .map(|(k, v)| format!("-D{}={}", k, v))
    .collect();

  let include_dirs: Vec<String> = include_dirs.iter().map(|s| format!("-I{}", s)).collect();

  let clang_args = base_args
    .iter()
    .chain(defines.iter())
    .chain(include_dirs.iter());

  println!("{:?}", clang_args);

  let res = bindgen::Builder::default()
    .clang_args(clang_args)
    .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    .header("wrapper.h")
    .allowlist_recursively(true)
    .allowlist_type("VkFFTConfiguration")
    .allowlist_type("VkFFTLaunchParams")
    .allowlist_type("VkFFTResult")
    .allowlist_type("VkFFTSpecializationConstantsLayout")
    .allowlist_type("VkFFTPushConstantsLayout")
    .allowlist_type("VkFFTAxis")
    .allowlist_type("VkFFTPlan")
    .allowlist_type("VkFFTApplication")
    .allowlist_function("vkfft_sync")
    .allowlist_function("vkfft_append")
    .allowlist_function("vkfft_plan_axis")
    .allowlist_function("vkfft_initialize")
    .allowlist_function("vkfft_delete")
    .allowlist_function("vkfft_get_version")
    .generate();

  let bindings = match res {
    Ok(x) => x,
    Err(_) => {
      eprintln!("Failed to generate bindings.");
      std::process::exit(1);
    }
  };

  Ok(bindings)
}

const VKFFT_MAX_FFT_DIMENSIONS_DEFAULT: usize = 4;

fn main() -> Result<(), Box<dyn Error>> {
  let vkfft_max_fft_dimensions = match std::env::var("VKFFT_MAX_FFT_DIMENSIONS") {
    Ok(env_var) => usize::from_str_radix(&env_var, 10)?,
    Err(_) => VKFFT_MAX_FFT_DIMENSIONS_DEFAULT,
  };

  println!(
    "cargo::rustc-env=VKFFT_MAX_FFT_DIMENSIONS={}",
    vkfft_max_fft_dimensions
  );

  let vulkan_library = pkg_config::Config::new()
    .atleast_version("1.3.280")
    .probe("vulkan")?;

  if vulkan_library.include_paths.len() != 1 {
    panic!("No vulkan include information")
  }
  let vulkan_include_dir = vulkan_library.include_paths[0].clone();
  let glslang_include_dir = vulkan_include_dir.join("glslang").join("Include");

  let cargo_manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);
  let vkfft_include_dir = cargo_manifest_dir.join("VkFFT").join("vkFFT");

  let out_dir = PathBuf::from(std::env::var("OUT_DIR")?);

  let library_dirs: Vec<_> = vulkan_library
    .link_paths
    .iter()
    .map(|p| p.to_string_lossy())
    .collect();

  let libraries = [
    "glslang",
    "MachineIndependent",
    "OSDependent",
    "GEnericCOdeGen",
    "vulkan",
    "SPIRV",
    "SPIRV-Tools",
    "SPIRV-Tools-opt",
  ];

  for library_dir in library_dirs.iter() {
    println!("cargo:rustc-link-search={}", library_dir);
  }

  for library in libraries.iter() {
    println!("cargo:rustc-link-lib={}", library);
  }

  println!("cargo:rerun-if-changed=wrapper.c");
  println!("cargo:rerun-if-changed=build.rs");

  let include_dirs = [
    vkfft_include_dir.to_string_lossy().to_string(),
    glslang_include_dir.to_string_lossy().to_string(),
  ];

  let defines = [("VKFFT_BACKEND", "0"), ("VK_API_VERSION", "11")];

  build_lib(
    library_dirs.iter(),
    libraries.iter(),
    &defines,
    &include_dirs,
  )?;

  let bindings = gen_wrapper(&defines, &include_dirs)?;
  bindings.write_to_file(out_dir.join("bindings.rs"))?;

  let consts = format!(
    "pub const VKFFT_MAX_FFT_DIMENSIONS: usize = {};",
    vkfft_max_fft_dimensions
  );
  std::fs::write(out_dir.join("consts.rs"), consts)?;

  Ok(())
}
