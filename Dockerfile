# The first stage, called `build` compiles the Rust program.
# The second stage is the actual image that will be used to generate conainers from.

# ===============
# = BUILD STAGE =
# ===============

# FROM: Download this image if necessary and generate a container that will run the RUN directives.
FROM ubuntu:jammy as build

# Install the packages necessary for compiling webshop.
RUN apt-get update
RUN apt-get install -y curl build-essential libpq-dev libpq5

# COPY: Copy files from the build context (all files in the directory of the Dockerfile, except things specified in .dockerignore).
WORKDIR /build
COPY src /build/src/
COPY migrations /build/migrations/
COPY Cargo.lock Cargo.toml /build/
COPY rust-toolchain /tmp/

# ENV: Declare variables that are used by Docker and are set as environment variables for RUN commands.
ENV RUSTUP_VERSION="1.25.1"
ENV RUSTUP_TRIPLE="x86_64-unknown-linux-gnu"
ENV RUSTUP_SHA="5cc9ffd1026e82e7fb2eec2121ad71f4b0f044e88bca39207b3f6b769aaa799c"
RUN curl "https://static.rust-lang.org/rustup/archive/${RUSTUP_VERSION}/${RUSTUP_TRIPLE}/rustup-init" > /tmp/rustup-init && \
    echo "${RUSTUP_SHA}  /tmp/rustup-init" | sha256sum --check && \
    chmod +x /tmp/rustup-init && \
    /tmp/rustup-init -y --no-modify-path --default-toolchain $(cat /tmp/rust-toolchain)

# Add the path to the installed rustup (installed as root, so in root $HOME).
ENV PATH=/root/.cargo/bin:$PATH

RUN cargo build --release

# ========================
# = IMAGE CREATION STAGE =
# ========================

FROM ubuntu:jammy

RUN apt-get update
RUN apt-get install -y libpq5

COPY --from=build /build/target/debug/webshop /usr/local/bin/webshop

WORKDIR /usr/local/bin/
COPY migrations migrations
COPY templates templates
COPY static static
COPY webshop.toml* webshop.toml

# ENTRYPOINT: Can't be overriden at runtime. Image has single purpose.
# CMD: Default command to run. Can be overriden at runtime (for example in docker-compose.yml).

ENTRYPOINT ["webshop"]
