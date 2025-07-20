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
Simulation(
    name: "Oven Simulation",
    description: "A simple simulation about an oven with multiple devices",
    start_at: Offset("-PT10s"),
    end_at: Never(),
    devices: [
        Device(
            name: "oven",
            sensors: [
                Sensor(
                    name: "temperature",
                    sampling_rate: 1000,
                    value_generator: GeneratorConfig(
                        generator_id: "sma",
                        params: {
                            "min": "10.0",
                            "max": "20.0",
                            "precision": "2",
                            "buffer_size": "10"
                        }
                    )
                ),
                Sensor(
                    name: "lumens",
                    sampling_rate: 5000,
                    value_generator: GeneratorConfig(
                        generator_id: "const",
                        params: {
                            "val": "10",
                        }
                    ),
                ),
            ],
            devices: [
                Device(
                    name: "fan",
                    sensors: [
                        Sensor(
                            name: "speed",
                            sampling_rate: 3000,
                            value_generator: GeneratorConfig(
                                generator_id: "sma",
                                params: {
                                    "min": "500",
                                    "max": "600",
                                    "precision": "1",
                                    "buffer_size": "10"
                                }
                            ),
                        ),
                    ]
                )
            ]
        )
    ],
    output_plugins: [
      OutputConfig(
        output_id: "stdout",
        params: {
          "pretty": "true"
        }
      )
    ]
)
```

### 2. Configure the Plugins

The simulator needs to know where to find the plugin libraries. This is configured in a `settings.toml` file:

```toml
[[generator_plugins]]
id = "const"
generator_type = "Stateless"
path = "./target/debug/libiot_simulator_generator_const.dylib"

[[generator_plugins]]
id = "sma"
generator_type = "Stateful"
path = "./target/debug/libiot_simulator_generator_sma.dylib"

[[output_plugins]]
id = "stdout"
path = "./target/debug/libiot_simulator_output_stdout.dylib"
```

### 3. Run the Simulation

Once you have created the configuration files, you can run the simulation using the command-line interface:

```bash
cargo run -- -c settings.toml -s test.ron
```

This will start the simulation and print the generated data to the console.

There are already prepared config files in the testing folder so the demo can be run with:
```bash
cargo run -- -s ./testing/test.ron -c ./testing/settings.toml
```
