# Atomic Update
Atomic Updates for any linux system, utilizing btrfs snapshots.

Written as a no-dependency Rust binary.

## What is this?
Normally when you update or install packages on a linux distro using the in-built package manager, the files on your system are updated in-place. This can lead to running programs needing to be restarted while you are using them, and power outages half way through an update can leave the machine in an unusable state, requiring the use of a live USB to restore.

Atomic Update allows you to apply updates or installs into a new btrfs snapshot, leaving the currently running system untouched until you next reboot.

## What do I need to use this?
Your system must be using btrfs on the root partition. That's it.

All usage of Atomic Update requires root permissions, or access to `sudo` or `doas`.

## Usage

### Initialize

Before making use of Atomic Update, you must initialize the system:

```bash
atomic-update init
```

Atomic Update will try and detect your distribution, your root partition, and your root btrfs subvolume name.

If you see an error message, you may need to set some of these yourself.

The config file lives at `/etc/atomic-update.conf`. The syntax is like so (using a Fedora system as an example):

```
PACKAGE_MANAGER dnf
UPDATE_COMMAND update
INSTALL_COMMAND install
YES_FLAG -y
ROOT_SUBVOLUME root
ROOT_PARTITION /dev/vda3
```

See [the config handler](https://github.com/Dvlv/atomic-update/blob/master/src/config_handler.rs#L26) for some examples.

### Updating
To update your system, run:

```bash
atomic-update update
```

This should run your package manager in a new snapshot, and if successful, set the snapshot as your next boot target.

### Installing
To install packages, for example `sshfs` and `pass`, run:

```bash
atomic-update install sshfs pass
```

### Arbitrary Commands
To run an arbitrary command, such as a custom install script for a package not in your repos, run:

```bash
atomic-update exec do_some_command
```

### Rolling Back
If you are unhappy with the results of your last update / install, you can roll back:

```bash
atomic-update rollback
```

This will set the previous snapshot to be loaded on next boot, and swap the current snapshot to its rollback target. This means rolling back twice in a row will undo the initial rollback. For example:

- System does not have `pass` -> State **A**
- `atomic-update install pass && reboot` -> State **B** (`pass` installed)
- `atomic-update rollback && reboot` -> State **A** (no `pass`)
- `atomic-update rollback && reboot` -> State **B** (`pass` installed again)

## Developing
The project is currently a Rust program with no external dependencies. This means building the project is as simple as:

- `git clone https://github.com/Dvlv/atomic-update.git`
- `cd atomic-update`
- `cargo build --release`

## TODOs
- [ ] Swap any magic strings for errors
- [ ] Allow Stacking
- [ ] Make use of config file's detections of root subvol / device
- [ ] Address all compiler warnings of unused Results / Options
- [ ] Better cleanup if a snapshot-swap step fails - try and manually undo any file moves
