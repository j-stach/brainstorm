
TODO: Switch framework root to `~/.cajal`

## Directory structure:
```
.brainstorm/
├── cajal/
│   └── hyper.toml
├── animi/
│   └── my_network/
│       ├── config.toml
│       ├── hyper.toml
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

