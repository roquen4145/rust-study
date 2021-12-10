# rust diesel practice

## install diesel

- create a new directory
- add dependency to Cargo.toml
- install diesel_cli
- setup diesel by following command
``` bash
echo DATABASE_URL=postgres://username:password@localhost/diesel_demo > .env
diesel setup
```

## install postgres

[docker-postgres](https://devinlife.com/postgresql/run-postgresql-on-docker/)
when you fail to run docker, install docker-desktop version on mac

## diesel_cli install fail issue  
brew install libpq
export PQ_LIB_DIR="${brew --prefix libpq}/lib"


## reference

[diesel](https://diesel.rs/guides/getting-started)