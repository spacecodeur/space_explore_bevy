# Bevy 0.16.1 Book Documentation Index

Advanced concepts documentation organized by topic. Each section contains detailed explanations and patterns.

## Directory Structure
- **intro/** - ECS fundamentals and Bevy philosophy  
- **storing-data/** - Data management in ECS
- **control-flow/** - Logic control and system execution
- **the-game-loop/** - App lifecycle and scheduling
- **modular-architecture/** - Plugin system and organization
- **development-practices/** - Development tools
- **releasing-projects/** - Optimization and deployment

---

## 1. Introduction (intro/)

### Core Concepts
- **[what-is-bevy.md](intro/what-is-bevy.md)** - Engine philosophy, licensing, when to use/avoid Bevy
- **[the-three-letters.md](intro/the-three-letters.md)** - ECS fundamentals: Entities, Components, Systems
- **[the-next-three-letters.md](intro/the-next-three-letters.md)** - Extended concepts: Resources, Queries, Commands
- **[apps-worlds.md](intro/apps-worlds.md)** - High-level containers: App and World

### Key Takeaways for Interactive Story Games
- ECS-first architecture for modular story elements
- Code-first approach for complex narrative logic
- Plugin system for feature organization

---

## 2. Data Storage (storing-data/)

### Entity and Component Management
- **[entities-components.md](storing-data/entities-components.md)** - Entity spawning, component patterns, design principles
- **[required-components.md](storing-data/required-components.md)** - Component dependencies with `#[require()]`
- **[relations.md](storing-data/relations.md)** - Entity hierarchies, parent-child relationships

### Data Access Patterns
- **[world.md](storing-data/world.md)** - World container, archetype storage, direct access
- **[queries.md](storing-data/queries.md)** - Query system for data retrieval
- **[resources.md](storing-data/resources.md)** - Global singleton data management
- **[local-system-param.md](storing-data/local-system-param.md)** - System-local state with `Local<T>`
- **[disabling-entities.md](storing-data/disabling-entities.md)** - Entity enable/disable patterns

### Interactive Story Applications
- Story nodes as entities with narrative components
- Player progress and choices as resources
- Hierarchical story structure with relations
- Local state for UI systems and transitions

---

## 3. Control Flow (control-flow/)

### System Management
- **[systems.md](control-flow/systems.md)** - System parameters, exclusive systems, piping, one-shot execution
- **[run-conditions.md](control-flow/run-conditions.md)** - Conditional system execution with `.run_if()`

### Data Flow and Events
- **[commands.md](control-flow/commands.md)** - Deferred world modifications, custom commands
- **[events.md](control-flow/events.md)** - Event system for communication
- **[observers.md](control-flow/observers.md)** - Event observation patterns

### State and Change Management
- **[states.md](control-flow/states.md)** - Finite state machines, sub-states, state transitions
- **[change-detection.md](control-flow/change-detection.md)** - Reactive programming with `Added<T>`, `Changed<T>`
- **[hooks.md](control-flow/hooks.md)** - Component lifecycle hooks

### Error Handling
- **[handling-errors.md](control-flow/handling-errors.md)** - Error handling in systems

### Story Game Applications
- Game states for menu/playing/paused phases
- Events for player choices and story progression
- Change detection for UI updates and save triggers
- Commands for spawning/despawning story elements

---

## 4. Game Loop (the-game-loop/)

### App Architecture
- **[app.md](the-game-loop/app.md)** - App container configuration, runners, world access
- **[schedules.md](the-game-loop/schedules.md)** - System organization, execution order, custom schedules

### Timing and Updates
- **[fixed-time.md](the-game-loop/fixed-time.md)** - Fixed timestep updates
- **[rendering-frames.md](the-game-loop/rendering-frames.md)** - Pipelined rendering architecture

### Input and Display
- **[input-and-windowing.md](the-game-loop/input-and-windowing.md)** - Input handling, window management
- **[custom-loops.md](the-game-loop/custom-loops.md)** - Custom game loop patterns

### Story Game Applications
- Separate schedules for narrative logic vs UI updates
- Fixed timestep for consistent story progression
- Input handling for choice selection
- Custom loops for AI integration via Tauri

---

## 5. Modular Architecture (modular-architecture/)

### Code Organization
- **[plugins.md](modular-architecture/plugins.md)** - Plugin system, configuration, lifecycle, PluginGroups
- **[project-organization.md](modular-architecture/project-organization.md)** - File structure, crate organization, visibility patterns

### Story Game Applications
- Separate plugins for: story generation, choice handling, save/load, UI, Tauri integration
- Domain-based organization (not by component type)
- Resource-based configuration for story settings

---

## 6. Development Practices (development-practices/)

### Productivity Tools
- **[debugging.md](development-practices/debugging.md)** - Debugging techniques and tools
- **[testing.md](development-practices/testing.md)** - Testing strategies for Bevy apps
- **[dev-tools.md](development-practices/dev-tools.md)** - Development tooling

### Performance and Workflow
- **[fast-compiles.md](development-practices/fast-compiles.md)** - Compilation optimization
- **[hot-reloading.md](development-practices/hot-reloading.md)** - Hot reloading for rapid iteration
- **[linting.md](development-practices/linting.md)** - Code quality tools

---

## 7. Releasing Projects (releasing-projects/)

### Optimization
- **[optimizing-performance.md](releasing-projects/optimizing-performance.md)** - Runtime performance optimization
- **[optimizing-binary-size.md](releasing-projects/optimizing-binary-size.md)** - Binary size reduction
- **[profiling.md](releasing-projects/profiling.md)** - CPU/GPU profiling with Tracy, Chrome tracing, platform tools

### Distribution
- **[libraries-for-bevy.md](releasing-projects/libraries-for-bevy.md)** - Creating reusable Bevy libraries

## Quick File Lookup

### Core Concepts
- **ECS Basics** → `intro/the-three-letters.md`
- **Resources/Queries/Commands** → `intro/the-next-three-letters.md`
- **Entity Design** → `storing-data/entities-components.md`
- **State Machines** → `control-flow/states.md`
- **Events** → `control-flow/events.md`
- **Plugin Architecture** → `modular-architecture/plugins.md`

### Advanced Topics
- **Change Detection** → `control-flow/change-detection.md`
- **System Scheduling** → `the-game-loop/schedules.md`
- **Performance** → `releasing-projects/profiling.md`