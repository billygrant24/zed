# Focused Zed

Focused Zed is a macOS-only fork of [Zed](https://github.com/zed-industries/zed)
for people who want a fast local editor without AI workflows, accounts, calls,
channels, or collaborative editing.

The binary and application-data paths are still named `Zed`. That keeps this
patch set small enough to rebase onto upstream releases. Do not install it over
an upstream Zed installation unless sharing the same settings and data directory
is intentional.

## Product boundary

- macOS is the only desktop target.
- AI agents, model providers, edit prediction, inline AI actions, and AI
  extension entry points are removed.
- Sign-in, cloud collaboration, channels, calls, screen sharing, and audio are
  removed.
- Application self-update is removed. Updates come from rebuilding this fork.
- SSH remotes and dev containers remain. Their headless remote server may run on
  Linux because that is useful to a macOS editor.
- Normal editor features remain: LSP, terminal, tasks, Git, debugger,
  diagnostics, themes, and non-AI extensions.

The enforceable product contract, compatibility exceptions, and upstream update
procedure are in [FOCUSED_PRODUCT.md](./FOCUSED_PRODUCT.md).

## Build on macOS

Install Xcode and Rust, then:

```sh
xcodebuild -downloadComponent MetalToolchain
cargo check -p zed --tests
cargo run -p zed
```

To create a local application bundle, use:

```sh
script/bundle-mac -d
```

Before committing an upstream merge, run:

```sh
./script/check-focused-product
cargo fmt --all -- --check
cargo check -p zed --tests
cargo check -p remote_server
```

## Licensing

This fork preserves Zed's licensing and attribution. Zed source code is licensed
primarily under GPL-3.0-or-later, with Apache-2.0 components where marked.
