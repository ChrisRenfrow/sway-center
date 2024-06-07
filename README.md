# Sway Center

A simple daemon for centering lone windows on large displays in Sway.

**Disclaimer:** At the time of writing I have spent about two hours writing this. It's very brittle and probably mostly tailored to my setup.

This program will run on a loop and listen for Sway IPC window events, checking to see if there is only one window in the active workspace. If there is, gaps will be set horizontally to center the window in about a third of the workspace/output. Otherwise it will set the horizontal gaps to zero.
