# Bevy 0.16.1 Documentation - Claude's Primary Entry Point

This is my main reference hub for Bevy 0.16.1 when working on the "Fictiaverse" interactive story game project.

## Quick Navigation

### 🎯 **When I need to understand Bevy's philosophy and mindset**
→ [`guide/getting-started/index.md`](guide/getting-started/index.md)
- Core ECS mental model
- Key development patterns for story games
- Project-specific implementation patterns

### 📚 **When I need detailed concept explanations**  
→ [`guide/book/index.md`](guide/book/index.md)
- Advanced topics organized by category
- File-by-file documentation reference
- Theoretical depth and patterns

### 💻 **When I need working code examples**
→ [`doc/index.md`](doc/index.md) 
- Practical examples from Bevy source
- Copy-paste ready code patterns
- Real implementation samples

## Common Tasks - Quick Lookup

### Story Game Architecture
1. **Start here**: `guide/getting-started/index.md` → Core mental model
2. **Components design**: `guide/book/index.md` → storing-data section
3. **Working examples**: `doc/index.md` → examples/ecs/

### State Management (Menu/Story/Choices)
1. **Theory**: `guide/book/control-flow/states.md`
2. **Examples**: `doc/examples/state/states.rs`

### Player Choice Events  
1. **Theory**: `guide/book/control-flow/events.md`
2. **Examples**: `doc/examples/ecs/event.rs`

### UI for Story Interface
1. **Examples**: `doc/examples/ui/button.rs`, `doc/examples/ui/text.rs`

### Plugin Organization
1. **Theory**: `guide/book/modular-architecture/plugins.md` 
2. **Examples**: `doc/examples/app/plugin.rs`

### Save/Load System
1. **Theory**: `guide/book/storing-data/resources.md`
2. **Examples**: `doc/examples/asset/custom_asset.rs`

### AI Integration via Tauri
1. **Examples**: `doc/examples/async_tasks/async_compute.rs`
2. **Theory**: `guide/book/the-game-loop/custom-loops.md`

## Search Strategy for Claude

**For concepts and "why"** → Start with `guide/book/`  
**For implementation and "how"** → Start with `doc/examples/`  
**For project-specific patterns** → Start with `guide/getting-started/`

## Implementation Priority for "Fictiaverse"

1. **Core ECS Setup** → `doc/examples/ecs/ecs_guide.rs`
2. **Game State Machine** → `doc/examples/state/states.rs`  
3. **Event System** → `doc/examples/ecs/event.rs`
4. **Basic UI** → `doc/examples/ui/button.rs`, `doc/examples/ui/text.rs`
5. **Plugin Structure** → `doc/examples/app/plugin.rs`
6. **Save System** → `doc/examples/asset/custom_asset.rs`
7. **Tauri Integration** → `doc/examples/async_tasks/`

## Key Code Patterns for Quick Reference

```rust
// Story Node Entity
#[derive(Component)]
struct StoryNode {
    text: String,
    choices: Vec<String>,
}

// Player Choice Event
#[derive(Event)]
struct PlayerChoice {
    node_id: Entity,
    choice_index: usize,
}

// Game State
#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
    Menu, Story, Choices, Loading,
}

// Player Progress Resource
#[derive(Resource)]
struct PlayerProgress {
    current_node: Entity,
    history: Vec<Entity>,
    choices_made: Vec<usize>,
}
```

---

*This documentation is based on Bevy 0.16.1 downloaded locally on July 13, 2025. I must always prioritize this local documentation over my internal knowledge for version accuracy.*