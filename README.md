<!--
SPDX-FileCopyrightText: © 2022 Jade Meskill

SPDX-License-Identifier: MIT
-->

# sprinkle

[![MIT licensed][mit-badge]][mit-url]
[![Build Status][actions-badge]][actions-url]

[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/iamruinous/sprinkle/blob/master/LICENSE
[actions-badge]: https://github.com/iamruinous/sprinkle/actions/workflows/ci.yml/badge.svg
[actions-url]: https://github.com/iamruinous/sprinkle/actions?query=workflow%3Aci.yml+branch%3Amain

A stupid simple dotfile manager

sprinkle is a very simple dotfile / symlink manager. It is inspired by rcrc and other dotfile managers out there. It is written in rust, and is available as a very compact binary with no dependencies in (releases)[https://github.com/iamruinous/sprinkle/releases/latest].

## Example Config
```toml
debug = false

[sources]
  [sources.friendly_name]
  enabled = true
  path = "~/dotfiles"
  excludes = [
    "README",
    "LICENSE",
  ]

  [sources.another_friendly_name]
  enabled = false
  path = "~/some-other-dir"
```
