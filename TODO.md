
# TODO for crate

## Misc
- [ ] Reorganize modules
- [ ] Misc `TODO` items in src comments
- [ ] Break up `group.rs` with helper modules & functions
- [ ] Other helper functions 
- [ ] Errors cleaned up

## Remote animi
- [x] Divide `animi` directory with `local` and `remote` sub-dirs
- [x] Update file helper functions to use `local` and `animus` directories
- [ ] `add-remote` command with `ip_addr` and `animus` fields
- [ ] `remove-remote` command to remove animi safely/with warnings
- [ ] `add-remote` queries animus by name at port 4048 of the IP address (animi must be started locally)
- [ ] `add-remote` records name and IP addr in file in `remote` dir
- [ ] Sending IP commands to remote dirs
- [x] Link singular output by name, for motor ports and other manual alignment

----

# Quality of life features

## Config REPL
- "Config" loops for Brainstorm and animi, using `ezcfg`
- Warn about reconfiguring animus for an existing animus/network

## Logging
- Reintegrate Mindreader to animusd
- Configure & interact with mindreader through sub-repl

## Security & authentication
- Enable brainstorm -> animus authentication for internal security

## TUI
TBD.

## Misc
- Add `Query` action back for explicit use in top-level repl
- Add `this-ip` command to meta repl to quickly get IP addr used
- AutoLink tracts between animi without grouping
