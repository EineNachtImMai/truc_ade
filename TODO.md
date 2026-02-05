# Urgent

# High
- Find which rooms correspond to which level for the Zik thingy

# Medium
- HEAVY refactor
- Change where (spread between 2 fields) and how (shorter name) the free rooms are displayed in the calendar view

# Low
- French readme version
- QR code for easier ADE app integration
- allow HOST to choose how long of a period the app covers (default 1 day) (if user, DDOS vector)

# Not planned
- more tests
- optimization: in get_free_rooms and get_zik_..., treat all time windows at once => couldn't find a way to optimize it





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
- test caching to see if it's worth it (IT IS)
- add the ability to specify which rooms to watch
- another layer of caching after the processing
- memory optimizations (borrow the array of calendars instead of cloning it every time)
- refactor: revamp the errors to use Box<dyn Error>
- if less than 3 (?) rooms, show them side by side instead
- logging
- README update
