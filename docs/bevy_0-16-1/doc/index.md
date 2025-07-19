# Bevy 0.16.1 API Reference - Claude's Quick Access Guide

This index helps me (Claude) efficiently locate specific Bevy API elements, code examples, and implementation patterns when developing the "Fictiaverse" interactive story game.

## Source Code Structure

### Main Entry Point
- **`src/lib.rs`** - Main crate re-exports and feature documentation
- **`Cargo.toml`** - Package metadata (Rust 1.85.0+, Edition 2024)

### Key Directories for "Fictiaverse"

#### Core ECS Examples (`examples/ecs/`)
Essential for story game architecture:

- **`ecs_guide.rs`** - Comprehensive ECS overview with practical examples
- **`event.rs`** - Event system for player choices and story progression
- **`hierarchy.rs`** - Entity relationships for story node hierarchies  
- **`relationships.rs`** - Advanced entity relationships
- **`change_detection.rs`** - Reactive updates for UI and story state
- **`run_conditions.rs`** - Conditional system execution
- **`system_param.rs`** - Custom system parameters
- **`observers.rs`** - Event observation patterns
- **`one_shot_systems.rs`** - Triggered systems for story events

#### State Management (`examples/state/`)
Critical for game flow:

- **`states.rs`** - Basic state machine implementation
- **`sub_states.rs`** - Nested states for complex game phases
- **`computed_states.rs`** - Derived states from other data
- **`custom_transitions.rs`** - Custom state transition logic

#### UI Examples (`examples/ui/`)
For narrative interface:

- **`button.rs`** - Interactive choice buttons
- **`text.rs`** - Text rendering for story content
- **`scroll.rs`** - Scrollable story text
- **`flex_layout.rs`** - UI layout patterns
- **`directional_navigation.rs`** - Keyboard navigation

#### App Architecture (`examples/app/`)
For integration structure:

- **`plugin.rs`** - Plugin creation patterns
- **`plugin_group.rs`** - Plugin organization
- **`headless.rs`** - Server-side story generation
- **`custom_loop.rs`** - Custom game loop for AI integration

#### Asset Management (`examples/asset/`)
For story content:

- **`asset_loading.rs`** - Loading story data files
- **`custom_asset.rs`** - Custom asset types for story content
- **`hot_asset_reloading.rs`** - Live story content updates

#### Input Handling (`examples/input/`)
For player interaction:

- **`keyboard_input.rs`** - Keyboard choice selection
- **`mouse_input.rs`** - Mouse/touch interaction
- **`text_input.rs`** - Player name/custom input

## Key Examples by Category

### Core Architecture
- **ECS Guide** → `examples/ecs/ecs_guide.rs`
- **Events** → `examples/ecs/event.rs`  
- **State Machines** → `examples/state/states.rs`
- **Hierarchies** → `examples/ecs/hierarchy.rs`

### UI & Interaction
- **Buttons** → `examples/ui/button.rs`
- **Text Display** → `examples/ui/text.rs`
- **Input Handling** → `examples/input/keyboard_input.rs`

### Organization
- **Plugins** → `examples/app/plugin.rs`
- **Custom Assets** → `examples/asset/custom_asset.rs`
- **Async Tasks** → `examples/async_tasks/async_compute.rs`

---

*This API reference is based on Bevy 0.16.1 source code copied locally on July 13, 2025. I must prioritize these local examples and patterns over my internal knowledge for accuracy and version compatibility.*