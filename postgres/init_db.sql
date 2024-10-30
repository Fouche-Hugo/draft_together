CREATE TABLE champion (
    id SERIAL PRIMARY KEY,
    riot_id VARCHAR(255) NOT NULL UNIQUE,
    name VARCHAR(255) NOT NULL UNIQUE,
    default_skin_image_path VARCHAR(255) NOT NULL UNIQUE,
    centered_default_skin_image_path VARCHAR(255) NOT NULL UNIQUE
);