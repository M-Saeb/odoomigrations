# Odoo Migrations
This cli tools is used to automate the migrations of an odoo istance from one machine to another. It can copy the database, custom modules and even conf file.
The simplist use of this CLI would be as followes:
```
odoomigrations run.conf
```

the file `run.conf` should contain all the needed info for migration, and should be written in the following format

```
source_ip = 127.0.0.1 # default is 127.0.0.1
source_port = 8069 # default is 8069
source_database_name = main # REQUIRED !!
source_master_password = password # REQUIRED !!
source_c_addons_path = /custom/modules/path # optional
source_conf_file = /etc/odoo/odoo.conf # optional
source_ssh_username = odoo # optional, becomes required when using source_c_addons_path or source_conf_file
source_ssh_password = odoo # optional, becomes required when using source_c_addons_path or source_conf_file

# if you only supplied the source info than the CLI will migrate all info above to your local machine under odoomigrations_cache directory
# more info about below
# once you add the dest_ip argument the CLI will attempt to push the all the files & folder to the required machine

dest_ip = 127.0.0.1 # default is 127.0.0.1
dest_port = 8069 # default is 8069
dest_database_name = main_dup # REQUIRED !!
dest_master_password = password # REQUIRED !!
dest_c_addons_path = /custom/modules/path # optional
dest_conf_file = /etc/odoo/odoo.conf # optional
dest_ssh_username = odoo # optional, becomes required when using source_c_addons_path or source_conf_file
dest_ssh_password = odoo # optional, becomes required when using source_c_addons_path or source_conf_file
```

a few other optional argument you can add are the following:


## `cache_dir`
- the directory that the source files & folders will be downloaded to before uploading to the destination folder. its defualt value is `./odoomigrations_cache`
```
-- odoomigrations_cache
	|
	| - db.zip
	| - c_addons
	L - file.conf
```
- by adding the `cache_dir` and makeing sure its in the proper format, you can start uploading to the destination machine without the need to download again from the source machine.
- if you wish to change the `odoomigrations_cache` path you can use the following format:
```
cache_dir = /home/odoo/tmp_odoomigrations_cache
```
- This argument is invalid if `db_migrate_method` was set to `direct`


## `db_migrate_method`
to decide the preferable migration method for you
- `local`: to downlod the migrated database, custom modules, and confiugration file locally before pushing it to the destination database
- - This is the default option. 
- - This method used the `cache_dir` to decided where to locally download all these files
- - Its the preferable method because it lets adds some kind of checkpoint incase the uploading fails

- `direct`: run all the commands from the destination database, so that downloading the custom modules, configuration file happends directly to the wanted destination
- - when selecting this method, addin the `dest_ssh_username` and `dest_ssh_password` becomes required
- - incase the migrations failed midway with this method, there is not way to revert what was downloaded and changed, so use it with caution