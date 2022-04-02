# Pocket Feeder

[![codecov](https://codecov.io/gh/daviskregers/pocket-feeder/branch/main/graph/badge.svg?token=PL9BT1YQHA)](https://codecov.io/gh/daviskregers/pocket-feeder)

Currently WIP.

Learning rust, trying to make an application that would aggregate RSS feeds apply filters and then
sync them with [Pocket App](https://getpocket.com/).

Applies additional filters on top of the RSS feeds to exclude specific categories or authors.

# Running

```console
$ cp config.example.yml
$ vim config.yml
$ cargo run
```

The Pocket API key can be found by creating a new Pocket app https://getpocket.com/developer/apps/new
