# Examples

This directory contains mod examples and documentation in order for you to better wrap your head around how it works. I promise it's super easy!

*\*For now, replacer mods are only supported in offline mode. Plugins are supported in both. This will probably change in the future!*

# Table of Contents

1. [How Modding Works](#how-modding-works)
2. [How to Activate Mods](#how-to-activate-mods)
3. [How do I Create a Texture/Audio/Font Mod?](#how-do-i-create-a-textureaudiofont-mod)
4. [Another Example - Audio Mod](#another-example---audio-mod)
5. [Creating a Plugin](#creating-a-plugin)
6. [Good Luck!](#good-luck)

## How Modding Works

There are two kinds of mods, "replacers" and "plugins". Replacers are mods that replace files in the game, such as textures or audio. Plugins are JavaScript files that may change the game's behavior.

Mods exist in their own folders. So, if you make or download a replacer mod called "MyMod", the mods folder should look like so:
```sh
mods/
  # Your mod goes into "replacers" because it replaces files in the game
  replacers/
    MyMod/
      # ... all of your files
```

...or a plugin mod called "MyPlugin":
```sh
mods/
  # Your mod goes into "plugins" because it changes the game's behavior
  plugins/
    MyPlugin.js
```

When a new mod is added to a user's mod directories, it is automatically added to the mod list.

## How to Activate Mods

Any changes a mod makes will apply automatically the next time the user loads the game. This doesn't even require a full restart, just press `Ctrl+R`!

## How do I Create a Texture/Audio/Font Mod?

Your mod must match the file structure of [PokeRogue](https://github.com/pagefaultgames/pokerogue/blob/main/). Here's an example:

* You want to replace the PokeRogue logo with your modified one.
* Anything that would be in the "public" folder of the PokeRogue repository should be in the ROOT of your "MyMod" folder.
  * In this example, you are replacing `public/images/logo.png`, so your mod folder should contain `images/logo.png` like so:
    ```sh
    # PokeRogue Structure:
    public/
      images/
        logo.png

    # Your Mod Structure:
    MyMod/
      images/
        logo.png
    ```

If you are familiar with how The Binding of Isaac mods work, this is basically the same. The result should be instantly noticeable, and the example provided in `mods/LogoReplacer` will result in the following:

![image](https://github.com/SpikeHD/RogueTop/assets/25207995/d734b3a6-ad37-4995-9152-a9f32f09ed24)


## Another Example - Audio Mod

Now, let's say you want to replace Pikachu's cry. In the PokeRogue repository, the file is located at `public/audio/cry/25.m4a` (because Pikachu is Pokémon #25). So, your mod folder should look like this:

```sh
# PokeRogue Structure:
public/
  audio/
    cry/
      25.m4a

# Your Mod Structure:
MyMod/
  audio/
    cry/
      25.m4a
```

That's it! Your mod should now properly apply! You can verify this by starting a new game and selecting the Pokemon in the starter selection menu (if you have them unlocked).

## Creating a Plugin

Plugins are pretty open-ended, technically you can make this do *anything*. Some examples would be changing values, names of things, or just how the game works. Here are some specific notes regarding plugins:

* RogueTop doesn't have any special API helpers for plugins... [yet](https://github.com/SpikeHD/RogueTop/issues/new/choose)
* Plugins must be singular JavaScript files (no `import`s!). If you want to bundle several files together, or like TypeScript, my personal reccomendation is to use a bundler like [esbuild](https://esbuild.github.io/).
* Plugins are executed somewhat late in the loading cycle. If you need plugins to load early for some reason, feel free to make a feature request.

A simple plugin that literally just logs something to the console would look like so:
```js
// MyPlugin.js
console.log("Hello from MyPlugin!");
```

# Good Luck!

If this document didn't make it click, that's fine, just take a look at the `mods/` folder here and see how it works. Both examples written here are present in that folder. It's really simple once you get the hang of it!

If you'd like to request more advanced modding features, feel free to open a new issue.
