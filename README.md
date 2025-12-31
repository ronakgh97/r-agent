# R-Agent

R-Agent is an experimental project designed to showcase and test the capabilities of my custom library, `my_lib`. This library is currently under development as part of the [Pokebrains](https://github.com/ronakgh97/Pokebrains) project.

## Features
- Demonstrates the integration of `my_lib` in a real-world application.
- Supports piping, regex, and all traditional Unix operations to maintain an old-school, not modern slop.

## Command-Line Arguments
R-Agent supports the following command-line arguments:

- **`--config <file>`**: Specifies the configuration file to use. This file contains settings and parameters for the agent.
- **`--session <name>`**: Defines the session name. Sessions allow you to maintain context across multiple commands.
- **`<task>`**: The task or command you want the agent to perform.

## Usage

``bash
cat Cargo.toml | ragent run "explain the dependencies" --config qwen_qwen3-coder-free.toml --session my_session
``

``bash
rg "TODO" | ragent run "explain the todos" --config qwen_qwen3-coder-free.toml --session my_session
``

## Note
`my_lib` is not yet published to crates.io. Frequent updates are made locally to ensure rapid iteration without the constraints of versioning and publishing.

## License
This project is experimental and does not yet have a formal license.
