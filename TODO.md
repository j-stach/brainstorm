
# TODO for crate

## Cleaning
- [ ] **DRY** & cleaning
- [ ] Reorganize modules
- [ ] Misc `TODO` items in src comments
- [ ] Break up `group.rs` with helper modules & functions
- [ ] Other helper functions 
- [ ] **Error types**

## Remote animi
- [x] `add-remote` meta repl command with `ip_addr` and `animus` fields
- [x] `add-remote` queries animus by name at port 4048 of the IP address (animi must be started locally)
- [x] Records name and IP addr in file in `remote` dir
- [ ] AnimusManager repl needs method for sending commands to remote animi using stored IP address

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
