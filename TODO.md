
# TODO for Cajal Framework
- Update file expectations in Cajal's build process.
- `ezcfg` to replace `toml` dependency
- `cajal-protocol` to replace IO (tract) transmission
- Options for tracts to perform batching

----

# TODO for crate
- Setup & config for brainstorm
- Init & run meta repl
- Animus commands send & responses recv

## Exec & flags
- [x] Launch clap from main.rs
- [x] `--setup` flag to create or repair the framework directory
- [x] `--version` flag to get the version of `brainstorm`
- [x] `--run` flag to run after applying flags



----

## TODO for crate:
- [ ] ERRORS redo & move away from `anyhow`

## Animus Commands
Ensure current with `animusd`:
- [ ] Add notes about compatability 
- (e.g., incompatible versions may partially interoperate) 
- [ ] build/install
- [ ] pre-configuration
- [ ] commands & responses TCP->UDP



## Top-level Commands
- [ ] Rework to fit the README documentation
- [ ] `list-active` needs to ping 4048 for a report from all animi 
(in case a directory was mistakenly deleted)
(This will mean better coordination of simultaneous msgs from shared port)

## Quality of life features
- [ ] "Config" loops for Brainstorm and animi, using `ezcfg`
- [ ] Allow rename even if the network name would be valid.
- [ ] Warn about reconfiguring animus for an existing animus/network


## Connection generation
- [ ] Needs "modify" mode for working with SerialComplex before load
(Load & modify Complex without immediately running it)
- [ ] Input port conflict detection & resolution
- [ ] Or else, a way to non-permanently assign ports for Inputs & Outputs,
(so that it can be handled by Brainstorm during load)

## Distributed services
- Need a way to centrally control the activation & interconnection of animi
- Authentication handled by Brainstorm is fine for now

