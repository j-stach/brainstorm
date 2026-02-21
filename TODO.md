
# TODO for crate

## Error Types
- [ ] ~Printable error types with `thiserror` to replace `anyhow`~ Not necessary
- [ ] Just need a section for common strings/display org

## Cleaning
- [ ] Misc `TODO` items in src comments

## Remote animi
- [ ] AnimusManager repl needs method for sending commands to remote animi using stored IP address
- [ ] Select needs to differentiate between local and remote
- [ ] `send_command` entirely new approach

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

