# Development

The desktop crate defines the entrypoint for the desktop app along with any assets, components and dependencies that are specific to desktop builds. The desktop crate starts out something like this:

```
desktop/
├─ assets/ # Assets used by the desktop app - Any platform specific assets should go in this folder
├─ src/
│  ├─ main.rs # The entrypoint for the desktop app.
├─ Cargo.toml # The desktop crate's Cargo.toml - This should include all desktop specific dependencies
```

## Dependencies
This crate will only be included in the desktop build, so you should add all desktop specific dependencies to this crate's [Cargo.toml](../Cargo.toml) file instead of the shared [ui](../ui/Cargo.toml) crate.

### Serving Your Desktop App

You can start your desktop app with the following command:

```bash
dx serve
```
