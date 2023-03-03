# Constraints

Several trade-offs were being made. The proposed alternative solutions are not hard to implement, just different and one decision had to be made. 

### The two I thought most about are:

* The cron job is set to run at 3:30am every night, based on the latest commit in the repository. I would change that maybe now to query the repo every 1hour, parse the date in the document, and check if it has changed. If it did, update the in-memory storage.
* Changing the storage is more time-intensive. Threads are sharing the in-memory storage via a `Mutex`. This could be changed to have an `ArcSwap`.  More interesting is the idea of not blocking at all (blocking in this case is a few milliseconds, even if that). So it's not a downtime and I, in my opinion, fulfilled this requirement to the fullest. To implement a cache, which invalided itself just to save a few milli or nanoseconds in this case could be done, but the engineering effort and the increased complexity of the code doesn't justify the current use case.

### Trade Offs
* Querying the blocklist every hour to see if it was getting updated, and if it was, updating the in-memory storage.
  * The problem there is that the repository is deleting old commits (or squashing them), therefore I have no track record of knowing when the automated services is publishing the latest version.
  * One way I am doing the scheduled check now is via a cron-like scheduler library. Probably easier (although more resource intensive) is to check every 30 minutes, and if the date in the document changed, update the in-memory storage. If not, don't do anything.
* Testing different in-storage read and writes, especially the choice between choosing a `HashMap` or using a `Vec`, sorting it, and using binary search to look up a given IP address.
  * It is not clear, without testing` if sorting + binary search is more performant than creating a Hash Table and lookup the IP address. BigO notation would dictate that Hash lookups are less complex (O(1)) than binary searches (O(log n)).
  * Complexity doesn't say much about speed though, and depending on this use case, one would have to test locally with an ever-growing list to see what ther results are.
* Testing different ways of accessing the in-memory storage close-to-lock free: `ArcSwap` vs. `Arc` for example.
* General testing: It's a pretty straight forward application. One could test filling the `Store` and call the `contain` function to see if it works. A manual `curl` is testing it and a few test cases are enough to see if it works or not.
* Error handling: Definitely not production ready. But after 4 hours, nothing could be production ready.
* No CLI arguments: I would have liked to pass at least the log level down, and maybe a set the time for the cron job to run.