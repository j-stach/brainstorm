
# TODO for crate
- [ ] Misc `TODO` items in src comments
- [ ] `await_response` method to return response instead of printing directly
- [ ] Other helper functions
- [ ] Errors cleaned up

## Connections
- [ ] ~Brainstorm `Link` command to auto-connect tracts between individual (ungrouped) animi~

## "Unchecked link (for Sensor & Motor)"
- `Link` output and input to PhantomLimb `Sensor` and `Motor` without using 4048
(4048 should be reserved for Brainstorm <-> Animus communication only)

Within the animus repl:
- [x] For `Sensor`, Brainstorm should allow query for raw port address (without ReceiverInfo),
and the application containing the sensor should permit manual entry of that address
during connection.
- [x] For `Motor`, Brainstorm should take in a raw port address and connect using that alone.

## Remote animi
- [ ] Divide `animi` directory with `local` and `remote` sub-dirs
- [ ] `add-remote` command with `ip_addr` and `animus` fields
- [ ] `remove-remote` command to remove animi safely/with warnings
- [ ] `add-remote` queries animus by name at port 4048 of the IP address (animi must be started locally)
- [ ] `add-remote` records name and IP addr in file in `remote` dir
- [ ] Sending IP commands to remote dirs
- [ ] Link singular output by name, for motor ports and other manual alignment

## Group
- [ ] Break up `group.rs` with helper modules & functions

----

# Quality of life features

## Config REPL
- "Config" loops for Brainstorm and animi, using `ezcfg`
- Warn about reconfiguring animus for an existing animus/network

## Logging
- Reintegrate Mindreader to animusd
- Configure & interact with mindreader through sub-repl

## TUI
TBD.

## Misc
- Add `Query` action back for explicit use in top-level repl
- Add `this-ip` command to meta repl to quickly get IP addr used
- AutoLink tracts between animi without grouping
