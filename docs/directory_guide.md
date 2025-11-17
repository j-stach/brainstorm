
TODO: Switch framework root to `~/.cajal`

## Directory structure:
```
.cajal/
├── neuro.cfg
├── brainstorm/
│   └── config.toml
├── animi/
│   └── my_network/
│       ├── service.cfg
│       ├── neuro.cfg
│       ├── run/
│       │   └── PID file, etc.
│       └── bin/
│           ├── my_network.nn
│           └── animusd-my_network
└── saved/
    └── my_network.nn
```

## `cajal/`
- "lib"-level user config for `hyper.toml`

## `animi/`
- contains config & save files and runtime/service data for each animus

## `saved/`
- repository for raw networks from which brainstorm can generate animi

