# Draft Together

Simple website to prepare [League of Legends](https://www.leagueoflegends.com) drafts with your friends, no account needed, just share the link and you are good to go !

Draft together is currently accessible online [here](https://draft-together.fouche-hugo.fr/).

This app is not affiliated with Riot Games.

## Features

- Create a new draft easily
- Share it with a simple link
- Updated in real time
- On new League of Legends patch, the app updates itself automatically using Riot Data Dragon
- Move champions using drag & drop
- Filter champions by their positions played

## Tech stack

Built using :

- Rust with Axum for the backend
- Nuxt for the frontend
- Deployement via Docker Compose

## Development

The easiest way to start a development environment is to use Docker Compose:

1. Clone the repo

    ```bash
    git clone https://github.com/Fouche-Hugo/draft_together.git
    cd draft_together
    ```

2. Create a `.env` file with the following content and choose your database password

    ```env
    POSTGRES_PASSWORD={YOUR PASSWORD}
    ```

3. Start the dev environment

    ```bash
    docker compose up -d
    ```

The first time the app is started, data from riot server should be downloaded (~2Go), that's why the backend could take some time to load.
You can see the logs from the application using:

```bash
docker compose logs -f
```
