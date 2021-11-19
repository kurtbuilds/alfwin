# Strategy

1. Two possible strategies
    1. Have a daemon running that writes AppleScripts to filesystem.
    
    
# Benchmarking

     11:46:57 kurt@kurts-MacBook-Pro:~/work/alfwin $ t build bench
    osacompile -l JavaScript -o winnames.jxa.scpt winnames.jxa.applescript
    osacompile  -o winnames.scpt winnames.applescript
    hyperfine --warmup 3 "osascript winnames.jxa.scpt" "osascript winnames.scpt" "osascript winnames.applescript" "osascript -l JavaScript winnames.jxa.applescript"
    Benchmark 1: osascript winnames.jxa.scpt
      Time (mean ± σ):      2.012 s ±  0.091 s    [User: 0.079 s, System: 0.031 s]
      Range (min … max):    1.945 s …  2.196 s    10 runs

      Warning: The first benchmarking run for this command was significantly slower than the rest (2.051 s). This could be caused by (filesystem) caches that were not filled until after the first run. You should consider using the '--warmup' option to fill those caches before the actual benchmark. Alternatively, use the '--prepare' option to clear the caches before each timing run.

    Benchmark 2: osascript winnames.scpt
      Time (mean ± σ):     327.2 ms ±  39.0 ms    [User: 29.3 ms, System: 28.3 ms]
      Range (min … max):   296.3 ms … 391.8 ms    10 runs

    Benchmark 3: osascript winnames.applescript
      Time (mean ± σ):     386.1 ms ±  17.8 ms    [User: 46.9 ms, System: 36.2 ms]
      Range (min … max):   352.7 ms … 402.1 ms    10 runs

    Benchmark 4: osascript -l JavaScript winnames.jxa.applescript
      Time (mean ± σ):      1.952 s ±  0.016 s    [User: 0.079 s, System: 0.030 s]
      Range (min … max):    1.945 s …  1.997 s    10 runs

      Warning: Statistical outliers were detected. Consider re-running this benchmark on a quiet PC without any interferences from other programs. It might help to use the '--warmup' or '--prepare' options.

    Summary
      'osascript winnames.scpt' ran
        1.18 ± 0.15 times faster than 'osascript winnames.applescript'
        5.97 ± 0.71 times faster than 'osascript -l JavaScript winnames.jxa.applescript'
        6.15 ± 0.78 times faster than 'osascript winnames.jxa.scpt'