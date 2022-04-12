# Servitor

This is main source code repository for Servitor. Servitor is Discord bot designed to help to manage SEA Discord server and infrastructure.

## Dependencies

The bot source code is written in Rust (`rustc 1.60.0 (7737e0b5c 2022-04-04)`). You can see Rust dependencies in `Cargo.toml` and `Cargo.lock` files. You also need to install PostgreSQL server unless you use remote one.  

## Usage

### cargo build

One option is to prebuilt binary and launch it manually

```
$ git clone https://github.com/sea-auca/servitor.git
$ cd servitor
$ cargo build --release
$ DISCORD_TOKEN=<token> DISCORD_DATABASE_HOST=<hostaddr> DISCORD_DATABASE_NAME=<database_name> DISCORD_DATABASE_USER=<database_user_name> DISCORD_DATABASE_PASSWORD=<database_user_password> DISCORD_LOGFILE=<path_to_logfile> ./target/release/servitor
```

If you want to clone via ssh you should make appropriate changes. You can change cargo build profile to debug using `--debug` instead of `--release` but in that case executable is going to be heavy-weighted. You can also change build directory using `--target-dir` flag. Use `cargo build help` for details and other options.

You can invoke executable with

```
$ ./target/release/servitor
```
If you make sure that enviromental variables are set correctly.

### cargo run

Build step can be skipped using `cargo run` command. The process might look like that:

```
$ git clone https://github.com/sea-auca/servitor.git
$ cd servitor
$ DISCORD_TOKEN=<token> DISCORD_DATABASE_HOST=<hostaddr> DISCORD_DATABASE_NAME=<database_name> DISCORD_DATABASE_USER=<database_user_name> DISCORD_DATABASE_PASSWORD=<database_user_password> DISCORD_LOGFILE=<path_to_logfile> cargo run --release
``` 

### cargo install

Final option is to use cargo install. Process may look like this:

```
$ git clone https://github.com/sea-auca/servitor.git
$ cd servitor
$ cargo install --path .
$ DISCORD_TOKEN=<token> DISCORD_DATABASE_HOST=<hostaddr> DISCORD_DATABASE_NAME=<database_name> DISCORD_DATABASE_USER=<database_user_name> DISCORD_DATABASE_PASSWORD=<database_user_password> DISCORD_LOGFILE=<path_to_logfile> servitor
``` 

You should make sure that installation path is in your `$PATH` variable.

### Docker

Alternatively, you could build docker image and run it. The process might look like this:

```
$ git clone https://github.com/sea-auca/servitor.git
$ cd servitor
$ rm -rf target 
$ docker build -t servitor .
```

and once image is build

```
$ docker run servitor -e DISCORD_TOKEN=<token> DISCORD_DATABASE_HOST=<hostaddr> DISCORD_DATABASE_NAME=<database_name> DISCORD_DATABASE_USER=<database_user_name> DISCORD_DATABASE_PASSWORD=<database_user_password> DISCORD_LOGFILE=<path_to_logfile>
```

## Release information

Release notes are available at [CHANGELOG.ьв](https://github.com/sea-auca/servitor/blob/master/Changelog.md)