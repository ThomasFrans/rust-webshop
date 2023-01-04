# Rust Webshop
A fictional webshop that sells stickers to put on your laptop, written in Rust using Rocket and Diesel. This project started because I had just completed an assignment to write a webshop in PHP, and felt like Rust could do it better. It appears I was right, as development in Rust hans't only been simpler, but also much more fun! I hope that this can be an example/inspiration for other people who want to start with web development in Rust, but don't really know where to start.

# Code style
- Clippy is completely broken when using Rocket macros, so clippy isn't used at all.

# Crates used
- Rocket: Used as the web server;
- Diesel: Used both for its ORM capabilities and the migrations from the cli tool.

# Setup
Setup can either be done through Docker, or manually. The Docker approach works
for the most part, but doesn't allow to add users yet, so is currently pretty
useless.

## Docker
Run `docker compose up` with environment variables `POSTGRES_USER`,
`POSTGRES_DB` and `POSTGRES_PASSWORD` set. This should automatically download
everything, create the database, run the migrations on it and start the
webserver on port 80 (http://localhost)

## Manually
### Step 1
Set up PostgreSQL server somewhere. Specific steps on how to do this are on their website. Make sure the user you are going to use for the database has the `INSERT`, `ALTER`, `DROP`, `UPDATE`, `CREATE` and `SELECT` privileges.

### Step 2
Create a database on the PostgreSQL server.

### Step 3
Install `diesel-cli` and create a file `.env` in the project root with a variable `DATABASE_URL`.
The value of the variable should be the URL to the database on the PostgreSQL server.

### Step 4
Add this to a file `Rocket.toml` (case-sensitive) in the project root:
```toml
[default.databases.webshop]
url = "postgresql://<username>:<password>@<host>/<database_name>"
```

### Step 5
Run `diesel setup`. This will create the database set in the `.env` file and run all migrations.

### Step 6
Generate a random key for Rocket to use for private cookies.

`openssl rand -base64 32`

Copy the result to the `Rocket.toml` file:
```toml
[release]
secret_key = "<generated_key>"
```

That should be it. If you now run the program and go to http://localhost, you should see the website.
