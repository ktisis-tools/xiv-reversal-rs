# Ktisis/GPose+

This project is a learning exercise in Reverse Engineering and DirectX rendering.
It runs via code injection.

The end goal is to expand the features of FFXIV's Group Pose mode and provide a workspace for posers.

If you're interested in reaching out about this project, please feel free to contact me on Discord at chirp#1337.

#### List of target features, in order of complexity (ascending):
- [ ] 2D overlay
- [ ] Free camera movement
- [ ] Save and load different camera settings
- [ ] User workspace UI
- [ ] In-game manipulation of actor and bone positions/rotations

#### Features that I would like to explore, but are not currently a priority:
- Timeline editor for rudimentary animation
- Creation of additional actors and editing
- Placement and rendering of 3D props in-game

## Disclaimers

As a note, I'm fully aware that this project, at least in its current scope, can be created as a [Dalamud](https://github.com/goatcorp/Dalamud) plugin. However, I would like to continue developing it as-is to maximise my own learning and skills development. If possible, I will likely look at porting functionality to a Dalamud plugin in future.

Additionally, due to some unknown funkiness, the DLL injector will only work with an instance of the game launched via [XIVQuickLauncher](https://github.com/goatcorp/FFXIVQuickLauncher). I'm not sure why, and haven't yet looked in-depth to find out why.

Finally, this tool is not intended to be a competitor to CMTool or Anamnesis. My focus is not to replicate the functionality of either, but to instead fill its own niche, whatever that may be.

## Acknowledgements

- Thanks to the developers from [Goat Corp](https://github.com/goatcorp) and [XIV Tools](https://github.com/XIV-Tools), whose existing work in this area provided excellent insight into internal memory and data structures.
- [@Fayti1703](https://github.com/Fayti1703) and [@hackmud-dtr](https://github.com/hackmud-dtr) for their helpful insights into reverse engineering and low-level code, and overall helpfulness in answering questions.
- [@BobTheBob9](https://github.com/BobTheBob9) for their helpful insights into reverse engineering and IDA.