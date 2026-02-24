
# TODO for crate

## Remote animi
- [ ] ~`Select` needs to differentiate between local and remote, storing IP address~
- [x] `handle_command` & `send_command` searches for animus in local and remote
- [x] Differentiate between `send_local_command` and `send_remote_command`
- [x] `animusd` will need to store controller IP addr, 
- [x] or else `animusd` can auto-respond to the IP addr where the command was received from

## Error Types
- [ ] Just need a section for common strings/display org

## Cleaning
- [ ] Misc `TODO` items in src comments

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

