
- Config loop
- Allow rename even if the network name would be valid.
- Warn about reconfiguring animus for an existing animus/network


## More capabilities
- [ ] Phantom limb fuzzing

## Distributed services
- [ ] Connecting to Animi on other computers, with authentication (lobes) using SSH (prompt for password)
- [ ] Store connection & authentication in animus config, allowing you to control remotely without ssh

## Errors
- [ ] Dedicated thiserror types for config, setup, command, etc.

## Directory structure:

```
.brainstorm/
├── cajal/
│   └── hyper.toml
├── animi/
│   └── network_01/
│       ├── config.toml
│       ├── hyper.toml
│       ├── run/
│       │   └── PID file, etc.
│       └── bin/
│           ├── network_01.nn
│           └── animusd-network_01
└── saved/
    └── network_01.nn
```
