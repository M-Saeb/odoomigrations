# Odoo Migrations

## <img src='https://img.icons8.com/office/344/error.png' width='20'> This project is still a work in progress

This cli tools is used to automate the migrations of an odoo istance from one machine to another. It can copy the database, custom modules and even conf file.
The simplist use of this CLI would be as followes:
```
odoomigrations run.conf
```

the file `run.conf` should contain all the needed info for migration, and should be written in the following format

```
[mode]
cache_dir = odoomigrations_cache # default is odoomigrations_cache, more info about it below
db_migrate_method = local # default is local, more info about below
log = true # default is true, more info about below

[source]
address = 127.0.0.1 # default is 127.0.0.1 only IP address are supported at the moment
port = 80 # default is 8069
database_name = main # required if you wish to backup database
master_password = password # required if you wish to backup database
c_addons_path = /custom/modules/path # required if you wish to backup custom modules
config_file_path = /etc/odoo/odoo.conf # required if you wish to backup configuration file
ssh_username = odoo # required if you wish to backup custom modules or onconfiguration file
ssh_password = odoo required if you wish to backup custom modules or onconfiguration file

[destination]
address = 127.0.0.1 # same as source "address" option
port = 8069
database_name = main_dup # required if you wish to upload database backup
master_password = password # required if you wish to upload database backup
c_addons_path = /custom/modules/path # required if you wish to upload custom modules
config_file_path = /etc/odoo/odoo.conf # required if you wish to upload configuration file
ssh_username = odoo # required if you wish to upload custom modules or configuration file
ssh_password = odoo # required if you wish to upload custom modules or configuration file
```

a few other optional argument you can add are the following:


## `cache_dir`
- the directory that the source files & folders will be downloaded to before uploading to the destination folder. its defualt value is `./odoomigrations_cache`
```
-- odoomigrations_cache
	|
	| - db.zip
	| - c_addons
	| - file.conf
	L - migration.log
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

## `log`
when set to true, it'll log the steps the the CLI took in the `odoomigrations_cache` path with the file name `migration.log`
when set to anything else, no log will be made