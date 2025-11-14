# Urgent

# High
- tests
- optimization: in get_free_rooms and get_zik_..., treat all time windows at once

# Medium
- memory and speed optimizations (borrow the array of calendars instead of cloning it every time)
- add other rooms and options to select which rooms you want to watch
- Zik thingy
- investigate why so slow despite multithreading

# Low
- allow user to choose how long of a period the app covers (default 1 day)
- French readme version
- QR code for easier ADE app integration

# Not planned





# Unclassified


# DONE
- automatically update the time window it covers
- host it on my server to test it in prod conditions
- make it multithreaded??? Useful if used by more than 3 people
- remove the awkward 10-min periods (integrate them into the bigger ones)
