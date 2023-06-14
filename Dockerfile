# Stage 1: Build Rust Code
FROM rust:latest as rust-builder

# Copy and compile your Rust code
WORKDIR /app/backend
COPY backend /app/backend
RUN cargo build --release

# Stage 2: Build Front-end
FROM node:latest as frontend-builder

# Copy and build the front-end code
WORKDIR /app/frontend
COPY frontend /app/frontend
RUN npm install && npm run build

# Stage 3: Create Final Image
FROM gcr.io/distroless/cc-debian11

# Copy .env file from backend
COPY --from=rust-builder \
    /app/backend/.env /app/backend/target/release/pattern_pursuit \
    /app/backend/*.pem \
    /app/backend/

# Copy built front-end files
COPY --from=frontend-builder /app/frontend/static /app/frontend/static

# Set the serving folder for Actix
WORKDIR /app/backend
CMD ["/app/backend/pattern_pursuit"]