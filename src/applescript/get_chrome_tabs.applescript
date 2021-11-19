on appIsRunning(appName)
  tell application "System Events" to (name of processes) contains appName
end appIsRunning

on list2string(theList, theDelimiter)

-- First, we store in a variable the current delimiter to restore it later
  set theBackup to AppleScript's text item delimiters

  -- Set the new delimiter
  set AppleScript's text item delimiters to theDelimiter

  -- Perform the conversion
  set theString to theList as string

  -- Restore the original delimiter
  set AppleScript's text item delimiters to theBackup

  return theString

end list2string


if appIsRunning("Google Chrome") is false then
  return
end if


tell application "Google Chrome"
  set tabnames to {}
  set i_win to 1
  repeat with win in every window
    set i_tab to 1
    repeat with t in every tab in win
      set t to i_win & "," & i_tab & "," & (get name of t) as string
      set tabnames to tabnames & t
      set i_tab to i_tab + 1
    end repeat
    set i_win to i_win + 1
  end repeat
end tell

return list2string(tabnames, "
")