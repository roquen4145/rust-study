# rust diesel practice

## install diesel

- create a new directory
- add dependency to Cargo.toml
- install diesel_cli  
``` bash
cargo install diesel_cli --no-default-features --features postgres
```
- setup diesel by following command
``` bash
echo DATABASE_URL=postgres://username:password@localhost/diesel_demo > .env
diesel setup
```
- create main migration
``` bash
diesel migration generate create_posts
```
- 
## install postgres

[docker-postgres](https://devinlife.com/postgresql/run-postgresql-on-docker/)
when you fail to run docker, install docker-desktop version on mac

## diesel_cli install link fail issue on Mac
$ brew install libpq
$ export PQ_LIB_DIR="${brew --prefix libpq}/lib"

## diesel_cli install link fail issue on Windows
insatll postgresql
$ setx PQ_LIB_DIR "C:\Program Files\PostgreSQL\14\lib"
reboot


# Trouble Shooting

## the trait 'Expression' is not implemented for naiveDateTime
diesel feature chrono needed in toml
``` toml
[depependencies]
diesel = { version = "1.4.8", features = ["chrono"] }
```

## reference

[diesel](https://diesel.rs/guides/getting-started)