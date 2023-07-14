Imperative sequential: 35.30s
Functional sequential: 195.12s
Functional parallel:  29.24s
Functional parallel v2: 18.16s

All of the above are run on --dev build

After eliminating clone from the tight loop, making it parallel with rayon and running on --release build, I get: **~90ms!!**
