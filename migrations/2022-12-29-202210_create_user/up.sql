CREATE TABLE "users" (
    "user_id" BIGSERIAL PRIMARY KEY,
    "first_name" VARCHAR(32) NOT NULL,
    "surname" VARCHAR(32) NOT NULL,
    "phone" VARCHAR(32) NOT NULL,
    "email" VARCHAR(255) NOT NULL UNIQUE,
    "password" VARCHAR(255) NOT NULL,
    "is_active" BOOLEAN NOT NULL DEFAULT TRUE,
    "is_admin" BOOLEAN NOT NULL
);