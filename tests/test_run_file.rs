use odoomigration::run_file::RunFile;


#[test]
fn test_from_file() {
    // assert_eq!(3, 4); 
    let run_file = RunFile::from_file("tests/static/test_run_file_1");
    assert_eq!(run_file.mode_section.cache_dir, "odoomigrations_cache"); 
}