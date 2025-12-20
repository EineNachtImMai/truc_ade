# Urgent

# High
- if less than 3 (?) rooms, show them side by side
- optimization: in get_free_rooms and get_zik_..., treat all time windows at once
- Find which rooms correspond to which level for the Zik thingy

# Medium
- memory optimizations (borrow the array of calendars instead of cloning it every time)
- add other rooms and options to select which rooms you want to watch
- allow user to choose how long of a period the app covers (default 1 day)
- logging
- more tests
- test caching to see if it's worth it

# Low
- French readme version
- QR code for easier ADE app integration

# Not planned





# Unclassified


# DONE
- automatically update the time window it covers
- host it on my server to test it in prod conditions
- make it multithreaded??? Useful if used by more than 3 people
- remove the awkward 10-min periods (integrate them into the bigger ones)
- Zik thingy
- CACHING
- tests
- investigate why so slow despite multithreading
