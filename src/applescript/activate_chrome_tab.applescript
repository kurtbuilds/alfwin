on run argv
  tell application "Google Chrome"
    activate
    set index of window (item 1 of argv as number) to 1
    set active tab index of window 1 to (item 2 of argv as number)
  end tell
end run