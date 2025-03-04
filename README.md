# PromQuery

Query and visualize instant values of Prometheus metrics from the command-line.

## Install

With [`cargo-binstall`](https://github.com/cargo-bins/cargo-binstall):

```
$ cargo binstall promquery
```

From source via crates.io:

```
$ cargo install promquery
```

From source via Git:

```
$ git clone https://github.com/romac/promquery
$ cd promquery
$ cargo install --path .
```

## Example

```
$ promquery 'http://localhost:29000' malachite_consensus_time_per_block --show-plot

malachite_consensus_time_per_block
Time taken to finalize a block, in seconds.
1 sample

1. 2024-09-12 11:57:29.665216 UTC
  Histogram: count=1901, sum=2517.38, mean=1.32, buckets=21

⡁⠈⠀⠁⠈⠀⠁⠈⠀⠁⠈⠀⠁⠈⠀⠁⠈⠀⠁⠈⠀⠁⠈⠀⠁⠈⠀⠁⠈⠀⠁⠈⠀⠁⠈⠀⠁⠈⠀⠁⠈⠀⠁⠈⠀⠁⠈⠀⠁⠈⠀⠁⠈⠀⠁⠈⠀⡏⠉⠉⠉⢹⠉⠉⠉⠉⢹⠉⠉⠉⠉⡏⠉⠉⠉⠉⡏⠉⠉⠉⢹⠉⠉⠉⠉⢹⠉⠉⠉⠉⡇ 1896.0
⠄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⢸⠀⠀⠀⠀⢸⠀⠀⠀⠀⡇⠀⠀⠀⠀⡇⠀⠀⠀⢸⠀⠀⠀⠀⢸⠀⠀⠀⠀⡇
⠂⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⢸⠀⠀⠀⠀⢸⠀⠀⠀⠀⡇⠀⠀⠀⠀⡇⠀⠀⠀⢸⠀⠀⠀⠀⢸⠀⠀⠀⠀⡇
⡁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⢸⠀⠀⠀⠀⢸⠀⠀⠀⠀⡇⠀⠀⠀⠀⡇⠀⠀⠀⢸⠀⠀⠀⠀⢸⠀⠀⠀⠀⡇
⠄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣀⣀⣀⣀⡇⠀⠀⠀⢸⠀⠀⠀⠀⢸⠀⠀⠀⠀⡇⠀⠀⠀⠀⡇⠀⠀⠀⢸⠀⠀⠀⠀⢸⠀⠀⠀⠀⡇
⠂⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⡇⠀⠀⠀⢸⠀⠀⠀⠀⢸⠀⠀⠀⠀⡇⠀⠀⠀⠀⡇⠀⠀⠀⢸⠀⠀⠀⠀⢸⠀⠀⠀⠀⡇
⡁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⡇⠀⠀⠀⢸⠀⠀⠀⠀⢸⠀⠀⠀⠀⡇⠀⠀⠀⠀⡇⠀⠀⠀⢸⠀⠀⠀⠀⢸⠀⠀⠀⠀⡇
⠄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⡇⠀⠀⠀⢸⠀⠀⠀⠀⢸⠀⠀⠀⠀⡇⠀⠀⠀⠀⡇⠀⠀⠀⢸⠀⠀⠀⠀⢸⠀⠀⠀⠀⡇
⠂⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⡇⠀⠀⠀⢸⠀⠀⠀⠀⢸⠀⠀⠀⠀⡇⠀⠀⠀⠀⡇⠀⠀⠀⢸⠀⠀⠀⠀⢸⠀⠀⠀⠀⡇
⡁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⡇⠀⠀⠀⢸⠀⠀⠀⠀⢸⠀⠀⠀⠀⡇⠀⠀⠀⠀⡇⠀⠀⠀⢸⠀⠀⠀⠀⢸⠀⠀⠀⠀⡇
⠄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⡇⠀⠀⠀⢸⠀⠀⠀⠀⢸⠀⠀⠀⠀⡇⠀⠀⠀⠀⡇⠀⠀⠀⢸⠀⠀⠀⠀⢸⠀⠀⠀⠀⡇
⠂⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⡇⠀⠀⠀⢸⠀⠀⠀⠀⢸⠀⠀⠀⠀⡇⠀⠀⠀⠀⡇⠀⠀⠀⢸⠀⠀⠀⠀⢸⠀⠀⠀⠀⡇
⡁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⡇⠀⠀⠀⢸⠀⠀⠀⠀⢸⠀⠀⠀⠀⡇⠀⠀⠀⠀⡇⠀⠀⠀⢸⠀⠀⠀⠀⢸⠀⠀⠀⠀⡇
⠄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⡇⠀⠀⠀⢸⠀⠀⠀⠀⢸⠀⠀⠀⠀⡇⠀⠀⠀⠀⡇⠀⠀⠀⢸⠀⠀⠀⠀⢸⠀⠀⠀⠀⡇
⠂⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⡇⠀⠀⠀⢸⠀⠀⠀⠀⢸⠀⠀⠀⠀⡇⠀⠀⠀⠀⡇⠀⠀⠀⢸⠀⠀⠀⠀⢸⠀⠀⠀⠀⡇
⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠀⠁⠈⠀⠁⠈⠀⠁⠈⠀⠁⠈⠀⠉⠈⠀⠁⠈⠁⠁⠈⠀⠁⠉⠀⠁⠈⠈⠁⠈⠀⠁⠈⠀⠁⠈⠀⠁ 0.0
0.0                                                                                    1.9
```

## License

Copyright 2024 Romain Ruetschi

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
