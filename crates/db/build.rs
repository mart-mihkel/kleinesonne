fn main() {
    let queries_path = "queries";
    let schema_file = "schema.sql";

    println!("cargo:rerun-if-changed={queries_path}");
    println!("cargo:rerun-if-changed={schema_file}");

    let db_conn_str = std::env::var("DB_STRING").expect("DB_STRING is undefined");
    let output = std::process::Command::new("cornucopia")
        .arg("-q")
        .arg(queries_path)
        .arg("--serialize")
        .arg("live")
        .arg(db_conn_str)
        .output()
        .unwrap();

    if !output.status.success() {
        panic!("{}", &std::str::from_utf8(&output.stderr).unwrap());
    }
}
