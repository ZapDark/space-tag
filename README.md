# Description

This is a multiplayer game of tag made using `godot` as the game engine, `godot-rust` in order to write game logic in rust, and `spacetimedb` to manage the networking, server side compute, and database.

I made this project to experiment with `spacetimedb` and understand it's strengths and limits.

# Docs

- [godot-rust docs](https://godot-rust.github.io/book/index.html)
- [spacetimedb docs](https://spacetimedb.com/docs)

# TODO

- [ ] Architect tables in `spacetimedb` module
- [ ] Implement tables in `spacetimedb` module
- [ ] Architect `spacetimedb` reducers
- [ ] Implement `spacetimedb` reducers
- [ ] Make this a command `spacetime publish --project-path server space-tag`
- [ ] Make this a tail command `spacetime logs space-tag`
- [ ] Make this a command `spacetime generate --lang rust --out-dir rust/src/module_bindings --project-path server`
- [ ] Implement authorization
