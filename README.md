# Rust Webshop
A fictional webshop that sells stickers to put on your laptop, written in Rust using Rocket and Diesel. This project started because I had just completed an assignment to write a webshop in PHP, and felt like Rust could do it better. It appears I was right, as development in Rust hans't only been simpler, but also much more fun! I hope that this can be an example/inspiration for other people who want to start with web development in Rust, but don't really know where to start.

# Code style
- Clippy is completely broken when using Rocket macros, so clippy isn't used at all.

# Crates used
- Rocket: Used as the web server;
- Diesel: Used both for its ORM capabilities and the migrations from the cli tool.
- Serde: General serialization/deserialization (API JSON responses, TOML config file)

# Setup
Setup can either be done through Docker, or manually. The docker approach is nice if you just want to get up 
and running as fast as possible, while the manual approach is way more space efficient and faster to rebuild.

## Docker
Create a file `.env`:

```dotenv
POSTGRES_USER=<database_user>
POSTGRES_DB=<database_name>
POSTGRES_PASSWORD=<database_password>
```

Create a file `webshop.toml`:

```toml
# Address for the Rocket webserver.
webserver_address = "0.0.0.0"

# URL of the database to use.
database_url = "postgresql://<database_user>:<database_password>@database/<database_name>"

# Secret key used by Rocket for encryption, generated with `openssl rand -base64 32` for example.
secret_key = "<secret_key>"
```

Run `docker compose up`. This should automatically download
everything, create the database, run the migrations on it and start the
webserver on port 80 (http://localhost)

## Manually
### Step 1
Set up PostgreSQL server somewhere. Specific steps on how to do this are on their website. Make sure the user you are going to use for the database has the `INSERT`, `ALTER`, `DROP`, `UPDATE`, `CREATE` and `SELECT` privileges.

### Step 2
Create a database on the PostgreSQL server.

### Step 3
Create a file `webshop.toml`:
```toml
# Socket for the Rocket webserver.
webserver_address = "<address>"
webserver_port = "<port>"

# URL of the database to use.
database_url = "postgresql://<database_user>:<database_password>@<database_host>/<database_name>"

# Secret key used by Rocket for encryption, generated with `openssl rand -base64 32` for example.
secret_key = "<secret_key>"
```

That should be it. If you now run the program and go to http://localhost:8000, you should see the website.
