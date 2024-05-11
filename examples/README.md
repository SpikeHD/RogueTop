# Examples

This directory contains mod examples and documentation in order for you to better wrap your head around how it works. I promise it's super easy!

*\*For now, mods are only supported in offline mode. This will probably change in the future!*

## How Modding Works

Mods exist in their own folders. So, if you make or download a mod called "MyMod", the mods folder should look like so:
```sh
mods/
  # Your mod goes into "replacers" because it replaces files in the game
  replacers/
    MyMod/
      # ... all of your files
```

When a new folder is added to a users mods directory, it is automatically added to the mod list.

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

# Good Luck!

If this document didn't make it click, that's fine, just take a look at the `mods/` folder here and see how it works. Both examples written here are present in that folder. It's really simple once you get the hang of it!

If you'd like to request more advanced modding features, feel free to open a new issue.
