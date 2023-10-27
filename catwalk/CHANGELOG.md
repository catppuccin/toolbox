# Changelog

## [0.1.4](https://github.com/catppuccin/toolbox/compare/catwalk-v0.1.4...catwalk-v0.1.4) (2023-10-27)


### ⚠ BREAKING CHANGES

* catwalk rewrite ([#26](https://github.com/catppuccin/toolbox/issues/26))
* attempt to fix broken alpha

### Features

* add grid lyout ([84f0017](https://github.com/catppuccin/toolbox/commit/84f001761f2182d314079edf08fddeff2cbc5168))
* add stacked layout ([781db52](https://github.com/catppuccin/toolbox/commit/781db52d096dfa01564b5e10c71f0e7d9252eb98))
* backgrounds ([046e181](https://github.com/catppuccin/toolbox/commit/046e18152b57c2fb529227307520b8450177a3aa))
* balanced sections ([407a377](https://github.com/catppuccin/toolbox/commit/407a37720affbeb2524f60bd58e40d7d13c1a5fe))
* catwalk rewrite ([#26](https://github.com/catppuccin/toolbox/issues/26)) ([28e9cc5](https://github.com/catppuccin/toolbox/commit/28e9cc5bc78f570a9ffc49af985170398c23aecc))
* **catwalk:** performance, bug fixes, refactor ([9c046a3](https://github.com/catppuccin/toolbox/commit/9c046a3588778bca67b83c701b10f5ba09845b4f))
* **catwalk:** remove Pillow-SIMD ([4ddf8cd](https://github.com/catppuccin/toolbox/commit/4ddf8cdecbd3630283bcb1a66a4617711754fb83))
* **catwalk:** use anti-aliasing ([6c92c29](https://github.com/catppuccin/toolbox/commit/6c92c2928d5ce8e6661cb988e7aa217991516795))
* **catwalk:** use Pillow-native `rounded_rectangle` ([075bf1c](https://github.com/catppuccin/toolbox/commit/075bf1c8519bfb45cc3e6700e353cf7313c0c7be))
* check Pillow version for Pillow-SIMD compat ([87a2828](https://github.com/catppuccin/toolbox/commit/87a2828d5955bec3ee99c894b6a98068d932655f))
* clean up code, create output dir if necessary ([c107d33](https://github.com/catppuccin/toolbox/commit/c107d334e07bdf00f67132ffcbd4662cf3f77408))
* inverted masks ([8bbf543](https://github.com/catppuccin/toolbox/commit/8bbf5439ff178a23c57b677ea926e18497881477))
* no longer crops image ([72cc98b](https://github.com/catppuccin/toolbox/commit/72cc98b61a72c92121349aa7292eb56c708f33be))
* prevent outer cropping ([d46b860](https://github.com/catppuccin/toolbox/commit/d46b860590ed12c7b6eb79540c22d9e56a297acc))
* rainbows! ([c776e9b](https://github.com/catppuccin/toolbox/commit/c776e9b8f70d46a80ab73a6a47e16e0a193c4933))
* rounded corners ([c73fc9a](https://github.com/catppuccin/toolbox/commit/c73fc9a7208ecaa1dec18c552b3d635afa624f58))
* shadows! ([dd58b5b](https://github.com/catppuccin/toolbox/commit/dd58b5b9d264b5f1a8bd968d8c4796a1c5b8992f))
* webp export ([46e84e9](https://github.com/catppuccin/toolbox/commit/46e84e980bfbaff3bb004e74ddc9b484bd0262f2))


### Bug Fixes

* alpha_fit typo (line 48) ([fb61041](https://github.com/catppuccin/toolbox/commit/fb6104162f366964af970ef167b6c8792bdd7a8d))
* anti-aliasing ([df7bad6](https://github.com/catppuccin/toolbox/commit/df7bad6644e925885c6648e2ab01e883df08a213))
* anti-aliasing works on linux ([4750a9a](https://github.com/catppuccin/toolbox/commit/4750a9a9d02afcd3bbdc3b2e9e5edb05ca4596ce))
* attempt to fix broken alpha ([2781c74](https://github.com/catppuccin/toolbox/commit/2781c742d8f15c3fbfa8ccb06d8828e9b26a9e54))
* **catwalk:** correctly handle wrong `--background` color input ([fd56535](https://github.com/catppuccin/toolbox/commit/fd56535cf355194d28a0603e34276833a16d6b1a))
* **catwalk:** enable png decoder ([cddcbd2](https://github.com/catppuccin/toolbox/commit/cddcbd2059be27cb027549eb0c9133b6af33d833))
* **catwalk:** fix alpha handling on paste, add error handling ([7eda1cc](https://github.com/catppuccin/toolbox/commit/7eda1cc1ad4a67c928d78512326ef2c01b3ae15e))
* **catwalk:** remove debug logs, update help, improve defaults ([18da8bc](https://github.com/catppuccin/toolbox/commit/18da8bc28ac52287400a1027711c634f5233953e))
* **catwalk:** remove global, add main function ([fb07e0e](https://github.com/catppuccin/toolbox/commit/fb07e0efcb39cdcacb9b54ba0488027ab8842955))
* **catwalk:** spelling in shebang ([a8b2a22](https://github.com/catppuccin/toolbox/commit/a8b2a2211aee3b654aa86f2e9e5fa2a3ce51cc89))
* **catwalk:** use lossless webp export ([b23d109](https://github.com/catppuccin/toolbox/commit/b23d1094685e8550d55e0aaab40b7881cada41d4))
* handle alpha screenshots correctly ([c14137e](https://github.com/catppuccin/toolbox/commit/c14137ef13749d6cd929495db8297be2188b3ad5))
* padding between images ([63ba7b3](https://github.com/catppuccin/toolbox/commit/63ba7b3e352079af1dba71dd0049d1028f55dac1))
* typo ([7f3a5ac](https://github.com/catppuccin/toolbox/commit/7f3a5acd4010a7a4d64d8e74372c34adbac19d4d))


### Performance Improvements

* **catwalk:** Use custom anti-aliasing ([#32](https://github.com/catppuccin/toolbox/issues/32)) ([7ca262b](https://github.com/catppuccin/toolbox/commit/7ca262bd51aef7b67ea9a71840c2001482721f77))


### Miscellaneous Chores

* release 0.1.1 ([790d8b5](https://github.com/catppuccin/toolbox/commit/790d8b5fc28b8e4ad488064abee811cc28d34c97))
* release 0.1.1-gh ([41ae2f5](https://github.com/catppuccin/toolbox/commit/41ae2f5c7480280a2ab7ef5e558db6bd51e32295))
* release 0.1.2 ([7a9824a](https://github.com/catppuccin/toolbox/commit/7a9824ab1507940e350838db4eb948c79fcb502b))
* release 0.1.3 ([d226007](https://github.com/catppuccin/toolbox/commit/d22600737891eac9e636cb99c1f4018b5d017bad))
* release 0.1.4 ([bd637f5](https://github.com/catppuccin/toolbox/commit/bd637f567335a799823597bd6aa58aa44ea9705b))

## [0.1.4](https://github.com/catppuccin/toolbox/compare/catppuccin-catwalk-v0.1.1-gh...catppuccin-catwalk-v0.1.4) (2023-10-27)

### Features

Initial release.

### Miscellaneous Chores

How many releases does it take to set up a monorepo workflow?

Five. It took 5 releases.
