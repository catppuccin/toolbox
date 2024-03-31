# Changelog

## [2.0.0](https://github.com/catppuccin/toolbox/compare/whiskers-v1.1.4...whiskers-v2.0.0) (2024-03-31)


### ⚠ BREAKING CHANGES

* switch to tera & rich context variables

### Features

* switch to tera & rich context variables ([bdf0dc5](https://github.com/catppuccin/toolbox/commit/bdf0dc54b0271c26ea5522e105a562ef946e46bd))
* **whiskers:** add colors object in context ([#104](https://github.com/catppuccin/toolbox/issues/104)) ([0f08acc](https://github.com/catppuccin/toolbox/commit/0f08acc98b77fb8ef2c62cf6d1e842afcc0265bf))
* **whiskers:** add rgba to hex helpers ([#120](https://github.com/catppuccin/toolbox/issues/120)) ([31ffd9e](https://github.com/catppuccin/toolbox/commit/31ffd9e2bc806fcbd9f0c14653c93c17a91ba6c7))
* **whiskers:** enforce semver versioning in frontmatter ([bdf0dc5](https://github.com/catppuccin/toolbox/commit/bdf0dc54b0271c26ea5522e105a562ef946e46bd))
* **whiskers:** update catppuccin to v2 & add `flavorName` var ([#137](https://github.com/catppuccin/toolbox/issues/137)) ([8e60740](https://github.com/catppuccin/toolbox/commit/8e607401c48447f368e4beb59157b34ace1c4a85))


### Bug Fixes

* **deps:** update rust crate catppuccin to 1.4 ([#124](https://github.com/catppuccin/toolbox/issues/124)) ([0b6de43](https://github.com/catppuccin/toolbox/commit/0b6de43b4817fa4e34fcebe5fde81159d9103a8c))
* **deps:** update rust crate catppuccin to 2.1 ([#145](https://github.com/catppuccin/toolbox/issues/145)) ([0eb1fd7](https://github.com/catppuccin/toolbox/commit/0eb1fd78420f6257a1ed11ee71af7e54d02b5c2c))
* **deps:** update rust crate catppuccin to 2.2 ([#146](https://github.com/catppuccin/toolbox/issues/146)) ([fe9fd1a](https://github.com/catppuccin/toolbox/commit/fe9fd1a8be8c2179b2d0c136b5ce324bae5b2c28))
* **deps:** update rust crate clap to 4.5 ([#127](https://github.com/catppuccin/toolbox/issues/127)) ([20d4047](https://github.com/catppuccin/toolbox/commit/20d40479bbf3345f2b1038c736a07ccb4c6efda9))
* **deps:** update rust crate clap-stdin to 0.4.0 ([#96](https://github.com/catppuccin/toolbox/issues/96)) ([dc6c017](https://github.com/catppuccin/toolbox/commit/dc6c0177cedbde090d63993587f6360722c0ed65))
* **deps:** update rust crate indexmap to 2.2.3 ([#126](https://github.com/catppuccin/toolbox/issues/126)) ([70bfca0](https://github.com/catppuccin/toolbox/commit/70bfca0dbc060e85be291ad230d617bc8c7f9c5e))
* **deps:** update rust crate tempfile to 3.10 ([#117](https://github.com/catppuccin/toolbox/issues/117)) ([c8846f6](https://github.com/catppuccin/toolbox/commit/c8846f6b038c69aa42a85cdaa46b1ae378f869ba))
* **whiskers:** use block context in darklight helper ([#144](https://github.com/catppuccin/toolbox/issues/144)) ([486a747](https://github.com/catppuccin/toolbox/commit/486a74772ebb159913063f668dd1f015e8418129))

## [1.1.4](https://github.com/catppuccin/toolbox/compare/whiskers-v1.1.3...whiskers-v1.1.4) (2023-12-10)


### Miscellaneous Chores

* **whiskers:** release as 1.1.4 ([0edb5ff](https://github.com/catppuccin/toolbox/commit/0edb5ff8bd2474eb6954a5a5539b27679873d2fc))

## [1.1.3](https://github.com/catppuccin/toolbox/compare/whiskers-v1.1.2...whiskers-v1.1.3) (2023-12-10)


### Bug Fixes

* use hex -&gt; rgb(a) without hsla where possible ([#95](https://github.com/catppuccin/toolbox/issues/95)) ([c7c095f](https://github.com/catppuccin/toolbox/commit/c7c095ff7d14d4b43065b4a81c45e9e5354c87c6))

## [1.1.2](https://github.com/catppuccin/toolbox/compare/whiskers-v1.1.1...whiskers-v1.1.2) (2023-11-23)


### Bug Fixes

* **deps:** update rust crate base64 to 0.21.5 ([#70](https://github.com/catppuccin/toolbox/issues/70)) ([be92614](https://github.com/catppuccin/toolbox/commit/be9261407e181a3cbf2bb88be871727ebd88dc3e))
* **deps:** update rust crate clap to 4.4.7 ([#71](https://github.com/catppuccin/toolbox/issues/71)) ([21cdc5d](https://github.com/catppuccin/toolbox/commit/21cdc5d1e51f2145758c49e8fff83a426ee72cee))
* **deps:** update rust crate clap to 4.4.8 ([#84](https://github.com/catppuccin/toolbox/issues/84)) ([efac5e3](https://github.com/catppuccin/toolbox/commit/efac5e3548521d5bdcaa83f49c8775bfab20dda2))
* **deps:** update rust crate handlebars to 4.5.0 ([#79](https://github.com/catppuccin/toolbox/issues/79)) ([cbd1cb7](https://github.com/catppuccin/toolbox/commit/cbd1cb7fdebb9e7f7deb57ed2cae9055a5623e56))
* **deps:** update rust crate serde to 1.0.192 ([#74](https://github.com/catppuccin/toolbox/issues/74)) ([02676a9](https://github.com/catppuccin/toolbox/commit/02676a91c57123b8b77b92a4f15fe9c4b2925b22))
* **deps:** update rust crate serde to 1.0.193 ([#86](https://github.com/catppuccin/toolbox/issues/86)) ([020f291](https://github.com/catppuccin/toolbox/commit/020f2910ade722dfa3d3a358f8e6baa7feacd29a))
* **deps:** update rust crate serde_json to 1.0.108 ([#75](https://github.com/catppuccin/toolbox/issues/75)) ([e9effd0](https://github.com/catppuccin/toolbox/commit/e9effd05376c041ac0605fde6bdc0e8f614de558))
* **deps:** update rust crate serde_yaml to 0.9.27 ([#76](https://github.com/catppuccin/toolbox/issues/76)) ([9eb5470](https://github.com/catppuccin/toolbox/commit/9eb54703ff49c9ee06b8be63396dddfca6a60f2c))

## [1.1.1](https://github.com/catppuccin/toolbox/compare/whiskers-v1.1.0...whiskers-v1.1.1) (2023-10-28)


### Miscellaneous Chores

* **whiskers:** release as 1.1.1 ([9033840](https://github.com/catppuccin/toolbox/commit/9033840c0b9cf591b7a35e5f595e044925f1cb2b))

## [1.1.0](https://github.com/catppuccin/toolbox/compare/whiskers-v1.0.3...whiskers-v1.1.0) (2023-10-28)

Re-released as 1.1.1 because the CI to publish the binary/crate failed.

### Features

* **whiskers:** add check mode with diff view ([6bb415e](https://github.com/catppuccin/toolbox/commit/6bb415e87921f8db1266edde15737ac7bb24bd90))
* **whiskers:** add version flag ([6bb415e](https://github.com/catppuccin/toolbox/commit/6bb415e87921f8db1266edde15737ac7bb24bd90))

## [1.0.3](https://github.com/catppuccin/toolbox/compare/whiskers-v1.0.2...whiskers-v1.0.3) (2023-10-28)


### Miscellaneous Chores

* **whiskers:** release as 1.0.3 ([5bd49bf](https://github.com/catppuccin/toolbox/commit/5bd49bfd1ef6b5b3e9618e6c7f8b4550e5b564ca))

## [1.0.2](https://github.com/catppuccin/toolbox/compare/whiskers-v1.0.1...whiskers-v1.0.2) (2023-10-27)


### Miscellaneous Chores

* **whiskers:** release as 1.0.2 ([d20e4b6](https://github.com/catppuccin/toolbox/commit/d20e4b6be08d85c26ea5896767d6b10988185e22))

## [1.0.1](https://github.com/catppuccin/toolbox/compare/whiskers-v1.0.0...whiskers-v1.0.1) (2023-10-27)


### Miscellaneous Chores

* **whiskers:** release as 1.0.1 ([92b9409](https://github.com/catppuccin/toolbox/commit/92b9409b67047d0f58a4255b8bed638a112cd54d))

## [1.0.0](https://github.com/catppuccin/toolbox/compare/whiskers-v1.0.0...whiskers-v1.0.0) (2023-10-27)


### Features

* add whiskers ([#46](https://github.com/catppuccin/toolbox/issues/46)) ([9c4a5bb](https://github.com/catppuccin/toolbox/commit/9c4a5bb84563e1af57a5ab8670f550b2fbcf21e9))


### Miscellaneous Chores

* **whiskers:** release as 1.0.0 ([c8e15ce](https://github.com/catppuccin/toolbox/commit/c8e15ce96aa04a835da970de5355b60c2b7b213d))
