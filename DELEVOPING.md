# Using the Rust builder: &nbsp; (suggested)

<br>

## Setup:

- Make sure you can launch vscode extensions: https://code.visualstudio.com/api/extension-guides/color-theme#create-a-new-color-theme
- Make sure you can build and run Rust programs: https://www.rust-lang.org/tools/install

<br>

## Develop:

- **1: In the 'builder' folder, run 'cargo run'** (or just run 'cargo -C ./builder -Z unstable-options run' from the main folder)
- **2: Make changes to 'base color-theme.json', and they will be automatically applied**
- **3: When done, enter 'Stop' to stop the builder**

<br>

## Export:

- **1: In the 'builder' folder, run 'cargo run'**
- **2: Enter 'Export', and a folder named 'output' should appear with the finished extension** (this command does not update the theme files, because the builder already builds the themes every time it is started)

<br>
<br>
<br>
<br>
<br>

# Using the Node builder:

<br>

## Setup:

- Make sure you can launch vscode extensions: https://code.visualstudio.com/api/extension-guides/color-theme#create-a-new-color-theme
- Intall 'hjson' and 'fs-extras' through npm

<br>

## Develop:

- **1: Make changes to 'base color-theme.json'**
- **2: Run 'node ./node_scripts/build_themes.js' to create and/or update the actual theme files**
- **3: Repeat**

<br>

## Export:

- Run 'node ./node_scripts/build_extension.js', and a folder named 'output' should appear with the finished extension (this script also builds the themes)
