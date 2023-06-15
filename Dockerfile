FROM rust:1.70

# Copy source
COPY . /src
WORKDIR /src

# Build release
RUN mkdir /app
RUN cargo build --release
RUN cp target/release/byakuren_ssg /app/byakuren_ssg

RUN cp -r templates /app/templates
RUN cp -r static /app/static
RUN cp Rocket.toml /app/Rocket.toml
# Mount the markdown pages here
RUN mkdir /app/pages

EXPOSE 8000

# Cleanup source dir
RUN cd /
RUN rm -rf /src

WORKDIR /app
ENTRYPOINT ["/app/byakuren_ssg"]
