# Development

The mobile crate defines the entrypoint for the mobile app along with any assets, components and dependencies that are specific to mobile builds. The mobile crate starts out something like this:

```
mobile/
├─ assets/ # Assets used by the mobile app - Any platform specific assets should go in this folder
├─ src/
│  ├─ main.rs # The entrypoint for the mobile app.
├─ Cargo.toml # The mobile crate's Cargo.toml - This should include all mobile specific dependencies
```

## Dependencies
This crate will only be included in the mobile build, so you should add all mobile specific dependencies to this crate's [Cargo.toml](../Cargo.toml) file instead of the shared [ui](../ui/Cargo.toml) crate.

### Serving Your Mobile App

Mobile platforms are shared in a single crate. To serve mobile, you need to explicitly set your target device to `android` or `ios`:

```bash
dx serve --platform android
```