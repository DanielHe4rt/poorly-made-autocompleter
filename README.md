# Poorly Autocompleter

![img.png](/.github/img.png)

Poorly Autocompleter is a Rust-based command-line application designed to provide basic text autocompletion functionality. It leverages the `ratatui` library for terminal user interface rendering and `crossterm` for handling terminal events. The application features a text box for user input and a suggestion box that displays possible completions based on the current query.

## Features

- **Text Box**: Allows users to input text.
- **Suggestion Box**: Displays possible completions for the current query.
- **Focus Management**: Switches between different focus states (StandBy, Editing, Done).
- **Event Handling**: Processes keyboard events to update the application state.

## Dependencies

- `ratatui`
- `crossterm`
- `color_eyre`

## Usage

1. Clone the repository.
2. Run `cargo build` to compile the project.
3. Execute the binary to start the application.

## Development

To contribute to the project, follow these steps:

1. Fork the repository.
2. Create a new branch for your feature or bugfix.
3. Submit a pull request with a detailed description of your changes.

## License

This project is licensed under the MIT License.