# Build stage
FROM rust:1.97.1-alpine3.24 as builder

# Устанавливаем зависимости для сборки
RUN apk add --no-cache musl-dev
ENV RUSTFLAGS="-C target-cpu=x86-64-v3"

WORKDIR /app

# Копируем файлы зависимостей для кэширования
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

RUN rm -rf src

# Копируем реальный код и пересобираем
COPY src ./src
RUN touch src/main.rs && cargo build --release

# Runtime stage
FROM alpine:3.24 AS runner

# Устанавливаем runtime зависимости
RUN apk add --no-cache ca-certificates libgcc

WORKDIR /app

RUN adduser -D -u 1000 myuser

# Копируем бинарник
COPY --from=builder /app/target/release/alphabot /app/alphabot

USER myuser

# Токен будет передан через .env файл
CMD ["/app/alphabot"]