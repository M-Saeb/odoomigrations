use std::fs;
use std::path::Path;
use odoomigration::run_file::RunFile;

#[tokio::test]
async fn test_file_1_sequence(){
	let run_file = RunFile::from_file("tests/static/test_run_file_1");
	run_file.run_proces().await;
	assert!(Path::new("test_odoomigrations_cache").exists(), "cache file was not found");
	assert!(Path::new("test_odoomigrations_cache/output.log").exists(), "log file was not created");
	let log_file = fs::read_to_string("test_odoomigrations_cache/output.log").expect("The migrations log file was not found");
	let mut lines = log_file.split("\n");
	assert!(
		lines.nth(0).expect("Line was not created")
		.contains("created local cache directory named test_odoomigrations_cache"),
		"line contain wrong value"
	);
	assert!(
		lines.nth(0).expect("Line was not created")
		.contains("called backup request for source server"),
		"line contain wrong value"
	);
	assert!(
		lines.nth(0).expect("Line was not created")
		.contains("successfuly backeduped database for source server"),
		"line contain wrong value"
	);
	assert!(Path::new("test_odoomigrations_cache/db.zip").exists(), "database file was not found");
	assert!(
		lines.nth(0).expect("Line was not created")
		.contains("ran scp for source server c_addons"),
		"line contain wrong value"
	);
	assert!(
		lines.nth(0).expect("Line was not created")
		.contains("downloaded source server c_addons"),
		"line contain wrong value"
	);
	assert!(Path::new("test_odoomigrations_cache/c_addons").exists(), "c_addons folder was not found");
	assert!(
		lines.nth(0).expect("Line was not created")
		.contains("ran scp for source server config_file"),
		"line contain wrong value"
	);
	assert!(
		lines.nth(0).expect("Line was not created")
		.contains("downloaded source server config_file"),
		"line contain wrong value"
	);
	assert!(Path::new("test_odoomigrations_cache/file.conf").exists(), "configuration file was not found");
	assert!(
		lines.nth(0).expect("Line was not created")
		.contains("called restore request for destination server"),
		"line contain wrong value"
	);
	assert!(
		lines.nth(0).expect("Line was not created")
		.contains("successfuly restored database for destination server"),
		"line contain wrong value"
	);
	assert!(
		lines.nth(0).expect("Line was not created")
		.contains("ran scp for destination server c_addons"),
		"line contain wrong value"
	);
	assert!(
		lines.nth(0).expect("Line was not created")
		.contains("uploaded destination server c_addons"),
		"line contain wrong value"
	);
	assert!(
		lines.nth(0).expect("Line was not created")
		.contains("ran scp for destination server config_file"),
		"line contain wrong value"
	);
	assert!(
		lines.nth(0).expect("Line was not created")
		.contains("uploaded destination server config_file"),
		"line contain wrong value"
	);
}