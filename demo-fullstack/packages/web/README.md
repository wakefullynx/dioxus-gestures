# Development

The web crate defines the entrypoint for the web app along with any assets, components and dependencies that are specific to web builds. The web crate starts out something like this:

```
web/
├─ assets/ # Assets used by the web app - Any platform specific assets should go in this folder
├─ src/
│  ├─ main.rs # The entrypoint for the web app.
├─ Cargo.toml # The web crate's Cargo.toml - This should include all web specific dependencies
```

## Dependencies
Since you have fullstack enabled, the web crate will be built two times:
1. Once for the server build with the `server` feature enabled
2. Once for the client build with the `web` feature enabled

You should make all web specific dependencies optional and only enabled in the `web` feature. This will ensure that the server builds don't pull in web specific dependencies which cuts down on build times significantly.

### Serving Your Web App

You can start your web app with the following command:

```bash
dx serve
```
