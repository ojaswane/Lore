#!/bin/bash
osascript -e 'tell application "Terminal"
    activate
    do script "cd /Users/ojaswane/lore/Lore && cargo run"
end tell'