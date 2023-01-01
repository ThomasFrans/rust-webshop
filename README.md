# Rust Webshop
A fictional webshop that sells stickers to put on your laptop, written in Rust using Rocket, Diesel and sqlx. This project started because I had just completed an assignment to write a webshop in PHP, and felt like Rust could do it better. It appears I was right, as development in Rust hans't only been simpler, but also much more fun! I hope that this can be an example/inspiration for other people who want to start with web development in Rust, but don't really know where to start.

# Code style
- Clippy is completely broken when using Rocket macros, so clippy isn't used at all.

# Crates used
- Rocket: Used as the web server;
- Diesel: Originally used for both the ORM capabilities and the migrations, but now only for migrations;
- sqlx: A crate that enables asynchronous access to MariaDB, the database I'm using.

# Setup
Currently, setup is completely manual. I'm planning to (finally) learn Docker as part of this experiment. If Docker can be used to automate this (I literally don't have a clue what Docker does at the moment), I'll automate it.

## Step 1
Set up MariaDB server somewhere. Specific steps on how to do this are on their website. Make sure the user you are going to use for the database has the `INSERT`, `ALTER`, `DROP`, `UPDATE`, `CREATE` and `SELECT` privileges.

## Step 2
Create a database on the MariaDB server.

## Step 3
Install `diesel-cli` and create a file `.env` in the project root with a variable `DATABASE_URL`.
The value of the variable should be the URL to the database on the MariaDB server.

## Step 4
Add this to a file `Rocket.toml` (case-sensitive) in the project root:
```toml
[default.databases.mysql_webshop]
url = "mysql://<username>:<password>@<host>/<database_name>"
```

## Step 5
Run `diesel setup`. This will create the database set in the `.env` file and run all migrations.

## Step 6
Generate a random key for Rocket to use for private cookies.

`openssl rand -base64 32`

Copy the result to the `Rocket.toml` file:
```toml
[release]
secret_key = "<generated_key>"
```

That should be it. If you now go to http://localhost:8000, you should see the website.