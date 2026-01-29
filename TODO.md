
# TODO for framework

## Misc
- [ ] Add `Query` action back for explicit use in top-level repl
- [ ] Add `this-ip` command to meta repl to quickly get IP addr used

## Animus
- [ ] Animus had new actions added -- implement?

## Group
- [ ] Command to group animi by name into a `Group` 
- [ ] Group command repl loop (`select` for `system`)
- [ ] Group command to auto-link and animi in system,
make sure to report tracts that are doubled, or unmatched, before linking
- [ ] `group` directory in filesystem

## Remote animi
- [ ] Divide `animi` directory with `local` and `remote` sub-dirs
- [ ] `add-remote` command with `ip_addr` and `animus` fields
- [ ] `remove-remote` command to remove animi safely/with warnings
- [ ] `add-remote` queries animus by name at port 4048 of the IP address (animi must be started locally)
- [ ] `add-remote` records name and IP addr in file in `remote` dir
- [ ] Sending IP commands to remote dirs
- [ ] Link singular output by name, for motor ports and other manual alignment

----

# Quality of life features

## Config REPL
- "Config" loops for Brainstorm and animi, using `ezcfg`
- Warn about reconfiguring animus for an existing animus/network

## Distributed services
- Need a way to centrally control the activation & interconnection of animi

## Logging
- Reintegrate Mindreader to animusd
- Configure & interact with mindreader through sub-repl

## TUI
TBD.

