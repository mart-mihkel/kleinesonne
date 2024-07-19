fn main() -> Result<(), cornucopia::Error> {
    let queries_path = "queries";
    let schema_file = "../../db/psql_dev_dump.sql";
    let destination = "src/cornucopia.rs";
    let settings = cornucopia::CodegenSettings {
        is_async: true,
        derive_ser: true,
    };

    println!("cargo:rerun-if-changed={queries_path}");
    println!("cargo:rerun-if-changed={schema_file}");
    cornucopia::generate_managed(
        queries_path,
        vec![schema_file.to_string()],
        Some(destination),
        false,
        settings,
    )?;

    Ok(())
}
