# TODO

- [ ] Index `MainScreen.textures: HashMap<String, (Texture, Rect)>` with something else than a `String`
- [ ] Refactor errors:
  - [ ] Either have a single error type.
  - [ ] Or have a overhead error type that can come from any of the others.
- [ ] Change GameScreen.game from `Option<Game>` to some sort of game pointer and store the `Game` in `Gui`.
