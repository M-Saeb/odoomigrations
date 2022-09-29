use odoomigration::run_file::RunFile;


#[test]
fn test_file_1() {
    let run_file = RunFile::from_file("tests/static/test_run_file_1");
    assert_eq!(run_file.mode_section.cache_dir, "test_odoomigrations_cache"); 
    assert_eq!(run_file.mode_section.db_migrate_method, "local"); 
    assert_eq!(
        run_file.source_server.address,
        "http://62.67.200.64"
    );
    assert_eq!(
        run_file.source_server.port,
        "8069"
    );
    assert_eq!(
        run_file.source_server.database_name.expect("database_name should have it's value set"),
        "demo"
    );
    assert_eq!(
        run_file.source_server.master_password.expect("master_password should have it's value set"),
        "8CC2&2@37dKt"
    ); 
    assert_eq!(
        run_file.source_server.c_addons_path.expect("c_addons_path should have it's value set"),
        "/usr/lib/python3/dist-packages/odoo/c-addons"
    ); 
    assert_eq!(
        run_file.source_server.config_file_path.expect("config_file_path should have it's value set"),
        "/etc/odoo/odoo.conf"
    ); 
    assert_eq!(
        run_file.source_server.ssh_username.expect("ssh_username should have it's value set"),
        "root"
    ); 
    assert_eq!(
        run_file.source_server.ssh_password.expect("ssh_password should have it's value set"),
        "root"
    ); 
    assert_eq!(
        run_file.destination_server.address,
        "127.0.0.1"
    );
    assert_eq!(
        run_file.destination_server.database_name.expect("database_name should have it's value set"),
        "main_dup"
    );
    assert_eq!(
        run_file.destination_server.master_password.expect("master_password should have it's value set"),
        "password"
    );
    assert_eq!(
        run_file.destination_server.c_addons_path.expect("c_addons_path should have it's value set"),
        "/custom/modules/path"
    );
    assert_eq!(
        run_file.destination_server.config_file_path.expect("config_file_path should have it's value set"),
        "/etc/odoo/odoo.conf"
    );
    assert_eq!(
        run_file.destination_server.ssh_username.expect("ssh_username should have it's value set"),
        "odoo"
    );
    assert_eq!(
        run_file.destination_server.ssh_password.expect("ssh_password should have it's value set"),
        "odoo"
    );
}

#[test]
#[should_panic(expected="Invalid run file format")]
fn test_file_2() {
    RunFile::from_file("tests/static/test_run_file_2");
}

#[test]
#[should_panic(expected="Invalid value for db_migrate_method. Can only assign 'local' or 'direct'")]
fn test_file_3() {
    RunFile::from_file("tests/static/test_run_file_3");
}

#[test]
fn test_file_4() {
    let run_file = RunFile::from_file("tests/static/test_run_file_4");
    assert_eq!(run_file.mode_section.cache_dir, "odoomigrations_cache"); 
    assert_eq!(run_file.mode_section.db_migrate_method, "local"); 
    assert_eq!(
        run_file.source_server.address,
        "127.0.0.1"
    );
    assert_eq!(
        run_file.source_server.port,
        "8069"
    );
    assert_eq!(
        run_file.source_server.database_name.expect("database_name should have it's value set"),
        "demo"
    ); 
    assert_eq!(
        run_file.source_server.master_password.expect("master_password should have it's value set"),
        "8CC2&2@37dKt"
    ); 
    assert_eq!(
        run_file.source_server.c_addons_path.expect("c_addons_path should have it's value set"),
        "/custom/modules/path"
    ); 
    assert_eq!(
        run_file.source_server.config_file_path.expect("config_file_path should have it's value set"),
        "/etc/odoo/odoo.conf"
    ); 
    assert_eq!(
        run_file.source_server.ssh_username.expect("ssh_username should have it's value set"),
        "odoo"
    ); 
    assert_eq!(
        run_file.source_server.ssh_password.expect("ssh_password should have it's value set"),
        "odoo"
    ); 
    assert_eq!(
        run_file.destination_server.address,
        "127.0.0.1"
    );
    assert_eq!(
        run_file.destination_server.database_name.expect("database_name should have it's value set"),
        "main_dup"
    );
    assert_eq!(
        run_file.destination_server.master_password.expect("master_password should have it's value set"),
        "password"
    );
    assert_eq!(
        run_file.destination_server.c_addons_path.expect("c_addons_path should have it's value set"),
        "/custom/modules/path"
    );
    assert_eq!(
        run_file.destination_server.config_file_path.expect("config_file_path should have it's value set"),
        "/etc/odoo/odoo.conf"
    );
    assert_eq!(
        run_file.destination_server.ssh_username.expect("ssh_username should have it's value set"),
        "odoo"
    );
    assert_eq!(
        run_file.destination_server.ssh_password.expect("ssh_password should have it's value set"),
        "odoo"
    );
}