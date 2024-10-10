# laika-th (tic tac toe)
[![build](https://github.com/sneakycrow/laika-th/actions/workflows/build.yml/badge.svg)](https://github.com/sneakycrow/laika-th/actions/workflows/build.yml)

This application serves a tic tac toe game. The API acts as the opponent and the front-end allows for the user to play against it

## Requirements
  - Cargo
  - Node
  - Justfile (optional)
  - Node

_Note: The easiest way to run this app is via the justfile task runner. But, one can copy the commands from the justfile and run them
directly as well_

## Development

1. Initialize the project with `just init`, this will setup the environment and install dependencies
2. Run the API locally with `just dev-api`
3. Run the Frontend locally with `just dev-web` (you may need to run this in a separate terminal)
