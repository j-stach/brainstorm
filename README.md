
>*"All of mankind was united in celebration; we marvelled at our own magnificance as we gave birth to A.I."* <br>
> -- Morpheus, *The Matrix*

# Brainstorm
This is a tool for animating and managing neural networks created using the [Cajal framework](https://github.com/j-stach/cajal).

----

## Set Up
1. Use Linux.
2. [Install Rust.](https://www.rust-lang.org/tools/install)
3. Ensure `~/.cargo/bin/` is in `PATH`.
4. Build from source:
```
cargo install brainstorm
```
5. Generate the framework directory structure:
```
brainstorm --setup
```
This command will preserve `~/.cajal/hyper.toml` and any other files you have already set up. <br>

### Add networks
Brainstorm manages spiking neural networks created with [`cajal`](https://crates.io/crates/cajal), which use the `.nn` file extension.
Add raw networks to the `~/.cajal/saved/` directory to make them discoverable through Brainstorm.

### Configure
- Default library hyperparameters can be reconfigured in `~/.cajal/hyper.toml`
- Animus-specific hyperparameters can be configured in `~/.cajal/animi/my_network/hyper.toml`
- Animus runtime can be configured in `~/.cajal/animi/my_network/config.toml` before it is loaded

----

## Use
To open the REPL, run:
```
brainstorm
```

### Commands
These commands are used to interact with network files and animi at the top level.
| Command | Purpose |
|---------|---------|
|`animate network=my_network`| Generate a new animus for `my_network.nn` |
|`load animus=my_network`| Launch (activate) the animus named `my_network` |
|`select animus=my_network`| Enter an animus-specific REPL for `my_network` to issue commands |
|`list-networks`| List all `.nn` files in `~/.cajal/saved/` |
|`list-all`| List all animi saved in `~/.cajal/animi/` |
|`list-active`| List all animi that are active and listening for commands |
|`quit` or `exit`| Close the REPL and exit Brainstorm |

### Animus Commands
These commands are used to control individual animi via the protocol from [`animusd_lib`](https://docs.rs/animusd/latest/animusd_lib).
| Command | Purpose |
|---------|---------|
|`version`| Retrieve the version of `animusd` used by this animus |
|`name`| Retrieve the name of the network managed by this animus |
|`list-structures`| List the names of all sub-structures in the network |
|`wake`| Start the neurotransmission runtime to begin processing signals |
|`sleep`| Ignore new stimuli and spin down neurotransmission\* |
|`status`| Display asleep/awake status |
|`save`| Serialize and overwrite the network |
|`terminate`| Shut down the animus service and return to the top-level REPL |
|`back`| Return to the top-level REPL without terminating |
\* *Note: This does not immediately cease neurotransmission. Activity may continue for some time, even in absence of stimulation (signal input).*

----

# Development
Brainstorm is a work-in-progress. <br>
Active priorities are tracked in [`TODO.md`](/TODO.md).

