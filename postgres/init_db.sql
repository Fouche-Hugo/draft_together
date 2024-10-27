CREATE TABLE champion (
    id SERIAL PRIMARY KEY,
    riotId VARCHAR(255),
    name VARCHAR(255),
    default_skin_image_path VARCHAR(255),
    centered_default_skin_image_path VARCHAR(255)
);