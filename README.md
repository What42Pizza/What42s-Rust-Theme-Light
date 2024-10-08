<br>

<h1 align="center">What42's Rust Theme (Light)</h1>

<p align="center">
	<img src="https://vsmarketplacebadges.dev/version-short/What42Pizza.what42s-rust-theme-light.png?style=for-the-badge&colorA=44444D&colorB=FFD600&label=VERSION" alt="Version">&nbsp;
	<img src="https://vsmarketplacebadges.dev/rating-short/What42Pizza.what42s-rust-theme-light.png?style=for-the-badge&colorA=44444D&colorB=FFD600&label=Rating" alt="Rating">&nbsp;
	<img src="https://vsmarketplacebadges.dev/installs-short/What42Pizza.what42s-rust-theme-light.png?style=for-the-badge&colorA=44444D&colorB=FFD600&label=Installs" alt="Installs">&nbsp;
	<img src="https://vsmarketplacebadges.dev/downloads-short/What42Pizza.what42s-rust-theme-light.png?style=for-the-badge&colorA=44444D&colorB=FFD600&label=Downloads" alt="Downloads">
</p>

<h3 align="center" style="font-weight: bold;">This is my personal VS Code theme, focused mainly on readability. It was originally inspired by / made because of <a href="https://www.youtube.com/@_codestyle">CodeStyle's</a> videos</h3>

<h4 align="center">Many color themes appear to prioritize looks and colors over readability, but not this one. I've combined every strategy that I know of to create the fastest reading code I've ever seen. And this isn't based on theory, I put every change and strategy to the test with an obscene amount of trail and error. The main strategies / rules employed are 1: dark text on a light background, 2: importance / noticeability matching (important items should stand out and vice-versa), and 3: distinct colors to break up otherwise monolithic text. Nearly every color theme breaks one or both of the first two rules, because if you think about it, they strongly conflict with the last rule for many varied reasons. It's almost impossible to fulfill all three rules at the same time, but I think I've been able to do it.</h4>

<h1></h1>

<br>

![Example](images/example.png)

### These images use [JetBrains Mono](https://www.jetbrains.com/lp/mono/) and [Material Icons](https://marketplace.visualstudio.com/items?itemName=PKief.material-icon-theme)

<br>

## Goals of this theme

#### **Readability**
&nbsp; &nbsp; &nbsp; &nbsp; **Text should be dark and should contrast against the background** \
&nbsp; &nbsp; &nbsp; &nbsp; **Nothing should be distracting**
#### **Bring attention to the right things**
&nbsp; &nbsp; &nbsp; &nbsp; **If it isn't important, it shouldn't stand out** \
&nbsp; &nbsp; &nbsp; &nbsp; **If it's possibly bad (mut, self, etc), it should stand out as bad**
#### **Pleasantness**
&nbsp; &nbsp; &nbsp; &nbsp; **It should be modern, colorful, simple, and just generally pleasant**

### But it's not just that, this color theme has a secret super power: you forget that you're using it.

Are you tired of constantly searching for a perfect theme? Are you tired of the nagging feeling that the theme you're using might not be the best you can get? Well, I'm proud to say that I rarely ever think about this theme, despite the fact that I'm constantly using it. When I create something that I'll actually use, one of my goals is to make it unobtrusive so you can focus on what's actually important, and that fact that I rarely every update this is proof that I've achieved my goal.

<br>

# Entire screen example &nbsp; (Blue and White themes)

![Entire screen example](images/entire_screen.png)

<br>

## First-class languages

&nbsp; &nbsp; &nbsp; **Rust** \
&nbsp; &nbsp; &nbsp; **Lua** \
&nbsp; &nbsp; &nbsp; **Glsl** \
&nbsp; &nbsp; &nbsp; **JSON** \
&nbsp; &nbsp; &nbsp; **Toml** \
&nbsp; &nbsp; &nbsp; **Markdown** \
&nbsp; &nbsp; &nbsp; **C#** \
&nbsp; &nbsp; &nbsp; **XML**

(Many more languages have some support, but not much)

## Links

&nbsp; &nbsp; &nbsp; &nbsp; **[Changelog](CHANGELOG.md)** \
&nbsp; &nbsp; &nbsp; &nbsp; **[Marketplace](https://marketplace.visualstudio.com/items?itemName=What42Pizza.what42s-rust-theme-light)** \
&nbsp; &nbsp; &nbsp; &nbsp; **[GitHub](https://github.com/What42Pizza/What42s-Rust-Theme-Light)**

<br>

# Additional customizations

## Suggested settings:

``` hjson
{
	"terminal.integrated.minimumContrastRatio": 2.5, // lower is more colorful, higher is easier to read
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

## Set your own colors:

``` hjson
{
	"workbench.colorCustomizations": {},
	"editor.tokenColorCustomizations": {},
	"editor.semanticHighlighting.enabled": true, // these last two are optional
	"editor.semanticTokenColorCustomizations": {},
}
```

To edit these into your settings, you can open your settings.json file by opening command palette (f1) and selecting "Preferences: Open User Settings (JSON)".

<br>

## Suggested programs:

&nbsp; &nbsp; &nbsp; &nbsp; **[f.lux](https://justgetflux.com/)** (darkens screen at night)

<br>

## Other themes to check out:

&nbsp; &nbsp; &nbsp; &nbsp; **[Amethyst Themes](https://marketplace.visualstudio.com/items?itemName=amodio.amethyst-theme)** (dark only) \
&nbsp; &nbsp; &nbsp; &nbsp; **[Winter Is Coming](https://marketplace.visualstudio.com/items?itemName=johnpapa.winteriscoming)** (dark and light) \
&nbsp; &nbsp; &nbsp; &nbsp; **[Mimesis](https://marketplace.visualstudio.com/items?itemName=AlexanderDyriavin.mimesis)** (light only) \
&nbsp; &nbsp; &nbsp; &nbsp; **[Yet Another Solarized Theme](https://marketplace.visualstudio.com/items?itemName=JulianSchelb.yet-another-solarized-theme)** (dark and light) \
&nbsp; &nbsp; &nbsp; &nbsp; **[Cerulean](https://marketplace.visualstudio.com/items?itemName=OwenWilliams.cerulean)** (dark and light) \
&nbsp; &nbsp; &nbsp; &nbsp; **[Kary Pro Colors](https://marketplace.visualstudio.com/items?itemName=karyfoundation.theme-karyfoundation-themes)** (dark and light) \
&nbsp; &nbsp; &nbsp; &nbsp; **[MagicUser](https://marketplace.visualstudio.com/items?itemName=BernardoPires.magicuser-color-themes)** (dark and light) \
&nbsp; &nbsp; &nbsp; &nbsp; **[Lightning](https://marketplace.visualstudio.com/items?itemName=zevross.lightning)** (dark and light)

<br>
<br>
<br>

I can't guarantee that I'll support other languages and/or take fixes or suggestions, but it's not impossible \
Also, I don't think I can change the color of 'unsafe' without sacrificing other coloring abilities, but hopefully that will change sometime soon \
If you want an alternate theme that colors 'unsafe', you can open an issue and I'll start working on it if I have the time (or you can make it yourself, this is under an MIT license)

<br>

### License: &nbsp; [MIT](LICENSE)
