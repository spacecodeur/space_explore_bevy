# Bevy Training

A collection of independent mini-projects for learning game development with Bevy 0.16.1.

## Project Structure

```
space_explore_bevy/
├── projects/           # Individual Bevy learning projects
├── shared/            # Common utilities and components
│   └── bevy_common/   # Shared Bevy prelude and utilities
├── docs/              # Local Bevy documentation
└── README.md
```

## Getting Started

Each project in the `projects/` directory is an independent Bevy application focused on learning specific game development concepts.

### Adding a New Project

1. Create a new directory in `projects/`
2. Add a `Cargo.toml` with `bevy = { workspace = true }`
3. Use `bevy_common` for shared utilities
4. The workspace will automatically include your project

## Learning Projects (Progressive Difficulty)

1. `player_movement_2d` - Basic 2D player movement controls

### Documentation

Local Bevy 0.16.1 documentation is available in `docs/bevy_0-16-1/` for offline reference during development.