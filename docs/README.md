![banner](https://ghoust.s3.fr-par.scw.cloud/ledswarm_banner.svg)

# What's this?

This project provides firmware code for game controllers, enabling them to drive LEDs, read sensor data and interact with their environment using the network.

More specifically, this repository contains a Rust project which uses `esp-idf` to implement firmware for ESP32 controller boards, like the Xiao ESP32-C3 used in internal testing. The RMT module of the boards is used to efficiently drive a NeoPixel Jewel with seven RGBW LEDs (SK6812). An ADXL343 accelerometer and a 128x64 OLED display are attached to the I2C bus.

Firmware code for controllers in the ESP32 ecosystem based on `esp-idf`.

This embedded code drives a NeoPixel Jewel while reading values off the integrated ADXL343 accelerometer and interacts with the WebSocket base station.

# ledswarm_firmware_esp32c3

## Dev Containers
This repository offers Dev Containers supports for:
-  [VS Code Dev Containers](https://code.visualstudio.com/docs/remote/containers#_quick-start-open-an-existing-folder-in-a-container)
-  [GitHub Codespaces](https://docs.github.com/en/codespaces/developing-in-codespaces/creating-a-codespace)
> **Note**
>
> In [order to use GitHub Codespaces](https://github.com/features/codespaces#faq)
> the project needs to be published in a GitHub repository and the user needs
> to be part of the Codespaces beta or have the project under an organization.

If using VS Code or GitHub Codespaces, you can pull the image instead of building it
from the Dockerfile by selecting the `image` property instead of `build` in
`.devcontainer/devcontainer.json`. Further customization of the Dev Container can
be achived, see [.devcontainer.json reference](https://code.visualstudio.com/docs/remote/devcontainerjson-reference).

When using Dev Containers, some tooling to facilitate building, flashing and
simulating in Wokwi is also added.
### Build
- Terminal approach:

    ```
    scripts/build.sh  [debug | release]
    ```
    > If no argument is passed, `release` will be used as default


-  UI approach:

    The default build task is already set to build the project, and it can be used
    in VS Code and GitHub Codespaces:
    - From the [Command Palette](https://code.visualstudio.com/docs/getstarted/userinterface#_command-palette) (`Ctrl-Shift-P` or `Cmd-Shift-P`) run the `Tasks: Run Build Task` command.
    - `Terminal`-> `Run Build Task` in the menu.
    - With `Ctrl-Shift-B` or `Cmd-Shift-B`.
    - From the [Command Palette](https://code.visualstudio.com/docs/getstarted/userinterface#_command-palette) (`Ctrl-Shift-P` or `Cmd-Shift-P`) run the `Tasks: Run Task` command and
    select `Build`.
    - From UI: Press `Build` on the left side of the Status Bar.

### Flash

> **Note**
>
> When using GitHub Codespaces, we need to make the ports
> public, [see instructions](https://docs.github.com/en/codespaces/developing-in-codespaces/forwarding-ports-in-your-codespace#sharing-a-port).

- Terminal approach:
  - Using `flash.sh` script:

    ```
    scripts/flash.sh [debug | release]
    ```
    > If no argument is passed, `release` will be used as default

- UI approach:
    - From the [Command Palette](https://code.visualstudio.com/docs/getstarted/userinterface#_command-palette) (`Ctrl-Shift-P` or `Cmd-Shift-P`) run the `Tasks: Run Task` command and
    select `Build & Flash`.
    - From UI: Press `Build & Flash` on the left side of the Status Bar.
- Any alternative flashing method from host machine.


### Wokwi Simulation

#### VS Code Dev Containers and GitHub Codespaces

The Dev Container includes the Wokwi Vs Code installed, hence you can simulate your built projects doing the following:
1. Press `F1`
2. Run `Wokwi: Start Simulator`

> **Note**
>
>  We assume that the project is built in `debug` mode, if you want to simulate projects in release, please update the `elf` and  `firmware` proprieties in `wokwi.toml`.

For more information and details on how to use the Wokwi extension, see [Getting Started] and [Debugging your code] Chapter of the Wokwi documentation.

[Getting Started]: https://docs.wokwi.com/vscode/getting-started
[Debugging your code]: https://docs.wokwi.com/vscode/debugging
