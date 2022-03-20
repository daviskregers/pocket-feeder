# Pocket aggregator

Currently WIP.

Learning rust, trying to make an application that would aggregate RSS feeds apply filters and then
sync them with [Pocket App](https://getpocket.com/).

Applies additional filters on top of the RSS feeds to exclude specific categories or authors.

# Running

```console
$ cp sources.example.yml
$ vim sources.yml
$ cargo run
```

# TODO:

- [x] Aggregate RSS feeds
- [x] Filter feed items with specific categories or authors
- [ ] Implement pocket API
- [ ] Refactor & add tests
