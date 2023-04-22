# What42's Rust Theme (Light)

### This is my personal VS Code theme, focused mainly on readability. It was originally inspired by / made because of [CodeStyle's](https://www.youtube.com/@_codestyle) videos

<br>

## Goals of this theme:

- **Readability**
  - Text should be dark and contrast against the background
- **Pleasentness**
  - I want it to just look as nice as possible
- **Bring attention to the right things**
  - If it isn't important, don't make it stand out
  - If it's possibly bad (mut, self, etc), it should stand out as bad

<br>

Unfortunately, I don't think I can change the color of 'unsafe' without sacrificing other coloring abilities, but hopefully that will change sometime soon. If you want an alternate theme that colors 'unsafe', you can open an issue and I'll start working on it if I have the time (or you can make it yourself, this is under an MIT license (edit 'editor.semanticTokenColorCustomizations')).

<br>

**I highly suggest that you use [f.lux](https://justgetflux.com/) or some other program that darkens your screen at night**

<br>
<br>

# Example:

![Example](images/example.png)

<br>

# Entire screen example:

![Entire screen example](images/entire_screen.png)

### Note: These images use [JetBrains Mono](https://www.jetbrains.com/lp/mono/) and [Material Icons](https://marketplace.visualstudio.com/items?itemName=PKief.material-icon-theme)

<br>
<br>

## Supported languages:

- **Rust**
- **Toml**
- **Markdown**

<br>
<br>

## Suggested settings:

```
{
	"files.autoSave": "afterDelay",
    "editor.acceptSuggestionOnEnter": "off",
	"editor.lightbulb.enabled": false,
	"editor.insertSpaces": false,  // use tabs instead of spaces
	"editor.detectIndentation": false,
	"editor.cursorBlinking": "solid",
	"editor.fontSize": 14.5,
	"editor.letterSpacing": 0.1,
	"editor.lineHeight": 1.3,
	"editor.padding.top": 8,
	"editor.fontLigatures": true,
	"workbench.tree.indent": 10,
	"editor.comments.insertSpace": false,
	"terminal.integrated.smoothScrolling": true,
	"workbench.list.smoothScrolling": true,
	"terminal.integrated.smoothScrolling": true,
}
```

<br>

**Reminder: You can always override colors with these settings:**

- "workbench.colorCustomizations"
- "editor.tokenColorCustomizations"
- "editor.semanticTokenColorCustomizations" & "editor.semanticHighlighting.enabled": true

<br>

## Links:

- [Changelog](CHANGELOG.md)
- [Marketplace](https://marketplace.visualstudio.com/items?itemName=What42Pizza.what42s-rust-theme-light)
- [GitHub](https://github.com/What42Pizza/What42s-Rust-Theme_Light)

<br>

### I can't guarantee that I'll support other languages and/or take fixes or suggestions, but it's not impossible
