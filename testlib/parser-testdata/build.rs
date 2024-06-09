use std::{
    ffi::OsStr,
    io,
    io::Write,
    path::{Path, PathBuf},
};

#[allow(unused_macros)]
macro_rules! echo {
    ($($tokens: tt)*) => {
        println!("cargo:warning=\r\x1b[32;1m   {}", format!($($tokens)*))
    }
}

macro_rules! warning {
    ($($tokens: tt)*) => {
        println!("cargo:warning=\r\x1b[33;1m   {}", format!($($tokens)*))
    }
}

/// Return the path to root of the crate being built.
///
/// The `CARGO_MANIFEST_DIR` env variable contains the path to the  directory containing the
/// manifest for the package being built (the package containing the build script). Also note that
/// this is the value of the current working directory of the build script when it starts.
///
/// https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts
fn crate_root_dir() -> PathBuf {
    PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"))
}

/// Code generation utilities.
mod codegen {
    use std::path::Path;

    /// Test vector file Jinja template
    const TEST_VECTOR_RS_TEMPLATE: &str = include_str!("data/__templates__/vector.rs.jinja");

    /// Test vector mod.rs Jinja template
    const TEST_VECTOR_MOD_RS_TEMPLATE: &str = include_str!("data/__templates__/mod.rs.jinja");

    /// Initialize the Jinja template engine with the templates.
    pub fn engine() -> &'static Codegen {
        // Init the template engine with the templates
        static CODEGEN: std::sync::OnceLock<Codegen> = std::sync::OnceLock::new();
        CODEGEN.get_or_init(|| {
            let mut jinja = minijinja::Environment::new();
            jinja.set_keep_trailing_newline(true);
            jinja
                .add_template("test_vector", TEST_VECTOR_RS_TEMPLATE)
                .expect("failed test_vector template initialization");
            jinja
                .add_template("test_vector_mod_rs", TEST_VECTOR_MOD_RS_TEMPLATE)
                .expect("failed test_vector_mod_rs template initialization");
            Codegen { jinja }
        })
    }

    /// Wrapper around the Jinja template engine.
    pub struct Codegen {
        jinja: minijinja::Environment<'static>,
    }

    impl Codegen {
        /// Render a test vector using the codegen template engine.
        pub fn render_test_vector(
            &self,
            name: &str,
            data_set: &str,
            content: Vec<String>,
        ) -> String {
            self.jinja
                .get_template("test_vector")
                .expect("template not found")
                .render(minijinja::context! {
                    data_set => data_set,
                    name => name,
                    content => content,
                })
                .expect("render failed")
        }

        /// Render a test vector mod.rs using the codegen template engine.
        pub fn render_test_vector_mod_rs_include(&self, path: impl AsRef<Path>) -> String {
            self.jinja
                .get_template("test_vector_mod_rs")
                .expect("template not found")
                .render(minijinja::context! {
                    path => path.as_ref(),
                })
                .expect("render failed")
        }
    }
}

/// Take a relative path and turn it into a sanitized string.
///
/// - Remove the file extension.
/// - Replace `/` with `_`.
/// - Replace `-` with `_`.
/// - Replace `.` with `_`.
///
/// For example, `exec/query_named.graphql` becomes `exec_query_named`.
fn sanitize_filename(name: impl AsRef<Path>) -> String {
    let base_dir = name.as_ref().parent().expect("file has no parent");
    let file_stem = name.as_ref().file_stem().expect("file has no stem");
    let mut name = PathBuf::new()
        .join(base_dir)
        .join(file_stem)
        .to_string_lossy()
        .to_string();
    name = name.replace('/', "_");
    name = name.replace('-', "_");
    name = name.replace('.', "_");
    name
}

/// Take a file name stem and turn it into a valid rust filename.
///
/// - Append `.rs` to the end.
///
/// For example, `query_named` becomes `exec_query_named.rs`.
fn as_rust_filename(stem: impl AsRef<OsStr>) -> std::ffi::OsString {
    let mut name = stem.as_ref().to_owned();
    name.push(".rs");
    name
}

/// Load the content of a file as a vector of strings, where each string is a line in the file.
fn load_file_and_read_lines(path: impl AsRef<Path>) -> io::Result<Vec<String>> {
    let lines = std::fs::read_to_string(path)?
        .lines()
        .map(|s| s.to_string())
        .collect();
    Ok(lines)
}

/// Given a `Path`, get a directory's file names iterator.
///
/// The iterator will return the file names in the directory in alphabetical order.
fn walk_dir_files(dir: impl AsRef<Path>) -> impl Iterator<Item = PathBuf> {
    let mut paths: Vec<_> = std::fs::read_dir(dir)
        .expect("failed to read directory")
        .filter_map(|dir| dir.ok())
        .filter(|dir| dir.file_type().expect("failed to get file type").is_file())
        .collect();
    paths.sort_by_key(|dir| dir.path());
    paths.into_iter().map(|dir| dir.path())
}

/// Determine if code generation feature is enabled.
fn is_codegen_feature_enabled() -> bool {
    std::env::var("CARGO_FEATURE_CODEGEN").is_ok()
}

fn main() {
    // Run code generation only if `codegen` feature is enabled
    if is_codegen_feature_enabled() {
        let data_root_dir = crate_root_dir().join("data");
        let src_gen_dir = crate_root_dir().join("src").join("gen");

        for test_set_name in ["exec", "kitchen-sink"] {
            let src_dir = data_root_dir.join(test_set_name);
            if !src_dir.exists() {
                warning!("Test vectors set `{test_set_name}` does not exist");
                continue;
            }

            // Create the destination directory
            let dest_dir = src_gen_dir.join(test_set_name);
            std::fs::create_dir_all(&dest_dir).expect("failed to create gen directory");

            // Open (and overwrite) the test vectors set mod.rs file
            let mut mod_file = std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(dest_dir.join("mod.rs"))
                .unwrap_or_else(|err| panic!("failed to open gen/{test_set_name}/mod.rs: {}", err));

            // Generate the test vector files
            for test_vector_file in walk_dir_files(&src_dir) {
                let test_vector_file_name = test_vector_file.file_name().expect("file has no name");
                let test_vector_name = &sanitize_filename(test_vector_file_name);

                // Load the content of the test vector file
                let content = match load_file_and_read_lines(test_vector_file) {
                    Ok(content) => content,
                    Err(err) => {
                        warning!(
                            "failed to read test vector `{test_set_name}/{test_vector_name}` source file: {err}"
                        );
                        continue;
                    }
                };

                let gen_file_name = as_rust_filename(test_vector_name);

                // Render the content and write to the generated file
                let rendered_content =
                    codegen::engine().render_test_vector(test_vector_name, test_set_name, content);
                let gen_file = dest_dir.join(&gen_file_name);
                std::fs::write(&gen_file, rendered_content).unwrap_or_else(|err| {
                    panic!("failed to write test vector `{test_vector_name}` source file: {err}")
                });

                // Append the new line to the mod.rs file
                let rendered_include =
                    codegen::engine().render_test_vector_mod_rs_include(&gen_file_name);
                writeln!(mod_file, "{}", rendered_include).unwrap_or_else(|err| {
                    panic!(
                        "failed to include generated file into gen/{test_set_name}/mod.rs: {err}"
                    )
                });
            }
        }
    }

    // Re-run this build script if any of the files under the data directory has changed.
    println!("cargo:rerun-if-changed=data/**");
}
