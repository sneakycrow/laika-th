# laika-th (tic tac toe)

This application serves a tic tac toe game. The API acts as the opponent and the front-end allows for the user to play against it

## Requirements
  - Cargo
  - Node
  - Justfile (optional)
  - Node

_Note: The easiest way to run this app is via the justfile task runner. But, one can copy the commands from the justfile and run them
directly as well_

## Development

1. Initialize the project with `just init`
2. Run the API locally with `just dev-api`
3. Run the Frontend locally with `just dev-web` (optional*)
_* You only need to do this if you want to develop the frontend. Alternatively, you can run `just build-web` and then turn on the `EMBED_SPA` feature to embed the frontend into the API_

## Deployment

The API serves the frontend, but the frontend can be configured to run indepdently.
