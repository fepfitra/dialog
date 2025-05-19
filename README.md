# dialogtocli
An application to open a dialog GUI and print it to stdout.

## Installation
```bash
cargo install dialogtocli
```

## Usage
```txt
Usage: <command> [options]
Commands:
  -h   Show this help message
  -s   Save a file
  -l   Load a file
  -d   Set a directory
```

## Bind it to your favorite app

### Bash
```bash
hyprshot -m region -o $(dialogtocli -d)
```

### Fish
```fish
hyprshot -m region -o (dialogtocli -d)
```

### Niri config
```niri
Mod+Shift+A { spawn "sh" "-c" "hyprshot -m region -o $(dialogtocli -d)"; }
```

### i3 config
```i3
bindsym $mod+Shift+s exec sh -c "hyprshot -m region -o $(dialogtocli -d)"
```
