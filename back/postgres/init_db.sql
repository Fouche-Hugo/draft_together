CREATE TABLE champion (
    id SERIAL PRIMARY KEY,
    riot_id VARCHAR(255) NOT NULL UNIQUE,
    name VARCHAR(255) NOT NULL UNIQUE,
    default_skin_image_path VARCHAR(255) NOT NULL UNIQUE,
    centered_default_skin_image_path VARCHAR(255) NOT NULL UNIQUE,
    positions jsonb NOT NULL DEFAULT '[]'::jsonb
);

CREATE TABLE draft (
    id SERIAL PRIMARY KEY,
    client_id uuid,
    blue_ban_1 INTEGER REFERENCES champion(id),
    blue_ban_2 INTEGER REFERENCES champion(id),
    blue_ban_3 INTEGER REFERENCES champion(id),
    blue_ban_4 INTEGER REFERENCES champion(id),
    blue_ban_5 INTEGER REFERENCES champion(id),
    red_ban_1 INTEGER REFERENCES champion(id),
    red_ban_2 INTEGER REFERENCES champion(id),
    red_ban_3 INTEGER REFERENCES champion(id),
    red_ban_4 INTEGER REFERENCES champion(id),
    red_ban_5 INTEGER REFERENCES champion(id),
    blue_1 INTEGER REFERENCES champion(id),
    blue_2 INTEGER REFERENCES champion(id),
    blue_3 INTEGER REFERENCES champion(id),
    blue_4 INTEGER REFERENCES champion(id),
    blue_5 INTEGER REFERENCES champion(id),
    red_1 INTEGER REFERENCES champion(id),
    red_2 INTEGER REFERENCES champion(id),
    red_3 INTEGER REFERENCES champion(id),
    red_4 INTEGER REFERENCES champion(id),
    red_5 INTEGER REFERENCES champion(id)
);