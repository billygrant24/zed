# Focused product contract

## Purpose

This fork is a focused, macOS-native code editor. It follows upstream Zed for
editor performance and ordinary editing features while excluding product
surfaces that turn the editor into an AI client, social workspace, or
cross-platform desktop application.

The contract is enforced at three levels:

1. Removed crates and assets cannot be linked accidentally.
2. The macOS editor dependency graph is checked for forbidden capabilities.
3. Product initialization, actions, menus, and updater entry points are checked
   for reintroduction during upstream rebases.

Run `./script/check-focused-product` after every upstream integration.

## Supported product

The supported desktop application is macOS only. The retained feature set
includes:

- text editing, syntax parsing, and language servers;
- terminals, tasks, Git, diagnostics, and debugging;
- themes, keymaps, and extensions that do not expose AI workflows;
- local projects;
- SSH projects and dev containers initiated from the macOS client.

This cut does not attempt to make the editor fully offline or remove all Zed
network services. Extension discovery/download, remote-server artifact
download, telemetry, and crash-reporting plumbing remain. They are independent
of the removed AI and collaboration products and can be evaluated in a separate
privacy-hardening patch series.

The desktop crates fail compilation on non-macOS targets. Linux, Windows,
FreeBSD, Flatpak, Snap, and Web desktop renderers, resources, packaging scripts,
and release workflows are removed.

## Excluded product capabilities

The following are not optional features. They are outside the product:

- built-in assistants, agents, agent protocols, skills, tools, prompts, and
  context servers;
- hosted or local model providers;
- AI chat, inline transformations, terminal assistance, edit prediction, and
  AI-generated commit messages;
- AI-related extension manifest fields and executable host routes;
- account sign-in and account UI;
- shared projects, collaborative buffers, following, contacts, channels, calls,
  screen sharing, and audio;
- application self-update and upstream release notifications.

The former application updater has been removed. The separate
`remote_server_download` crate only obtains a matching headless server archive
when establishing an SSH or dev-container connection. It cannot replace or
update the desktop application.

## Deliberate compatibility residue

Not every historical type bearing an excluded name should be deleted. The
following residue is allowed when it has no product entry point:

- versioned extension WIT definitions needed to load old non-AI extension
  binaries; current guest methods for removed AI capabilities return explicit
  unsupported errors;
- generic protocol and client data structures still shared by local projects,
  extension distribution, remote development, or upstream wire compatibility;
- redaction rules for third-party API-key-shaped text;
- test fixtures that describe external development-container or protocol data.

These exceptions do not permit actions, menus, settings, initialization calls,
host dispatch, or linked implementation crates for an excluded capability.

## Remote-host boundary

“macOS only” describes the desktop product, not every machine it can edit on.
A macOS editor that could only open local macOS files would lose a major editor
feature. Therefore:

- the graphical application and GPUI platform layer are macOS-only;
- the CLI distributed with the desktop app is macOS-only;
- the headless `remote_server` remains buildable for remote hosts;
- Linux code needed inside that headless server or its transport is retained;
- Linux/Windows desktop renderers, installers, resources, and release jobs are
  not retained.

This is the narrowest boundary that preserves useful SSH and dev-container
workflows.

## Upstream maintenance

Use stable upstream tags as explicit bases. Keep each removal category in a
separate commit so conflicts remain classifiable.

For a new upstream version:

```sh
git fetch upstream --tags
git branch baseline/vNEXT vNEXT
git switch focused/vCURRENT
git switch -c focused/vNEXT
git rebase --onto baseline/vNEXT baseline/vCURRENT
```

Replace `vCURRENT` and `vNEXT` with concrete versions. During conflict
resolution:

1. Inventory newly added workspace crates and Zed initialization calls.
2. Classify each upstream addition against this contract.
3. Port ordinary editor improvements.
4. Remove new AI, account, or collaboration entry points in the same rebase.
5. Do not restore upstream updater or release workflows.
6. Run the boundary check, formatting, the macOS editor check, the remote-server
   check, and a native application build.

If upstream couples a wanted editor feature to an excluded subsystem, extract
the smallest local interface needed by the editor. Do not reintroduce an entire
service graph to save a short-term merge conflict.

## Naming and data

The application is described as Focused Zed, but the executable, bundle
metadata, URL schemes, and application-data paths retain the upstream `Zed`
name. A complete rebrand would touch many unrelated paths and make every
upstream rebase harder. It should be a separate decision and patch series.

Until then, use a separate macOS user account or pass `--user-data-dir` when
testing beside upstream Zed if data separation is required.
