set -l seen '__fish_seen_subcommand_from'
set -l has_opt '__fish_contains_opt'

set -l commands shell toggle scheme screenshot record clipboard emoji wallpaper resizer
set -l not_seen "not $seen $commands"

# Disable file completions
complete -c vela -f

# Add help for any command
complete -c vela -s 'h' -l 'help' -d 'Show help'

# Subcommands
complete -c vela -n $not_seen -a 'shell' -d 'Start the shell or message it'
complete -c vela -n $not_seen -a 'toggle' -d 'Toggle a special workspace'
complete -c vela -n $not_seen -a 'scheme' -d 'Manage the color scheme'
complete -c vela -n $not_seen -a 'screenshot' -d 'Take a screenshot'
complete -c vela -n $not_seen -a 'record' -d 'Start a screen recording'
complete -c vela -n $not_seen -a 'clipboard' -d 'Open clipboard history'
complete -c vela -n $not_seen -a 'emoji' -d 'Emoji/glyph utilities'
complete -c vela -n $not_seen -a 'wallpaper' -d 'Manage the wallpaper'
complete -c vela -n $not_seen -a 'resizer' -d 'Window resizer'

# Shell
set -l commands mpris drawers wallpaper notifs
set -l not_seen "$seen shell && not $seen $commands"
complete -c vela -n $not_seen -s 'd' -l 'daemon' -d 'Start the shell detached'
complete -c vela -n $not_seen -s 's' -l 'show' -d 'Print all IPC commands'
complete -c vela -n $not_seen -s 'l' -l 'log' -d 'Print the shell log'
complete -c vela -n $not_seen -l 'log-rules' -d 'Log rules to apply'
complete -c vela -n $not_seen -a 'mpris' -d 'Mpris control'
complete -c vela -n $not_seen -a 'drawers' -d 'Toggle drawers'
complete -c vela -n $not_seen -a 'wallpaper' -d 'Wallpaper control (for internal use)'
complete -c vela -n $not_seen -a 'notifs' -d 'Notification control'

set -l commands getActive play pause playPause stop next previous list
set -l not_seen "$seen shell && $seen mpris && not $seen $commands"
complete -c vela -n $not_seen -a 'play' -d 'Play media'
complete -c vela -n $not_seen -a 'pause' -d 'Pause media'
complete -c vela -n $not_seen -a 'playPause' -d 'Play/pause media'
complete -c vela -n $not_seen -a 'next' -d 'Skip to next song'
complete -c vela -n $not_seen -a 'previous' -d 'Go to previous song'
complete -c vela -n $not_seen -a 'stop' -d 'Stop media'
complete -c vela -n $not_seen -a 'list' -d 'List media players'
complete -c vela -n $not_seen -a 'getActive' -d 'Get a property from the active MPRIS player'

set -l commands trackTitle trackArtist trackAlbum position length identity
set -l not_seen "$seen shell && $seen mpris && $seen getActive && not $seen $commands"
complete -c vela -n $not_seen -a 'trackTitle' -d 'Track title'
complete -c vela -n $not_seen -a 'trackArtist' -d 'Track artist'
complete -c vela -n $not_seen -a 'trackAlbum' -d 'Track album'
complete -c vela -n $not_seen -a 'position' -d 'Track position'
complete -c vela -n $not_seen -a 'length' -d 'Track length'
complete -c vela -n $not_seen -a 'identity' -d 'Player identity'

set -l commands list toggle
set -l not_seen "$seen shell && $seen drawers && not $seen $commands"
complete -c vela -n $not_seen -a 'list' -d 'List toggleable drawers'
complete -c vela -n $not_seen -a 'toggle' -d 'Toggle a drawer'

set -l commands (vela shell drawers list 2> /dev/null)
complete -c vela -n "$seen shell && $seen drawers && $seen toggle && not $seen $commands" -a "$commands" -d 'drawer'

set -l commands list get set
set -l not_seen "$seen shell && $seen wallpaper && not $seen $commands"
complete -c vela -n $not_seen -a 'list' -d 'List wallpapers'
complete -c vela -n $not_seen -a 'get' -d 'Get current wallpaper path'
complete -c vela -n $not_seen -a 'set' -d 'Change wallpaper'
complete -c vela -n "$seen shell && $seen wallpaper && $seen set" -F

complete -c vela -n "$seen shell && $seen notifs && not $seen clear" -a 'clear' -d 'Clear popup notifications'

# Toggles
set -l commands communication music specialws sysmon todo
complete -c vela -n "$seen toggle && not $seen drawers && not $seen $commands" -a "$commands" -d 'toggle'

# Scheme
set -l commands list get set
set -l not_seen "$seen scheme && not $seen $commands"
complete -c vela -n $not_seen -a 'list' -d 'List available schemes'
complete -c vela -n $not_seen -a 'get' -d 'Get scheme properties'
complete -c vela -n $not_seen -a 'set' -d 'Set the current scheme'

complete -c vela -n "$seen scheme && $seen list" -s 'n' -l 'names' -d 'List scheme names'
complete -c vela -n "$seen scheme && $seen list" -s 'f' -l 'flavors' -d 'List scheme flavors'
complete -c vela -n "$seen scheme && $seen list" -s 'm' -l 'modes' -d 'List scheme modes'
complete -c vela -n "$seen scheme && $seen list" -s 'v' -l 'variants' -d 'List scheme variants'

complete -c vela -n "$seen scheme && $seen get" -s 'n' -l 'name' -d 'Get scheme name'
complete -c vela -n "$seen scheme && $seen get" -s 'f' -l 'flavor' -d 'Get scheme flavor'
complete -c vela -n "$seen scheme && $seen get" -s 'm' -l 'mode' -d 'Get scheme mode'
complete -c vela -n "$seen scheme && $seen get" -s 'v' -l 'variant' -d 'Get scheme variant'

complete -c vela -n "$seen scheme && $seen set" -l 'notify' -d 'Send a notification on error'
complete -c vela -n "$seen scheme && $seen set" -s 'r' -l 'random' -d 'Switch to a random scheme'
complete -c vela -n "$seen scheme && $seen set" -s 'n' -l 'name' -d 'Set scheme name' -a "$(vela scheme list -n)" -r
complete -c vela -n "$seen scheme && $seen set" -s 'f' -l 'flavor' -d 'Set scheme flavor' -a "$(vela scheme list -f)" -r
complete -c vela -n "$seen scheme && $seen set" -s 'm' -l 'mode' -d 'Set scheme mode' -a "$(vela scheme list -m)" -r
complete -c vela -n "$seen scheme && $seen set" -s 'v' -l 'variant' -d 'Set scheme variant' -a "$(vela scheme list -v)" -r

# Screenshot
complete -c vela -n "$seen screenshot" -s 'r' -l 'region' -d 'Capture region'
complete -c vela -n "$seen screenshot" -s 'f' -l 'freeze' -d 'Freeze while selecting region'

# Record
complete -c vela -n "$seen record" -s 'r' -l 'region' -d 'Capture region'
complete -c vela -n "$seen record" -s 's' -l 'sound' -d 'Capture sound'

# Clipboard
complete -c vela -n "$seen clipboard" -s 'd' -l 'delete' -d 'Delete from clipboard history'

# Wallpaper
complete -c vela -n "$seen wallpaper" -s 'p' -l 'print' -d 'Print the scheme for a wallpaper' -rF
complete -c vela -n "$seen wallpaper" -s 'r' -l 'random' -d 'Switch to a random wallpaper' -rF
complete -c vela -n "$seen wallpaper" -s 'f' -l 'file' -d 'The file to switch to' -rF
complete -c vela -n "$seen wallpaper" -s 'n' -l 'no-filter' -d 'Do not filter by size'
complete -c vela -n "$seen wallpaper" -s 't' -l 'threshold' -d 'The threshold to filter by' -r
complete -c vela -n "$seen wallpaper" -s 'N' -l 'no-smart' -d 'Disable smart mode switching'

# Emoji
complete -c vela -n "$seen emoji" -s 'p' -l 'picker' -d 'Open emoji/glyph picker'
complete -c vela -n "$seen emoji" -s 'f' -l 'fetch' -d 'Fetch emoji/glyph data from remote'

# Resizer
complete -c vela -n "$seen resizer" -s 'd' -l 'daemon' -d 'Start in daemon mode'
complete -c vela -n "$seen resizer" -a 'pip' -d 'Quick pip mode'
complete -c vela -n "$seen resizer" -a 'active' -d 'Select the active window'
