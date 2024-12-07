## Update Checklist

The following is a list of things you HAVE TO DO before submitting a version update PR.

- [ ] Generate packet identifiers with [CrabToolbox](https://github.com/CrabCraftDev/CrabToolbox/) using packets.json
  datagen
    - [ ] Decompile the game jar and check for any changes related to the affected packets
- [ ] Generate assets [(more info here)](https://github.com/CrabCraftDev/CrabCraft/blob/main/assets/README.md)
    - [ ] `blocks.json` using CrabExtractor (link coming soon)
    - [ ] `datapack` using Mojang's datagen
- [ ] Change version constants in `src/mc_server.rs`
- [ ] Test the update and ensure it works correctly

### REMEMBER: THIS LIST IS NOT EXHAUSTIVE.

**REMEMBER TO CHECK FOR ANY OTHER CHANGES ON [MINECRAFT WIKI](https://minecraft.wiki) AND IN THE DECOMPILED GAME JAR!**