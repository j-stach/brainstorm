
>*"All of mankind was united in celebration; we marvelled at our own magnificance as we gave birth to A.I."* <br>
> -- Morpheus, *The Matrix*

# Brainstorm
This is a tool for animating and managing spiking neural networks created using the [Cajal framework](https://github.com/j-stach/cajal).


## Installation
1. Use Linux.
2. [Install Rust.](https://www.rust-lang.org/tools/install)
3. Make sure `~/.cargo/bin/` is in your `PATH` environment variable.
4. Build Brainstorm from source:
```
$ cargo install brainstorm
```
5. To generate the necessary files and directory structure, run:
```
$ brainstorm --setup
```
This command will preserve `~/.cajal/hyper.toml` and any other files you have already set up.
(See the [setup guide](/docs/setup_guide.md) for more info.)


## Use
### Add networks
Brainstorm manages spiking neural networks created with `cajal`, which use the `.nn` file extension.
Add raw networks to the `~/.cajal/saved/` directory to make them discoverable through Brainstorm.

### Run
To open the REPL, run:
```
$ brainstorm
```

### Commands
These commands are used to interact with network files and animi at the top level.
| Command | Purpose |
|---------|---------|
|`list-networks`| List all `.nn` files in `~/.cajal/saved/` |
|`animate network=my_network`| Generate a new animus for `~/.cajal/saved/my_network.nn` |
|`list-all`| List all animi saved in `~/.cajal/animi/` |
|`load animus=my_network`| Launch the animus for `my_network` |
|`list-active`| List all active (loaded) animi |
|`select animus=my_network`| Enter the animus-specific REPL for `my_network` to issue commands |
|`quit` or `exit`| Close the REPL and exit Brainstorm |

### Animus commands
These commands are used to control individual animi by implementing the protocol from `animusd_lib`.
| Command | Purpose |
|---------|---------|
|`name`| Retrieve the name of the network managed by this animus |
|`version`| Retrieve the version of `animusd` used by this animus |
|`list-structures`| List the names of all sub-structures in the network |
|`wake`| Start the neurotransmission runtime to begin processing signals |
|`sleep`| Stop processing new inputs and spin down activity |
|`status`| Display asleep/awake status |
|`save`| Serialize and overwrite the network at `~/.cajal/animi/my_network/bin/my_network.nn` |
|`terminate`| Shut down the animus service and return to the top-level REPL |
|`back`| Return to the top-level REPL without terminating |


## Other uses
*TBD: Distributed computing*


## Configuration 
The Cajal framework has configurable parameters for many of its components, and these influence how networks operate and interact with Brainstorm.

### Core library
Default hyperparameters for neurotransmission can be configured in `~/.cajal/hyper.toml` if the parameters provided by the core library are insufficient for your use case.

### Animus-specific configuration
Animus-specific hyperparameters can be configured in `~/.cajal/animi/my_network/hyper.toml`.
To modify the animus runtime (for example, to enable or disable logging), configure `~/.cajal/animi/my_network/config.toml` before loading.


# Development
Brainstorm is a work-in-progress.
Active priorities are tracked in [`TODO.md`](/TODO.md).

