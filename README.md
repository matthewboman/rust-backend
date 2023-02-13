```sh
cargo make docker up -d
```

To shut it down:

```sh
cargo make docker down
```

Create a database based on the `DATABASE_URL` in the `.envrc`, if you haven't already:

```sh
cargo make db-create
```

Run migrations:

```sh
cargo make db-migrate
```

If you want to wipe your database and start over:

```sh
cargo make db-reset
```