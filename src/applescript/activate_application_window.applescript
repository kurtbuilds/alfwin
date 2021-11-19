on run argv

  set proc to (item 1 of argv)
  set windowName to (item 2 of argv)

  tell application "System Events"
    with timeout of 0.1 seconds
      tell process proc to perform action "AXRaise" of window windowName
    end timeout
  end tell

  tell application proc
    activate
  end tell

end run