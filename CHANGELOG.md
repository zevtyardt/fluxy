##  (2024-12-02)

* ci: add GitHub Actions workflow for release ([e09b60d](https://github.com/zevtyardt/fluxy/commit/e09b60d))
* ci: add support for aarch64 targets in release workflow ([b79bb64](https://github.com/zevtyardt/fluxy/commit/b79bb64))
* ci: remove changelog path from release workflow ([ac71770](https://github.com/zevtyardt/fluxy/commit/ac71770))
* ci: update GitHub Actions workflow for releasing ([61ea629](https://github.com/zevtyardt/fluxy/commit/61ea629))
* feat: Add benchmark section in readme ([05b4098](https://github.com/zevtyardt/fluxy/commit/05b4098))
* feat: add convenience methods for Source creation ([a0e06e9](https://github.com/zevtyardt/fluxy/commit/a0e06e9))
* feat: add GithubRepoProvider for proxy list fetching ([546d439](https://github.com/zevtyardt/fluxy/commit/546d439))
* feat: add initial validator module ([5e7658b](https://github.com/zevtyardt/fluxy/commit/5e7658b))
* feat: add Progress struct for download status tracking ([bb584ed](https://github.com/zevtyardt/fluxy/commit/bb584ed))
* feat: Create README.md ([edc75aa](https://github.com/zevtyardt/fluxy/commit/edc75aa))
* feat: enable module name display in logging setup ([1e4ecfb](https://github.com/zevtyardt/fluxy/commit/1e4ecfb))
* feat: enhance FreeProxyListProvider with multiple sources ([52d1b66](https://github.com/zevtyardt/fluxy/commit/52d1b66))
* feat: enhance GithubRepoProvider with additional sources ([935ae05](https://github.com/zevtyardt/fluxy/commit/935ae05))
* feat: enhance models with country and source structures ([ead4eb9](https://github.com/zevtyardt/fluxy/commit/ead4eb9))
* feat: enhance Proxy and Protocol models ([37654ce](https://github.com/zevtyardt/fluxy/commit/37654ce))
* feat: enhance ProxyFetcher with concurrency and provider support ([66214a7](https://github.com/zevtyardt/fluxy/commit/66214a7))
* feat: enhance ProxyFetcher with unique IP enforcement ([4e66a39](https://github.com/zevtyardt/fluxy/commit/4e66a39))
* feat: expose downloader module in library interface ([b674f97](https://github.com/zevtyardt/fluxy/commit/b674f97))
* feat: implement FreeProxyListProvider for proxy fetching ([f1bb2fc](https://github.com/zevtyardt/fluxy/commit/f1bb2fc))
* feat: implement ProxyClient for handling proxy connections ([dbc4789](https://github.com/zevtyardt/fluxy/commit/dbc4789))
* feat: initialize Cargo.toml for Rust project "fluxy" ([28d5bb9](https://github.com/zevtyardt/fluxy/commit/28d5bb9))
* feat: integrate ProxyClient and improve main application logic ([2cbf7c7](https://github.com/zevtyardt/fluxy/commit/2cbf7c7))
* feat: introduce ProxyFilter and ProxyFetcherConfig models ([aaf4c29](https://github.com/zevtyardt/fluxy/commit/aaf4c29))
* feat(fetcher): add ProxyFetcherOptions struct ([a443c4d](https://github.com/zevtyardt/fluxy/commit/a443c4d))
* feat(fetcher): enhance ProxyFetcher with geo lookup options ([f0baca9](https://github.com/zevtyardt/fluxy/commit/f0baca9))
* feat(fetcher): integrate GeoIp into ProxyFetcher ([244d078](https://github.com/zevtyardt/fluxy/commit/244d078))
* feat(geoip): implement GeoIp struct for IP geolocation ([7919928](https://github.com/zevtyardt/fluxy/commit/7919928))
* refactor: adjust logging level and simplify main function ([8a3c2cf](https://github.com/zevtyardt/fluxy/commit/8a3c2cf))
* refactor: derive PartialEq for Anonymity and Protocol enums ([8988609](https://github.com/zevtyardt/fluxy/commit/8988609))
* refactor: enhance download_geolite with detailed progress reporting ([13898e6](https://github.com/zevtyardt/fluxy/commit/13898e6))
* refactor: enhance Proxy and Source models ([56e9975](https://github.com/zevtyardt/fluxy/commit/56e9975))
* refactor: improve documentation in free_proxy_list and github modules ([2a23c94](https://github.com/zevtyardt/fluxy/commit/2a23c94))
* refactor: improve IProxyTrait for better proxy management ([4e7fcd1](https://github.com/zevtyardt/fluxy/commit/4e7fcd1))
* refactor: modularize work handling in ProxyFetcher ([4d6b74d](https://github.com/zevtyardt/fluxy/commit/4d6b74d))
* refactor: move fetch method implementation to IProxyTrait ([4d9a98d](https://github.com/zevtyardt/fluxy/commit/4d9a98d))
* refactor: optimize proxy gathering in ProxyFetcher ([f92ff54](https://github.com/zevtyardt/fluxy/commit/f92ff54))
* refactor: remove downloader module ([7d79fc7](https://github.com/zevtyardt/fluxy/commit/7d79fc7))
* refactor: rename default_protocols to default_types for clarity ([3c06ab1](https://github.com/zevtyardt/fluxy/commit/3c06ab1))
* refactor: rename protocols to types in Proxy and Source structs ([b194165](https://github.com/zevtyardt/fluxy/commit/b194165))
* refactor: replace validator module with client module ([cc1fc1b](https://github.com/zevtyardt/fluxy/commit/cc1fc1b))
* refactor: simplify main function logging ([963bc4e](https://github.com/zevtyardt/fluxy/commit/963bc4e))
* refactor: simplify sources and scrape methods in FreeProxyListProvider ([3287af9](https://github.com/zevtyardt/fluxy/commit/3287af9))
* refactor: streamline ProxyFetcher configuration and remove unused options ([5bc041d](https://github.com/zevtyardt/fluxy/commit/5bc041d))
* refactor: update fetch method signature and scrape return type ([b80b2ae](https://github.com/zevtyardt/fluxy/commit/b80b2ae))
* refactor: update GeoIP module to use hyper for HTTP requests ([55b1928](https://github.com/zevtyardt/fluxy/commit/55b1928))
* refactor: update IProxyTrait fetch method to use hyper ([c47b899](https://github.com/zevtyardt/fluxy/commit/c47b899))
* refactor: update IProxyTrait for improved proxy handling ([9d88d0e](https://github.com/zevtyardt/fluxy/commit/9d88d0e))
* refactor: update main function for proxy fetching ([f441e0f](https://github.com/zevtyardt/fluxy/commit/f441e0f))
* refactor: update main function to display filtered proxies ([7aed229](https://github.com/zevtyardt/fluxy/commit/7aed229))
* refactor: update main function to use new ProxyFetcherConfig ([e057530](https://github.com/zevtyardt/fluxy/commit/e057530))
* refactor: update proxy handling in IProxyTrait ([9210d11](https://github.com/zevtyardt/fluxy/commit/9210d11))
* refactor: update ProxyFetcher to handle optional proxies ([34e22d8](https://github.com/zevtyardt/fluxy/commit/34e22d8))
* refactor: update ProxyFetcher to use hyper and improve error logging ([9dd973b](https://github.com/zevtyardt/fluxy/commit/9dd973b))
* refactor(fetcher): simplify proxy retrieval logic ([95839b0](https://github.com/zevtyardt/fluxy/commit/95839b0))
* refactor(main): switch to manual Tokio runtime management ([fc8ba28](https://github.com/zevtyardt/fluxy/commit/fc8ba28))
* refactor(models): replace Country enum with GeoData struct ([e34fe74](https://github.com/zevtyardt/fluxy/commit/e34fe74))
* refactor(providers): change SyncSender to Sender for proxy transmission ([e978b37](https://github.com/zevtyardt/fluxy/commit/e978b37))
* deps: remove unused native-tls and tokio-native-tls dependencies ([a2f37f8](https://github.com/zevtyardt/fluxy/commit/a2f37f8))
* deps: update dependencies and bump version to 0.2.0 ([1e7de2c](https://github.com/zevtyardt/fluxy/commit/1e7de2c))
* deps: update tokio dependency features in Cargo.toml ([2015335](https://github.com/zevtyardt/fluxy/commit/2015335))
* chore: bump version to 0.1.2 and update features ([83cba7d](https://github.com/zevtyardt/fluxy/commit/83cba7d))
* chore: increment version to 0.1.5 in Cargo.toml ([6c902e3](https://github.com/zevtyardt/fluxy/commit/6c902e3))
* chore: initiating core function. well not working for now ([890b25d](https://github.com/zevtyardt/fluxy/commit/890b25d))
* chore: remove timestamp configuration in logging setup ([23b6c85](https://github.com/zevtyardt/fluxy/commit/23b6c85))
* chore: remove unused validator module ([6bda521](https://github.com/zevtyardt/fluxy/commit/6bda521))
* chore: rename release.yaml to workflows directory ([0aa8563](https://github.com/zevtyardt/fluxy/commit/0aa8563))
* chore: update .gitignore for Rust project ([493b331](https://github.com/zevtyardt/fluxy/commit/493b331))
* chore: update dependencies and version bump to 0.1.4 ([a013303](https://github.com/zevtyardt/fluxy/commit/a013303))
* chore: update version and add futures-util dependency ([f2a5509](https://github.com/zevtyardt/fluxy/commit/f2a5509))
* chore(lib): remove downloader module reference ([aa77797](https://github.com/zevtyardt/fluxy/commit/aa77797))
* chore(main): initialize ProxyFetcher with options ([9ed2c37](https://github.com/zevtyardt/fluxy/commit/9ed2c37))
* chore(main): update proxy retrieval method ([0587927](https://github.com/zevtyardt/fluxy/commit/0587927))
* style: improve log messages formatting in geoip module ([d62c15b](https://github.com/zevtyardt/fluxy/commit/d62c15b))
* docs: add documentation comments to GeoIp and related functions ([792982c](https://github.com/zevtyardt/fluxy/commit/792982c))
* docs: add documentation comments to providers module ([d3a9075](https://github.com/zevtyardt/fluxy/commit/d3a9075))
* docs: add documentation comments to ProxyFetcher and its options ([7c895a7](https://github.com/zevtyardt/fluxy/commit/7c895a7))
* docs: enhance documentation for models in models.rs ([e57f80d](https://github.com/zevtyardt/fluxy/commit/e57f80d))
* docs: enhance module documentation in lib.rs ([c5d915c](https://github.com/zevtyardt/fluxy/commit/c5d915c))
* docs: update README for clarity and progress ([6b522b4](https://github.com/zevtyardt/fluxy/commit/6b522b4))
* docs: update README with example and debug output ([47bc235](https://github.com/zevtyardt/fluxy/commit/47bc235))
* fix: remove nipper dependency from Cargo.toml ([5995afe](https://github.com/zevtyardt/fluxy/commit/5995afe))
* fix(geoip): improve progress display formatting ([e6f3cdf](https://github.com/zevtyardt/fluxy/commit/e6f3cdf))
* fix(main): await ProxyFetcher initialization ([9fe4e2d](https://github.com/zevtyardt/fluxy/commit/9fe4e2d))
* enhance: increase concurrency limit in ProxyFetcher ([17b10e1](https://github.com/zevtyardt/fluxy/commit/17b10e1))
* cleanup: remove redundant fetch method in FreeProxyListProvider ([120b120](https://github.com/zevtyardt/fluxy/commit/120b120))
* cleanup: remove redundant unique IP enforcement in main ([b503ca1](https://github.com/zevtyardt/fluxy/commit/b503ca1))


