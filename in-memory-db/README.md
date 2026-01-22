# In-Memory Database Workspace

Monorepo-style workspace featuring:
- Custom derive macros (`database_derive`) to generate CRUD and indexing boilerplate for entities.
- A generic in-memory database with pluggable indexing and timestamp traits (`lib/database`).
- An Actix-Web service exposing workflow CRUD endpoints backed by the in-memory store and a JSON file for persistence.
- A lightweight runner crate for experimentation.

Highlights macro usage, trait-driven design, and web API wiring over a homegrown data layer.
