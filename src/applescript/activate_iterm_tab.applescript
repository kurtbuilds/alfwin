on run argv
	tell application "iTerm"
		activate
		tell window (item 1 of argv as number)
			select
			tell tab (item 2 of argv as number)
				select
			end tell
		end tell
	end tell
end run