# Sway Center

A simple daemon for automatically centering lone windows on large displays in Sway.

**Disclaimer:** At the time of writing I have spent about two hours writing this. It's very brittle and probably mostly tailored to my setup.

This program will run on a loop and listen for Sway IPC window events, checking to see if there is only one window in the active workspace. If there is, the daemon sets the horizontal gaps to center the window in about a third of the workspace/output. If there is more than one window, it will reset the horizontal gaps to zero.

# Other options

For a more manual approach, you may also achieve this functionality with something like the following in your Sway config.
 
```sway
# Toggle the horizontal gaps in the current workspace
bindsym $mod+g gaps horizontal current toggle 600 # adjust for your output size
```

Or for a more granular approach you might use something like this.

```sway
# Increase/reduce/reset horizontal gaps for the current workspace
mode "gaps" {
    bindsym {
        $up     gaps horizontal current plus  100
        Up      gaps horizontal current plus  100
        $down   gaps horizontal current minus 100
        Down    gaps horizontal current minus 100
        r       gaps horizontal current set   0
        Return  mode "default"
        Escape  mode "default"
    }
}
bindsym $mod+g mode "gaps"
```

As of right now, `sway-center` will readily override gaps set by other processes, so mixing these two methods is not advised.
