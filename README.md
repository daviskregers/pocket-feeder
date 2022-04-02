# Pocket Feeder

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

# TODO:

- [x] Aggregate RSS feeds
- [x] Filter feed items with specific categories or authors
- [x] Implement pocket API
- [ ] Refactor & add tests
- [ ] Optimize - learn proper usage without copying things all the time
- [ ] Implement Atom RSS like https://martinfowler.com/feed.atom
- [ ] Implement RSS items outside channels like https://export.arxiv.org/rss/cs
- [ ] Tag items by source name
- [ ] Implement loading strategy when API returns incomplete list of items
