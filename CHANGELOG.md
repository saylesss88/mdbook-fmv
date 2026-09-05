# 1.0.0 (2026-09-05)


### Bug Fixes

* **ci:** add write permissions for semantic-release ([e47f5d7](https://github.com/saylesss88/mdbook-fmv/commit/e47f5d7028592a9b216740fd9f3b0a3e69d553e8))
* move back in commit history to before semantic-release fucked it all up ([cf6e44b](https://github.com/saylesss88/mdbook-fmv/commit/cf6e44bf8cbcdc592594045236820882dffce732))
* **tests:** fix tests that now expect a tags field ([3460750](https://github.com/saylesss88/mdbook-fmv/commit/34607501bc9c4aa3ee4a07f96be4c10b148be4ac))
* use rfind to correctly parse chapter paths containing parentheses in title ([c25047f](https://github.com/saylesss88/mdbook-fmv/commit/c25047f0d1f20b6fcddd7faa88e052ea6f8e5896))


### Features

* add CLI argument parsing with --fm and --html flags ([a050098](https://github.com/saylesss88/mdbook-fmv/commit/a05009818335c5e52714a216f29411761d1a67ad))
* add fix_frontmatter with Frontmatter struct ([5ff72c5](https://github.com/saylesss88/mdbook-fmv/commit/5ff72c50139929f73ab49704542955e1af6a4af9))
* add lang check and diagnostic ([6aa514c](https://github.com/saylesss88/mdbook-fmv/commit/6aa514c23af9e307c984a0179b1a6e087e6e4b97))
* add lang field to Frontmatter struct ([8e1cb67](https://github.com/saylesss88/mdbook-fmv/commit/8e1cb67b5c2ee2865e43dec7720c0652926b652b))
* check for unclosed YAML fence ([fcaaff5](https://github.com/saylesss88/mdbook-fmv/commit/fcaaff5ad74be0c6141e956a6c1703d15e59c733))
* detect missing author field in frontmatter ([0b2cc5f](https://github.com/saylesss88/mdbook-fmv/commit/0b2cc5f9d320ea5cd2c9542ca957c486ac0d0f71))
* detect missing date field in frontmatter ([173b6ec](https://github.com/saylesss88/mdbook-fmv/commit/173b6ec95c8f5b6f3f39eba60e65fb3d1b9c7587))
* detect missing title field in frontmatter ([c47df29](https://github.com/saylesss88/mdbook-fmv/commit/c47df29a03f141f2d1488ac7421df93f9954057b))
* detect unclosed details blocks in html content ([681a0ec](https://github.com/saylesss88/mdbook-fmv/commit/681a0ec07aa7c29d030c4df09233bafecb6a644c))
* detect unclosed summary blocks in html content ([7d35f41](https://github.com/saylesss88/mdbook-fmv/commit/7d35f417b1be53a92a4060ea1deec9d67c79381a))
* **frontmatter:** add fix_missing_tags function to pass test ([85cb115](https://github.com/saylesss88/mdbook-fmv/commit/85cb115f3e306e9f116b2a9c7b7246beef989e72))
* **frontmatter:** tie in tags to Frontmatter struct & fix_frontmatter function ([e7a3d5a](https://github.com/saylesss88/mdbook-fmv/commit/e7a3d5a8b7be030b09050e2b9bb95d6eede622bf))
* implement --fix flag to write missing frontmatter to disk ([b0db928](https://github.com/saylesss88/mdbook-fmv/commit/b0db928752c89aa2aea4d980d392e2fc7581abee))
* implement fix_missing_lang ([ee667df](https://github.com/saylesss88/mdbook-fmv/commit/ee667df6599abad56d8bf0609ffb8b965f61122f))
* layout project structure & create failing test ([00ce462](https://github.com/saylesss88/mdbook-fmv/commit/00ce4621900e0e9904d7f7eb74a3dbb3bc401f6e))
* **main:** exit early if book.toml isn't found in current directory ([b124b33](https://github.com/saylesss88/mdbook-fmv/commit/b124b338a7c0f6d75c72675a3d20cb12e5e6686a))
* **main:** read SUMMARY.md and parse it to collect chapter paths ([f9db2f4](https://github.com/saylesss88/mdbook-fmv/commit/f9db2f478d5803183d28ea1ae753f875b352dd70))
* **main:** wire in fix_missing_lang ([1762a40](https://github.com/saylesss88/mdbook-fmv/commit/1762a40d0a05fd9cf4b18576dfea2677e59ee30f))
* **main:** wire lang into the --fix flow ([61bb843](https://github.com/saylesss88/mdbook-fmv/commit/61bb843ebe323cf83aa6eeafc74854ed5f03af53))
* parse language field from book.toml with en fallback ([4821c38](https://github.com/saylesss88/mdbook-fmv/commit/4821c389ab4190fc228b379f5755008995de3000))
* **summary:** strip ./ prefix from chapter paths in summary parser ([bb74699](https://github.com/saylesss88/mdbook-fmv/commit/bb74699f2907ed03e577cc22befe44f803f33ca0))
* **tags:** implement infer_tags to derive tags from path segments ([dfb1d4f](https://github.com/saylesss88/mdbook-fmv/commit/dfb1d4f7da3e88a6fb8f2cb54e030e33e687cd5b))
* wire check_frontmatter and check_html into main validation loop ([5249e9f](https://github.com/saylesss88/mdbook-fmv/commit/5249e9f29f23d4fe6ef222bf8ff9eb146147b8ee))
