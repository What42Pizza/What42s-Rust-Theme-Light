## Setup:

- Make sure you can launch vscode extensions: https://code.visualstudio.com/api/extension-guides/color-theme#create-a-new-color-theme
- Intall 'hjson' and 'fs-extras' through npm

<br>

## Develop:

- **1: Make changes to 'src/base color-theme.json'**
- **2: Run 'node ./src/scripts/build_themes.js' to create and/or update the actual theme files**
- **3: Repeat**

<br>

## Export:

- Run 'node ./src/scripts/build_extension.js', and a folder names 'output' should appear with the finished extension (this script also builds the themes)
