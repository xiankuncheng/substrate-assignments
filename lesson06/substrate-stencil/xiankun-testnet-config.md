# Public testnet:

## SECRECT (sr25519)
❯ ./target/release/substrate-stencil key generate --scheme sr25519

Secret phrase:       bag mansion tornado satisfy parent excuse baby pave balance icon avocado shift
  Network ID:        substrate
  Secret seed:       0x673650c995af3728adab529947f3f1e8dc548733c9c209242edd6d68b1a752c3
  Public key (hex):  0xb42b6073054a1732c79d2c71e15272c80a2ae6221fceb9d95a9cef92d132c315
  Account ID:        0xb42b6073054a1732c79d2c71e15272c80a2ae6221fceb9d95a9cef92d132c315
  Public key (SS58): 5G8wMx1Dp4FjpEbbs6SSdgVpXMNP5Z5JDhZwv5t7GEeCJkHJ
  SS58 Address:      5G8wMx1Dp4FjpEbbs6SSdgVpXMNP5Z5JDhZwv5t7GEeCJkHJ
------

## Stash, Controller account
❯ for i in 1 2 3 4; do for j in stash controller; do ./target/release/substrate-stencil key inspect "$SECRET//$i//$j"; done; done

Secret Key URI `bag mansion tornado satisfy parent excuse baby pave balance icon avocado shift//1//stash` is account:
  Network ID:        substrate
 Secret seed:       0x5e0399ea366ed67531cbe1a9d545975e78b7e0b2aaa3c2b83a8de348b3281503
  Public key (hex):  0x00ea330a63b346ff9145fba8a737d1578abd8ffb898f51e6761dc41b2a137663
  Account ID:        0x00ea330a63b346ff9145fba8a737d1578abd8ffb898f51e6761dc41b2a137663
  Public key (SS58): 5C5uRq6eC8Xu7eQ9TDXLbqCfpn2AWWxQBhF9F55wu5GTAYDw
  SS58 Address:      5C5uRq6eC8Xu7eQ9TDXLbqCfpn2AWWxQBhF9F55wu5GTAYDw
Secret Key URI `bag mansion tornado satisfy parent excuse baby pave balance icon avocado shift//1//controller` is account:
  Network ID:        substrate
 Secret seed:       0xcd116196af36326371f0da6aa5ed2d010f71ddddb750340c04f04713626a4319
  Public key (hex):  0x263c19766a66f9ca1d5f3463a2c50a23db57e5b02e80d50bbc7b68b98daeaa04
  Account ID:        0x263c19766a66f9ca1d5f3463a2c50a23db57e5b02e80d50bbc7b68b98daeaa04
  Public key (SS58): 5CvqXhLYrXdNgBEuLfEcmrfKthxow7JLT4ZQ2DJcx9L1zVFn
  SS58 Address:      5CvqXhLYrXdNgBEuLfEcmrfKthxow7JLT4ZQ2DJcx9L1zVFn
Secret Key URI `bag mansion tornado satisfy parent excuse baby pave balance icon avocado shift//2//stash` is account:
  Network ID:        substrate
 Secret seed:       0x13b9252c18d95da00a91e46c34833512e18abe0978b777b34d1b8fe34081cb13
  Public key (hex):  0x44a468940048dd8e4464288a50898a19e608dc7d7eb3a7ab66ce34ca903a3b34
  Account ID:        0x44a468940048dd8e4464288a50898a19e608dc7d7eb3a7ab66ce34ca903a3b34
  Public key (SS58): 5DchxDdtBkWzNpT8UHHqVQpFYaehagDJV7fcvV1rsrV1erAB
  SS58 Address:      5DchxDdtBkWzNpT8UHHqVQpFYaehagDJV7fcvV1rsrV1erAB
Secret Key URI `bag mansion tornado satisfy parent excuse baby pave balance icon avocado shift//2//controller` is account:
  Network ID:        substrate
 Secret seed:       0xf0b05145bb0011277088195692d613c65d62db6ded56b9de41cc61445822325e
  Public key (hex):  0x98c9eea6adccac9194306a80c91b4b8b247b972fb3b46e9e7b14fcc668507172
  Account ID:        0x98c9eea6adccac9194306a80c91b4b8b247b972fb3b46e9e7b14fcc668507172
  Public key (SS58): 5FX382nhyAtz1xBWz4sda1SBSuCwr6FiQXxTQyjChBMgfpRw
  SS58 Address:      5FX382nhyAtz1xBWz4sda1SBSuCwr6FiQXxTQyjChBMgfpRw
Secret Key URI `bag mansion tornado satisfy parent excuse baby pave balance icon avocado shift//3//stash` is account:
  Network ID:        substrate
 Secret seed:       0xb3635c0193c67b42d6759a3a698a423f29321112fe8a09a8cf61ff0249c4d41a
  Public key (hex):  0xd6fef377b9b139a5878fae43b3fd7fb81c57d84451a7b7e6d8618db5b7ac245a
  Account ID:        0xd6fef377b9b139a5878fae43b3fd7fb81c57d84451a7b7e6d8618db5b7ac245a
  Public key (SS58): 5GvbqibTDM7poo3fiam9ygzsVGRxXeNBB3HroE9hEyxAt8sp
  SS58 Address:      5GvbqibTDM7poo3fiam9ygzsVGRxXeNBB3HroE9hEyxAt8sp
Secret Key URI `bag mansion tornado satisfy parent excuse baby pave balance icon avocado shift//3//controller` is account:
  Network ID:        substrate
 Secret seed:       0xe65addb53495417cf2b015c5d7700a0871b8c0af77f4bcf5b3bf8994bc2e9ee1
  Public key (hex):  0x14078c9f8b7a0e0e211b6ebb3a6a3d49c1d729d9d8543216f402c7d6edfbbe44
  Account ID:        0x14078c9f8b7a0e0e211b6ebb3a6a3d49c1d729d9d8543216f402c7d6edfbbe44
  Public key (SS58): 5CWy4HaKQr4GSvYe9QzUDSE2ZW5HdxRV1gvQZcVZX2mKfQQM
  SS58 Address:      5CWy4HaKQr4GSvYe9QzUDSE2ZW5HdxRV1gvQZcVZX2mKfQQM
Secret Key URI `bag mansion tornado satisfy parent excuse baby pave balance icon avocado shift//4//stash` is account:
  Network ID:        substrate
 Secret seed:       0x9a64b7ce4bdb14927da55adf6d2d6e1e82339cdf92f470d5a8a39ea9e9fa5c4e
  Public key (hex):  0x7ec651e7d7b526652f5d8d90af64a2afd89c5c1b14c6d0dca003352b40b7d13c
  Account ID:        0x7ec651e7d7b526652f5d8d90af64a2afd89c5c1b14c6d0dca003352b40b7d13c
  Public key (SS58): 5EvvoanKphM7627EK2BSazJvsVbambqJkYRu8WmjPzjEWfrD
  SS58 Address:      5EvvoanKphM7627EK2BSazJvsVbambqJkYRu8WmjPzjEWfrD
Secret Key URI `bag mansion tornado satisfy parent excuse baby pave balance icon avocado shift//4//controller` is account:
  Network ID:        substrate
 Secret seed:       0x733783c64f1dbc1a2488a24c9901e007bd29857be253d4d2c09eae5a80d8c306
  Public key (hex):  0xc8d1c0c8aefdf8ef28f82865b97ff8f9ff727de7e790c1ad319c188708cb0d68
  Account ID:        0xc8d1c0c8aefdf8ef28f82865b97ff8f9ff727de7e790c1ad319c188708cb0d68
  Public key (SS58): 5Gc1k6KFkKyqF2eiaytAsqACRtn6zzmUCfALJrqx1AfEQSZX
  SS58 Address:      5Gc1k6KFkKyqF2eiaytAsqACRtn6zzmUCfALJrqx1AfEQSZX
------

## BABE key
❯ for i in 1 2 3 4; do for j in babe; do ./target/release/substrate-stencil key inspect --scheme sr25519 "$SECRET//$i//$j"; done; done

Secret Key URI `bag mansion tornado satisfy parent excuse baby pave balance icon avocado shift//1//babe` is account:
  Network ID:        substrate
 Secret seed:       0x16d5124f5c5a36e058eb7ba9f884e545d60944c8c698b841b3bfc041b0bda060
  Public key (hex):  0xb631c16f6d7e351e73a5ba5bf932dc2631805e98164738333c1fa49b889f0551
  Account ID:        0xb631c16f6d7e351e73a5ba5bf932dc2631805e98164738333c1fa49b889f0551
  Public key (SS58): 5GBbMQfaLAnsLzyXcxb8SDG9QVBh8uytRyMZ8X1DW81gi4yn
  SS58 Address:      5GBbMQfaLAnsLzyXcxb8SDG9QVBh8uytRyMZ8X1DW81gi4yn
Secret Key URI `bag mansion tornado satisfy parent excuse baby pave balance icon avocado shift//2//babe` is account:
  Network ID:        substrate
 Secret seed:       0x75b6b5f44dc4cf544e8ddbe3b514780c68be9396a91bbced463619570594067f
  Public key (hex):  0x5c0530e5b166f90185dc3b5c72a4ecb679836e8f44bda774473353c858e4433b
  Account ID:        0x5c0530e5b166f90185dc3b5c72a4ecb679836e8f44bda774473353c858e4433b
  Public key (SS58): 5E9MocwW7kKQtQDfjcghny1SW14DaGoUFnMKZEopuyVFJnQw
  SS58 Address:      5E9MocwW7kKQtQDfjcghny1SW14DaGoUFnMKZEopuyVFJnQw
Secret Key URI `bag mansion tornado satisfy parent excuse baby pave balance icon avocado shift//3//babe` is account:
  Network ID:        substrate
 Secret seed:       0x30a422793b9486072e0b415f7c82935be79c7250990cb3198bd5eb8ff7be4373
  Public key (hex):  0x9cce6d4dad7230ffb947b57b4c893993fbcddb95e7833ecc9f8364b4b122167b
  Account ID:        0x9cce6d4dad7230ffb947b57b4c893993fbcddb95e7833ecc9f8364b4b122167b
  Public key (SS58): 5FcJeanmxXr1GeY6TokHFGD3nqogonzWdpcyP5P58C5u25VM
  SS58 Address:      5FcJeanmxXr1GeY6TokHFGD3nqogonzWdpcyP5P58C5u25VM
Secret Key URI `bag mansion tornado satisfy parent excuse baby pave balance icon avocado shift//4//babe` is account:
  Network ID:        substrate
 Secret seed:       0x694d3d6c3de6c24dd85969b4bfc290c66c4dd7080a432704660e3f062d3dbb0c
  Public key (hex):  0xe4557937ac522fb9b37fdbbc29a095bea2c720825555357075d05bf997a9ed1b
  Account ID:        0xe4557937ac522fb9b37fdbbc29a095bea2c720825555357075d05bf997a9ed1b
  Public key (SS58): 5HE6Aa2kpV6Q4czJDEJaBHHiT3R4LmGATMemDJtRkWEwKUSs
  SS58 Address:      5HE6Aa2kpV6Q4czJDEJaBHHiT3R4LmGATMemDJtRkWEwKUSs

-----

## GRANDPA key

❯ for i in 1 2 3 4; do for j in grandpa; do ./target/release/substrate-stencil key inspect --scheme Ed25519 "$SECRET//$i//$j"; done; done

Secret Key URI `bag mansion tornado satisfy parent excuse baby pave balance icon avocado shift//1//grandpa` is account:
  Network ID:        substrate
 Secret seed:       0x7f589628b36dae92ad9eab08f8fd542d2aa57907455d86c8e005b19aa9ca9859
  Public key (hex):  0x957356455a1fc364f193d137f9fc36780d12b3f2172f5d92f4591189c05042b5
  Account ID:        0x957356455a1fc364f193d137f9fc36780d12b3f2172f5d92f4591189c05042b5
  Public key (SS58): 5FSfFhU9PLjuhGTK7wD6nfngYH8fhCd9EewZERNkuRpGoHCb
  SS58 Address:      5FSfFhU9PLjuhGTK7wD6nfngYH8fhCd9EewZERNkuRpGoHCb
Secret Key URI `bag mansion tornado satisfy parent excuse baby pave balance icon avocado shift//2//grandpa` is account:
  Network ID:        substrate
 Secret seed:       0x7e89464fc62a5b24368aac7c5682054eaecd7d9f732b2ca3fa54446afa976182
  Public key (hex):  0xeec10641cd0ea867a25de2242bd7325f52aff749adcab3ad3e2ef4e120665433
  Account ID:        0xeec10641cd0ea867a25de2242bd7325f52aff749adcab3ad3e2ef4e120665433
  Public key (SS58): 5HTkbQKfGwfqKtxcrvJ6SuHVFbfD2oELM5NPXnWWx18jmwGA
  SS58 Address:      5HTkbQKfGwfqKtxcrvJ6SuHVFbfD2oELM5NPXnWWx18jmwGA
Secret Key URI `bag mansion tornado satisfy parent excuse baby pave balance icon avocado shift//3//grandpa` is account:
  Network ID:        substrate
 Secret seed:       0xe9f47aa6d323ac9883a21bc5a0726050d2f2b6796f4bd2fd3b9d8219f4ccd215
  Public key (hex):  0x292402b0f2422b617f3e767568cf684c463d5b49249da7bad0e6cab62da59f35
  Account ID:        0x292402b0f2422b617f3e767568cf684c463d5b49249da7bad0e6cab62da59f35
  Public key (SS58): 5CzeWz6gdwCUGEuG2radHEcSQguvdnAecHUB1mDMp9DUhbKT
  SS58 Address:      5CzeWz6gdwCUGEuG2radHEcSQguvdnAecHUB1mDMp9DUhbKT
Secret Key URI `bag mansion tornado satisfy parent excuse baby pave balance icon avocado shift//4//grandpa` is account:
  Network ID:        substrate
 Secret seed:       0x08fc50f84e0235a5d7c8c78ac7bebafda5e7aa3f3ac11f2484cd290abefa8dd4
  Public key (hex):  0xb127ab30bd4c546c81f6e2eec5fdf14bc48abe72dfeede437b3d7b7e2017a0fd
  Account ID:        0xb127ab30bd4c546c81f6e2eec5fdf14bc48abe72dfeede437b3d7b7e2017a0fd
  Public key (SS58): 5G4z7jF4BRXTq1BvYb21oUWKsxqHxWS7RZ4FrMG3nQuUYD5B
  SS58 Address:      5G4z7jF4BRXTq1BvYb21oUWKsxqHxWS7RZ4FrMG3nQuUYD5B
----

## Node key

❯ ./target/release/substrate-stencil key generate-node-key

12D3KooWDH8ESQ38Kf55PG3pDTrsuhDBweA3sdjSvxwG7Z5b1xBp
f0740bfc20b61fde1ca92ced1293b9c7bc9ea8459f0ee0e12a9410d72fab0be7%

----

## im-online key:

❯ for i in 1 2 3 4; do for j in im_online; do ./target/release/substrate-stencil key inspect --scheme sr25519 "$SECRET//$i//$j"; done; done

Secret Key URI `bag mansion tornado satisfy parent excuse baby pave balance icon avocado shift//1//im_online` is account:
  Network ID:        substrate
 Secret seed:       0xa0661db03efb1c2fe7e98e06045869dd35b04adbd8c399c0847c4e0845483509
  Public key (hex):  0xb84cebbb0fcc221516c659375d085feb80603901f9ee7418551103cb92870115
  Account ID:        0xb84cebbb0fcc221516c659375d085feb80603901f9ee7418551103cb92870115
  Public key (SS58): 5GEMX1VMhner1DonB9gyrFNQC9pX1HUz6ixstxLEa5i8Pt1C
  SS58 Address:      5GEMX1VMhner1DonB9gyrFNQC9pX1HUz6ixstxLEa5i8Pt1C
Secret Key URI `bag mansion tornado satisfy parent excuse baby pave balance icon avocado shift//2//im_online` is account:
  Network ID:        substrate
 Secret seed:       0xd18d9c91adee0622fc8e9685fe26061970fa72527fe6d4fea710059c0a4f4abc
  Public key (hex):  0xcafe668138b13be9b1b634120502af5b94bfa5f84e1a60ba27180e1c31d7b03d
  Account ID:        0xcafe668138b13be9b1b634120502af5b94bfa5f84e1a60ba27180e1c31d7b03d
  Public key (SS58): 5Ges6uKoMStJiBG6U9SyfXRBNMnaiZeCAC6uMdNQ11zfFu9p
  SS58 Address:      5Ges6uKoMStJiBG6U9SyfXRBNMnaiZeCAC6uMdNQ11zfFu9p
Secret Key URI `bag mansion tornado satisfy parent excuse baby pave balance icon avocado shift//3//im_online` is account:
  Network ID:        substrate
 Secret seed:       0x4ec6e318963bcd050583949e453a39b7749f83b5659a8b1a825d0446b3ae193e
  Public key (hex):  0xf0dcc236e3f42c33fadebc3d0598c9b44a65a1ddf43798eb066018512d718f54
  Account ID:        0xf0dcc236e3f42c33fadebc3d0598c9b44a65a1ddf43798eb066018512d718f54
  Public key (SS58): 5HWWvokRgoKnUVqNUzpVs6tzii64DoRvJ5sygfk93HTPUggM
  SS58 Address:      5HWWvokRgoKnUVqNUzpVs6tzii64DoRvJ5sygfk93HTPUggM
Secret Key URI `bag mansion tornado satisfy parent excuse baby pave balance icon avocado shift//4//im_online` is account:
  Network ID:        substrate
 Secret seed:       0x63e496b2a73a3250d1db3071933c2ba96ec2e940ae911addcf05bb7e51da2648
  Public key (hex):  0x143ab765d99f620afa1326e6843c9e1f677c22b55a522ebb905ed49de76a0236
  Account ID:        0x143ab765d99f620afa1326e6843c9e1f677c22b55a522ebb905ed49de76a0236
  Public key (SS58): 5CXEFshj3WjGp5nNLicp2hf3CtW7CBu4CxRFNbbKGqLgrjxQ
  SS58 Address:      5CXEFshj3WjGp5nNLicp2hf3CtW7CBu4CxRFNbbKGqLgrjxQ
-----

## Endowed account

❯ for i in 1 2 3 4; do for j in endowed; do ./target/release/substrate-stencil key inspect --scheme sr25519 "$SECRET//$i//$j"; done; done

Secret Key URI `bag mansion tornado satisfy parent excuse baby pave balance icon avocado shift//1//endowed` is account:
  Network ID:        substrate
 Secret seed:       0xc30c04c4ec772cad708ab0c00c48a093a64099c8447b784e416ae99840d709d2
  Public key (hex):  0x94683996ba380b51d678b2f2ac57a3c46f3b552a5b8c08b6c9b05c2f67ff901a
  Account ID:        0x94683996ba380b51d678b2f2ac57a3c46f3b552a5b8c08b6c9b05c2f67ff901a
  Public key (SS58): 5FRHuTwLJNacWiVHeipJdYTQW8431hL7fBFFn8EVLKZGV3T3
  SS58 Address:      5FRHuTwLJNacWiVHeipJdYTQW8431hL7fBFFn8EVLKZGV3T3
Secret Key URI `bag mansion tornado satisfy parent excuse baby pave balance icon avocado shift//2//endowed` is account:
  Network ID:        substrate
 Secret seed:       0x5fc6e92a189cb83a944ba03baae0b26d33afe01092e2bce1b1b169348427b59d
  Public key (hex):  0x72008fed483bb939cd816799682d5d30ef16669e091f5fce2e909b3d8c623f4b
  Account ID:        0x72008fed483bb939cd816799682d5d30ef16669e091f5fce2e909b3d8c623f4b
  Public key (SS58): 5EeBUyATqNtcopfyU5CqYTPt8PctWcHD1mzN5mYs2FF7SnQc
  SS58 Address:      5EeBUyATqNtcopfyU5CqYTPt8PctWcHD1mzN5mYs2FF7SnQc
Secret Key URI `bag mansion tornado satisfy parent excuse baby pave balance icon avocado shift//3//endowed` is account:
  Network ID:        substrate
 Secret seed:       0xcaeaa416f559507030dc1e61c0a90658ae9a1247225cf7c43dd5c8e0d2be5a93
  Public key (hex):  0x86d9294a6c2b6c2b0ad4d80163db108465b66d11258447f68cea63d9bff4407f
  Account ID:        0x86d9294a6c2b6c2b0ad4d80163db108465b66d11258447f68cea63d9bff4407f
  Public key (SS58): 5F7WnRvszr3fu7czADjnsdCfLdb9HSfmNEM7J7VAgcYbKL5F
  SS58 Address:      5F7WnRvszr3fu7czADjnsdCfLdb9HSfmNEM7J7VAgcYbKL5F
Secret Key URI `bag mansion tornado satisfy parent excuse baby pave balance icon avocado shift//4//endowed` is account:
  Network ID:        substrate
 Secret seed:       0xcde06f8d148b12563c7c3c6e00b96516a300645fbaac6a084f3b7cdcc45a3c22
  Public key (hex):  0x44f23d7c7eb7fe9db27b25e5b8c4ae1348693ac47a032330b291bb6ceeb36e45
  Account ID:        0x44f23d7c7eb7fe9db27b25e5b8c4ae1348693ac47a032330b291bb6ceeb36e45
  Public key (SS58): 5Dd75E8UM2TqKRKEkBx4XjPyGJBWdV2xPU68s9H35LKhUrn7
  SS58 Address:      5Dd75E8UM2TqKRKEkBx4XjPyGJBWdV2xPU68s9H35LKhUrn7

# root key
❯ ./target/release/substrate-stencil key inspect "$SECRET"/fir

Secret Key URI `bag mansion tornado satisfy parent excuse baby pave balance icon avocado shift/fir` is account:
  Network ID:        substrate
 Secret seed:       n/a
  Public key (hex):  0x2ea8f664f8d88208742f7b2f0df87d11e011eb2c9814be64b570be42debba217
  Account ID:        0x2ea8f664f8d88208742f7b2f0df87d11e011eb2c9814be64b570be42debba217
  Public key (SS58): 5D7tFaeJ6cfYP2rYZ2oZP6WAtB8LAwLymNHQYCxBqqEmwgjp
  SS58 Address:      5D7tFaeJ6cfYP2rYZ2oZP6WAtB8LAwLymNHQYCxBqqEmwgjp

# start chain commands
for i in 1 2 3 4; do for j in 1 2 3 4; do ./target/release/substrate-stencil key insert --base-path /tmp/xiankunvalidator$i --chain xiankun-staging-raw.json --scheme sr25519 --suri "bag mansion tornado satisfy parent excuse baby pave balance icon avocado shift//$j//babe" --key-type babe; done; done

for i in 1 2 3 4; do for j in 1 2 3 4; do ./target/release/substrate-stencil key insert --base-path /tmp/xiankunvalidator$i --chain xiankun-staging-raw.json --scheme ed25519 --suri "bag mansion tornado satisfy parent excuse baby pave balance icon avocado shift//$j//grandpa" --key-type gran; done; done

./target/release/substrate-stencil --base-path /tmp/xiankunvalidator1 --chain xiankun-staging-raw.json --name xiankunvalidator1 --validator
./target/release/substrate-stencil --base-path /tmp/stencilvalidator1 --chain stencil-staging-raw.json --name stencilvalidator1 --validator

for i in 1 2 3 4; do for j in 1 2 3 4; do ./target/release/substrate-stencil key insert --base-path /tmp/stencilnode$i --chain stencil-staging-raw.json --scheme sr25519 --suri "owner word vocal dose decline sunset battle example forget excite gentle waste//$j//babe" --key-type babe; done; done

for i in 1 2 3 4; do for j in 1 2 3 4; do ./target/release/substrate-stencil key insert --base-path /tmp/stencilnode$i --chain stencil-staging-raw.json --scheme ed25519 --suri "owner word vocal dose decline sunset battle example forget excite gentle waste//$j//grandpa" --key-type gran; done; done
