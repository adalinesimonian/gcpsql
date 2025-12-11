# gcpsql

A simple wrapper around the [GCP SQL Proxy](https://cloud.google.com/sql/docs/mysql/sql-proxy) and `psql` to connect to a Google Cloud SQL PostgreSQL database.

```bash
gcpsql my-project:europe-west1:my-instance my-database
```

Username and password can be provided with the `PGUSER` and `PGPASSWORD` environment variables or the `--user` and `--password` command line options.

`.env` files are supported and will be loaded automatically if present in the current directory.

## Licence

[ISC](LICENCE)
