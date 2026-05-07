<!----------------------------------------------------------------------------->

# Copper Launcher - Legacy branch

> [!IMPORTANT]
> You are currently viewing the `legacy` branch of Copper Launcher.
> 
> Here is the old codebase for Copper Launcher before the rewrite, this code was never actually finished, so don't expect anything fully working here.

**Copper Launcher** is a GTK based launcher for Minecraft Java Edition that allows you to have multiple instances.

<!----------------------------------------------------------------------------->

## Table of Contents

- [Current features](#current-features)
- [Currently working on](#currently-working-on)
- [Note](#note)
- [Installation](#installation)
- [License](#license)

<!----------------------------------------------------------------------------->

## Current features

- **GTK Interface** - Will fit nicely within Gnome Desktop Environment.
- **Instances** - Allows for having multiple Minecraft installations.

<!----------------------------------------------------------------------------->

## Currently working on...

> [!NOTE]
> Since launcher is in very early stage of development, many features are still missing.

- [-] Core features
  - [x] Launcher's UI
  - [x] Instances
    - [x] Version selection
    - [x] Creating instance
    - [x] Downloading the game
  - [ ] Ability to launch the game
- [ ] Launcher preferences
- [ ] Auto downloading java
- [ ] Login with Microsoft account

<!----------------------------------------------------------------------------->

## Note

Currently Copper Launcher is only aimed at Linux based systems. Windows and overall multi-platform support is not planned at the moment.

<!----------------------------------------------------------------------------->

## Installation

To install Copper Launcher you can clone this repository and compile it using cargo.

```
git clone https://github.com/Suverent-Shiro/Copper-Launcher.git

cd Copper-Launcher/

cargo run --release
```

<!----------------------------------------------------------------------------->
## License
Copper Launcher is licensed under GNU General Public License v3.0.
For more information check LICENSE file in this repository.
