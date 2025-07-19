# Bevy 0.16.1 Getting Started - Claude Reference

This file serves as Claude's quick reference for Bevy's core philosophy and fundamental patterns when working on the "Fictiaverse" project. It summarizes the essential mindset needed to work effectively with Bevy's ECS architecture.

## Core Philosophy

**ECS-First Architecture**: Everything in Bevy revolves around the Entity Component System (ECS) pattern:
- **Entities** are unique identifiers for game objects
- **Components** are data structs attached to entities  
- **Systems** are functions that process entities with specific component combinations
- This separation of data and logic enables high performance through optimized memory access and automatic parallelization

**Code-First Approach**: Unlike traditional game engines with visual editors, Bevy prioritizes writing game logic directly in Rust code. This approach offers:
- Type safety and compile-time guarantees
- Superior refactoring capabilities
- Version control-friendly development
- Direct integration with Rust's ecosystem

**Modular Design**: Bevy is built as a collection of plugins that you can mix and match:
- Use only the features you need (headless servers don't need rendering)
- Replace any component you don't like with custom alternatives
- All engine features are implemented as plugins, treating user code and engine code equally

**App as Central Orchestrator**: Every Bevy program is an `App` that contains:
- A `World` for storing all game data (entities, components, resources)
- A `Schedule` for organizing systems and their execution order
- A `Runner` that controls the overall execution strategy

## Key Development Patterns

**Resources for Global State**: Use resources for singleton data like time, input state, or game configuration that needs global access.

**Queries for Data Processing**: Systems use queries to efficiently iterate over entities with specific component combinations, leveraging Bevy's optimized storage.

**Commands for Deferred Operations**: Use commands to spawn entities, modify components, or perform other operations that need to happen between system executions.

**Plugins for Organization**: Group related systems, resources, and startup logic into plugins for better code organization and reusability.

## Core Mental Model for Claude

When implementing features for "Fictiaverse", I should think in terms of:

1. **Data as Components** - Story text, player choices, game state as separate components
2. **Logic as Systems** - Story progression, UI updates, save/load as systems
3. **Global State as Resources** - Player progress, settings, AI configuration
4. **Organization as Plugins** - Separate concerns into focused plugins

## Quick Implementation Patterns

- **Story Node**: Entity with `StoryText`, `ChoiceOptions`, `StoryId` components
- **Player State**: Resource containing progress, history, settings
- **Choice Selection**: Event triggered by UI, processed by story system
- **Save/Load**: Commands for deferred file operations
- **AI Integration**: Tauri commands implemented as Bevy systems

## Key Files for Detailed Implementation

- `apps.md` - App setup patterns
- `ecs.md` - Component and system examples  
- `plugins.md` - Plugin organization
- `resources.md` - Global state management