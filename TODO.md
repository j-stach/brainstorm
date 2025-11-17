
## TODO for crate:
- [ ] Switch framework root to `~/.cajal`

## Readme
### Tutorial:
- [x] Install/build
- [x] Load network & start animus
- [x] Save state & shutdown

## Quality of life features
- [ ] Config loop
- [ ] Allow rename even if the network name would be valid.
- [ ] Warn about reconfiguring animus for an existing animus/network


## Connection generation
- [ ] Brainstorm needs "modify" mode for working with SerialComplex before load
- [ ] Brainstorm needs Input port conflict detection & resolution

## XX
- [ ] Brainstorm needs to load & modify Complex without immediately running it
- [ ] Brainstorm needs to query complex for connection-related info, at minimum
- [ ] Brainstorm should be able to run PhantomLimb instances using animus-like executable


## Distributed services
- [ ] Connecting to Animi on other computers, with authentication (lobes) using SSH (prompt for password)
- [ ] Store connection & authentication in animus config, allowing you to control remotely without ssh

## Errors
- [ ] Dedicated thiserror types for config, setup, command, etc.

