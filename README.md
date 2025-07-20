# IoT Simulator

This project is a highly extensible IoT simulator written in Rust. It allows you to define a hierarchy of devices and sensors, and simulate the data they generate over a period of time. The simulator is designed to be modular, with a plugin-based architecture for both data generation and output.

## Features

*   **Hierarchical Device Structure:** Define a tree-like structure of devices and sensors to accurately model your IoT deployments.
*   **Pluggable Data Generators:** Create your own data generators to simulate any type of sensor data. The simulator comes with two examples:
    *   `const`: A simple generator that produces a constant value.
    *   `sma`: A simple moving average generator.
*   **Pluggable Outputs:** Send your simulated data to any destination. The simulator comes with a `stdout` output plugin that prints the data to the console.
*   **Configuration via RON:** The simulation is defined in a [RON (Rusty Object Notation)](https://github.com/ron-rs/ron) file, which is easy to read and write.

## How to Use

### 1. Configure the Simulation

The simulation is defined in a RON file. Here is an example:

```ron
(
    start_at: "2025-07-20T00:00:00Z",
    end_at: "2025-07-20T00:01:00Z",
    devices: [
        (
            name: "building-1",
            devices: [
                (
                    name: "floor-1",
                    sensors: [
                        (
                            name: "temperature",
                            generator: (
                                generator_id: "sma",
                                params: {
                                    "initial_value": "25.0",
                                    "max_change": "0.5"
                                }
                            )
                        ),
                        (
                            name: "humidity",
                            generator: (
                                generator_id: "const",
                                params: {
                                    "value": "60.0"
                                }
                            )
                        )
                    ]
                )
            ]
        )
    ]
)
```

### 2. Configure the Plugins

The simulator needs to know where to find the plugin libraries. This is configured in a `settings.toml` file:

```toml
[[generator_plugins]]
id = "const"
path = "target/debug/libiot_simulator_generator_const.so"

[[generator_plugins]]
id = "sma"
path = "target/debug/libiot_simulator_generator_sma.so"

[[output_plugins]]
id = "stdout"
path = "target/debug/libiot_simulator_output_stdout.so"
```

### 3. Run the Simulation

Once you have created the configuration files, you can run the simulation using the command-line interface:

```bash
cargo run -- -c settings.toml -s test.ron
```

This will start the simulation and print the generated data to the console.
