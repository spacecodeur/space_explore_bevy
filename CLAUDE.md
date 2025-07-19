# Bevy Training - Learning Projects

## Project Overview
Collection of independent mini-projects for learning game development with Bevy. Each project in `projects/` focuses on specific Bevy concepts and features. Projects share common utilities through the `shared/` workspace.

## Project Structure
- **Main Directory**: Collection of independent Bevy learning projects
- **Workspace Architecture**: Cargo workspace with shared dependencies and utilities
- **Project Organization**:
  - `projects/` - Individual learning projects (each with its own Cargo.toml)
  - `shared/bevy_common/` - Common Bevy utilities and prelude
  - `docs/bevy_0-16-1/` - Local Bevy documentation

## Technology Stack
- **Game Engine**: Bevy 0.16.1 (latest stable)
- **Build System**: Cargo workspace for managing multiple projects
- **Shared Dependencies**: Workspace-level dependency management

## Development Guidelines

### Creating New Projects
1. Create new directory in `projects/`
2. Add minimal `Cargo.toml` with workspace dependencies
3. Use `bevy_common` for shared utilities
4. Focus each project on specific learning objectives

### Code Conventions
- **Component Naming**: PascalCase with descriptive suffixes (e.g., `PlayerComponent`, `MovementEvent`)
- **File Organization**: Standard Rust module structure
- **Project Focus**: Each project should explore specific Bevy features

## Documentation Reference

### Documentation Version Information
- **Version**: Bevy 0.16.1 (latest stable)
- **Location**: Local documentation available at `docs/bevy_0-16-1/`
- **CRITICAL**: Always consult local documentation FIRST for any Bevy development task. Do NOT rely on potentially outdated internal knowledge.

### Local Bevy Documentation Usage
- **Entry Point**: Start with `docs/bevy_0-16-1/index.md` for navigation
- **Priority**: Local documentation MUST be consulted before implementing any Bevy features
- **Key Sections**:
  - **Getting Started**: `docs/bevy_0-16-1/guide/getting-started/index.md`
  - **ECS Guide**: `docs/bevy_0-16-1/guide/book/index.md`
  - **Examples**: `docs/bevy_0-16-1/doc/examples/`
  - **API Reference**: `docs/bevy_0-16-1/doc/index.md`

### Development Workflow
1. **Research Phase**: Always check local documentation first
2. **Implementation**: Use examples from `docs/bevy_0-16-1/doc/examples/`
3. **Reference**: Consult API docs for specific components and systems
4. **Testing**: Build and test within the workspace context