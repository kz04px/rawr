use super::{
    bitboard::Bitboard,
    colour::Colour,
    mv::Mv,
    piece::Piece,
    position::Position,
    square::{Square, *},
};

const KEYS: [u64; 768] = [
    0xdc2ff8a63cec2624,
    0x0559f63989701ea0,
    0xd26d7aff0ca842a7,
    0x5c2d39d2999567e0,
    0xed7f6cb9df6b4f21,
    0xe713e0f99aa6fd19,
    0xf8ab3dba736e8e48,
    0x955731258899a285,
    0x6a998a29c42a344a,
    0x5ebdee69de0afbd3,
    0xeb66e2a9247b3697,
    0x6bd74d4d8350fcf0,
    0xef98ba079b0aa680,
    0x71ec9b51577d830f,
    0xfd42f5d9f02f3dc9,
    0xc3570a78807a08dc,
    0xb6270acc8ab69c9f,
    0x4c445cc497154ebd,
    0xa190b372aab95f44,
    0xcbfae0c9e5e685e5,
    0x891a490831e619c2,
    0x0c9983565d9f2a71,
    0x3fcd8ef19e21e54b,
    0xacd2439a6b5812ee,
    0xa141b58d879e750c,
    0x6dcea8d27b3882bb,
    0x127ff8effb529a4e,
    0x4e09eeeb9cb3237b,
    0xc86a17a343edcc05,
    0xfedb959d03989381,
    0xc4ae8f1698335ea6,
    0x81207964f5692558,
    0x52002b6203635931,
    0x0296a6d95a4e7f94,
    0x5429bb9730722345,
    0x76463cff61bdb51e,
    0x976e6f12bd9c77fb,
    0x7ae6e3cc8ab016f9,
    0xdf5ba255efd331ba,
    0xa39ec36ae33d1813,
    0x2ae2f3af546c7f6a,
    0x8c7859fc7b1657c2,
    0x3c525894feaddb15,
    0x6c667fe7ebed4495,
    0x03c4315c06492dc5,
    0x191ac2dedc7e12f6,
    0x3a6974d6d1b1dd79,
    0x0fa92a326272e7a6,
    0x746326b0e432cb00,
    0x67cbc4e75c15c899,
    0xfdf54ffb60542515,
    0xaf9ce3f0bb289304,
    0x0cdd5824bf8153cc,
    0x40e63d4d7a36764d,
    0xfa0146a666394c85,
    0x6ab1ab98b37106eb,
    0xb065957a0443be32,
    0x2fb6fae461c3edf3,
    0xe2a237216db79faa,
    0xcbf7a3896e9f52ef,
    0x4706770807f98528,
    0x905181951865b1ee,
    0x17f2935f8fddfc7b,
    0x4c13c838e73ef45c,
    0xa029c797ae3c9565,
    0x8bf53ee873eceb5d,
    0x92e384dc41c7556c,
    0x7d0cf03bbf52978d,
    0x280b7c92c92c1eaa,
    0x58792273bb06736b,
    0x21eb9fe51f369ca0,
    0xf5f108f7379b1643,
    0x69f192d8570fabb3,
    0xca03b72f6ba6bd0b,
    0x12461b428854182c,
    0x4ee58a2f66e271a7,
    0x846acf515a728b92,
    0x5e5e682a2e5b1d87,
    0x9ba4b4e9c182cef6,
    0xae4889df2fb7fe54,
    0xdc27f371b02f8dc1,
    0xe342f5891b9de92f,
    0x0a4157867d51883e,
    0xd6b36f03fffaea5f,
    0x1b63bceebc6040cd,
    0xab5d5c0a158f7be5,
    0xbbd48638f2e75e88,
    0x8571d6585661f42e,
    0x20267c1879ee776a,
    0xa016374febb16b8f,
    0xfbca2138143f94f6,
    0xa3dd7d181f886227,
    0x0843327b9acbbbbf,
    0x825b87efe81826ee,
    0xbc500a629541a487,
    0x6a8e4723e1479144,
    0x89242cf23c8d1bd9,
    0x844c584367460e27,
    0x9120b3cf7cc5affb,
    0x13ef35867057c125,
    0xd594568bde2298d1,
    0xbb7d41611e7e432e,
    0x9483ef1576648e11,
    0xd2379f083aa05e17,
    0x79a0c961002b00ec,
    0xb3ab1a68cbb88288,
    0x737dc2b6f89e30f3,
    0xc34bcb8bee31677d,
    0x1f0a8030b8a0ca6b,
    0xa3fab76a90a186e0,
    0x380cf13d10323e19,
    0x93510039cc57d05f,
    0xac7ed2de293bbbc1,
    0x4d6d6e8947c5c430,
    0x56c69412d9541b96,
    0xb0697229ab27b6b9,
    0xd9bf3985c80ced34,
    0xb426109bcb92142d,
    0x66164e549fffd109,
    0xf88d5d3dbccced33,
    0x90917b3586928a2e,
    0x7c71e80e834a258d,
    0x9af7495099b78953,
    0x04936efc3452a663,
    0x7ea10f54665faf2d,
    0xc038720bda94490e,
    0xa4597f2a3ec902aa,
    0x1ef546f276868b34,
    0x5f5636c6d09e5312,
    0x5aca750771e921a3,
    0x7db03e31ea9e8663,
    0xf0be99e7cf990bb4,
    0x30a60e322fadf420,
    0x5a417fe880f80466,
    0x26a66d8066e1bfa4,
    0x75e00f5ce7873e6a,
    0x6886bef1e612a07b,
    0xa72b3467104f313e,
    0x4622738764d81a29,
    0x14ba7f9f79ee106a,
    0x154ca77dde21ac25,
    0x61f05554edc5818e,
    0x662d87a9ab34f12a,
    0xdd3d808bdd5e0cb2,
    0xa7bb241c65c2c045,
    0x57c26933f0611e2d,
    0x53a2638b51a7714c,
    0x59388980571919a4,
    0xde7531fba10e8bc8,
    0x26f5217842cd4625,
    0x9c6c58bf1d46b173,
    0x3e55a971def0b20d,
    0xeda9b250920b3c48,
    0x539f08bca2f75700,
    0x8916838540073d25,
    0xe0a02ff51a87325e,
    0x99e353ff6c93bff8,
    0xc3bed3a6dbdfa45d,
    0x483832595fa7f78f,
    0x0a421156a23317f6,
    0xd114208f8e6fbaf1,
    0xe5aaef08e326e434,
    0x956cf1d5865b7e7b,
    0x26880c312b44dfc2,
    0x53142086661cd4cf,
    0xf56a6693db27825e,
    0x54659c10cded8697,
    0x7d8b3935152dc0ec,
    0x8878e8d9a0fbbdb6,
    0x2f7fb911d7faaaf6,
    0x00b615bf54da72a5,
    0x1730cafa621e2db6,
    0xddd782db9f31790b,
    0x417a19fead36f671,
    0x2492fb5fb469a163,
    0x9af0095563342f5d,
    0x78bba9a48787d88d,
    0xf3ea217579ac8420,
    0xfabf3714e1ee331a,
    0x0dbd1f721c1269ad,
    0x1a6cdf861afc57de,
    0xde83f62d1bbc8873,
    0x6fdb16a342c98b23,
    0xc19db8c5a48dce32,
    0x1ab27519c7227d46,
    0xea6f36dc86dd743e,
    0x58e532f89d4e7700,
    0x49df959ae34ed6fe,
    0xe9be50ada6ab1b93,
    0xef87079eef955075,
    0x817fab15583978c1,
    0xf2311ee83d6ab0d4,
    0x8fb2612a6c03c3ea,
    0xc4b9d17aa8a6e6a6,
    0x0a171b476ae58160,
    0x985d2e71015a8cf1,
    0x2f967c78ab448560,
    0x4a19c7af1f30b1e3,
    0xa230f709d6d18e5b,
    0x243f0c4eb9cf03c1,
    0x64cd420f442c7ad5,
    0xd1a2d44095ca6194,
    0x637617693b161fa5,
    0xd667481f87c00c31,
    0xebcd0b7d049ee6a1,
    0x7fab54e8c45e7e69,
    0x237434e8a92ff28b,
    0x2ecf70a5109001df,
    0xbd36940b0ad6b20b,
    0x45a36239f855923a,
    0xd3eede87694d2236,
    0x6c97748679c3b048,
    0x3e3af0aa7d4f3cb2,
    0x0c70e5a76a54be91,
    0x4b8e9cf0a34dd41d,
    0xd0848f83bd46efc3,
    0x1ad099447a3c0bc8,
    0x1e64d602306ddacb,
    0xb443fae2725469e5,
    0x55d90390692aa0d2,
    0x5b4f1d46a4ec52e1,
    0xd0a715a5f944e735,
    0xf14f9905849db456,
    0x8362c5eb2bcc1254,
    0xb63e051433bfd46d,
    0x9c0465a305c0188a,
    0xd8b86e81388d3d83,
    0xd0d7f27148d53468,
    0x5026fd2b8fbfc22a,
    0x85f1fc267004c5b9,
    0x3a0dd96213f25fc8,
    0x5c7bc38a17ab51d6,
    0x425791d3bf6f396b,
    0xbc91af8a27c9974f,
    0x9a062c14b840bdca,
    0x3de8bde4448f842f,
    0xc0b7aecb7bb6c822,
    0x445442647651d908,
    0xd6fcd473ead3cf85,
    0xb740a12c10211fde,
    0x892c3e84fd8d99b9,
    0x2feeab0d08051bcb,
    0x872c3623a5277981,
    0xea84f9cf3a7b8ba9,
    0x6fc7d5b9df6a69ed,
    0x8e096ecd51873f58,
    0xa4707a3e66853db1,
    0x6e1bf6f5f12f2567,
    0x52620f35b51e4125,
    0xc300044389dfd1b4,
    0xb845bc37489674a1,
    0xa026f70670d48c11,
    0x6eda348da1385118,
    0x9b7fe877d239001b,
    0x7b7892977d28e6b4,
    0x53769a00a31f49c4,
    0x8aefdf1579ac2bae,
    0xf5ef62fc9848d463,
    0xd514985607d9b4c2,
    0x19ec97092a2ce853,
    0x1f1619d67474679f,
    0x120deee81af8c06c,
    0x169110fb39949114,
    0x4a57e1f2d2a2d1a6,
    0xa19947988389ba29,
    0xbe2cc01545adb660,
    0x39b26e14ec1c8b0d,
    0xf1265e28d7b76526,
    0x82ae8ef13e2ddc23,
    0xcb540f5e72a981e0,
    0xb38cf44adbdf5e20,
    0x5dc0219bfbf9500d,
    0x5f796dd4a4207ce1,
    0x6f0ec4a4e6f6fd84,
    0x5e3712dc4ace737b,
    0xff488c86a3300adb,
    0x480bbb175f5e719a,
    0xd6a52754db39218d,
    0x39e80c37a2a184dc,
    0x709aab2d88765a17,
    0xf8b8d68482db60c3,
    0x9e7cbf4a1db47c78,
    0x831f486d9783ea6f,
    0xc61ac1da4cc57fd7,
    0xdd891ab8bf11ad5d,
    0x6fb9613a6271e02c,
    0x7fd935a2c150365f,
    0x5ac17c9bcf918d92,
    0x5b84c2bbb22ba91a,
    0x0b7aa9fe59549830,
    0xd4cc056f7f77bb01,
    0xa7457e3f31657983,
    0xb634739f8b9e9010,
    0x0a32047ae4ec93a6,
    0x0c74b22b911a79b5,
    0x18a1b631b1f1e415,
    0x6d5f5fc5c51f9495,
    0xdb3f048984d0bd54,
    0x2a782e7e1e296ad9,
    0xdebb442cb2c0f8fa,
    0x993a3b036ba77006,
    0x16abf8928d09462c,
    0x4c95d511152f43a2,
    0x7bffd7c964e285b9,
    0x1bd8accaca413b07,
    0xc527337c0064000a,
    0x3255581f2ef06006,
    0xcce266986024f7df,
    0xa8c9b3d12d26765b,
    0xae30b23e7a958ca2,
    0x8a7bd0c121c27bed,
    0x286d1a7ea6434425,
    0xcb907808f03a7da3,
    0x44abb5dce2397049,
    0x13252696a478efa7,
    0x908c902e00066fcb,
    0x9e8526356876425f,
    0x1ba46df171cc0a0c,
    0x1878954c8802750c,
    0x071fc1e7a4ed447a,
    0xaec40ce0eed5263b,
    0x64dd67c874ea6c61,
    0xdada4f998309179e,
    0xa93c4ffe906ca566,
    0x66dde7ab2fe26d64,
    0xe046464216fb7de8,
    0xc4e6df83f7cf51fc,
    0xcb295bacd9d1098e,
    0xa9097bcdd13f4143,
    0x9ae1c371fda7cd98,
    0x3f8ba137c482c6b4,
    0x68c8f7e6b397af36,
    0x8255e7f3732d6ae8,
    0x557ca8dee89f0a12,
    0x8997604f26e22442,
    0x7ba15d35452dc792,
    0xcf2a4eb61561da01,
    0xc3ed42386bbef005,
    0xcfd6aba2887be3f0,
    0xe2b486526ce37df4,
    0x83fc1a1bbfcca86c,
    0xe5a684948f016eab,
    0x8efa276ecdba1356,
    0xcda106d44bc8e5b2,
    0xf8994056897f7f99,
    0xe55dc427c312d6f3,
    0xeea01916f496cd30,
    0x8955f0a72a4a4b40,
    0xacccc97557f420e3,
    0x0d47bff5515b2eae,
    0x5f8cc03b81200120,
    0x1a5e81cc79a8f23e,
    0xb2ef9e5cbd36bf79,
    0x2385e884d115c80d,
    0xcea6d3190472afb4,
    0xe716f7bff3a16129,
    0x226af0b714c2ebe5,
    0xf677cf32abcb463d,
    0xcb9f908b66c63f34,
    0x22dae1f0d5c540f4,
    0xcb98c38586253b0e,
    0xb7f002b4d36020cd,
    0x27ec86f7993cd304,
    0xbe8d5c767f873293,
    0xd5571f9d500eddf0,
    0x3c6d85ed9e23b68b,
    0xe5ebc6e79bb10684,
    0x15fa52b05186b0a5,
    0x4d76f9b87175591c,
    0xfa18409ff9004b0c,
    0x26131ed5b7044c41,
    0xba5bf6afe15c3b99,
    0x10024551ca906d03,
    0xe84f1d3e255f4aa7,
    0x3a2dbc5a1aa4a5cf,
    0xfab875723ca12f18,
    0x30d10395ecd04b5f,
    0x6437bcd083adb5f7,
    0x64103e422a33f778,
    0xeb2b87ddc64431fc,
    0x5e6ce7b4e9e097e0,
    0x7e61fca1164bb16f,
    0x943c05740dbe7e5f,
    0xde0179a0dedfbb33,
    0x43b034e6af618d19,
    0x11f088b35ea38d09,
    0xf6588fb819e577a0,
    0x83dafaf87fc19cc9,
    0xd2fe1260cd315d0b,
    0xc37dfe6b603b9064,
    0x0a30dcf430a75521,
    0xf41e0f54f56a69f9,
    0x3c8ececf6d3ebc36,
    0x9867efb15f563914,
    0xa4b7fac3a207ec5d,
    0x25b247f89bb339e1,
    0xe205a9afd7e2e657,
    0x4ebf163e52bb7fce,
    0xca2ea8a23ad2f80f,
    0x7a44dbd615cc1d22,
    0x737dbce7e3f60b25,
    0x805cd307fe2ce330,
    0x3c93438a349ef0f3,
    0x8c1d4d8daebd6350,
    0xbbc7a5595af78b3b,
    0xfb5043a413fcc39b,
    0xee9f0dfba7d6b078,
    0xb501d251ffa1ef92,
    0xb5e62775a06d0993,
    0x410b256250dcebef,
    0x72788da427266f17,
    0xd8b1354074fcacf7,
    0x716ad47e67075371,
    0xe858f0f8bc2285a3,
    0x89f72e147982dfd3,
    0x18a9b3e8cebdd816,
    0xca78813cb55f7abe,
    0x87b3077dc06e6c42,
    0xb9f008f185013d10,
    0xf6ea12b8e5330b7e,
    0xc788fda38ad51cd6,
    0xd3e637648efc4948,
    0x7d149c2f0da559cc,
    0xcc99dc3d27627305,
    0x35f61408304c84a0,
    0xa93769c7419ae7ff,
    0x1795d2f43b0b8f01,
    0x540120b194fdafaf,
    0xd1cecf107cf38bcb,
    0x081b78042d283ebe,
    0xf52642e6804b9b3f,
    0xaf55eba7126835d9,
    0x7d08118526018b2a,
    0xd38e4252836044c4,
    0xa414c1638c97348c,
    0xabb919e612c95f95,
    0x1b4a81d12bc0cccb,
    0x55522a88ad2753bb,
    0xeff26806c412f635,
    0xe4e9c76f6dba4163,
    0x352853c2a0221faf,
    0x87b813732ef5f5d5,
    0x22eb2f87f8c7b2a0,
    0xef7f3abc7968e215,
    0x2c33e0a2ab00229c,
    0x82e88c8b7f81c545,
    0xebd60994b6a60b70,
    0x6bdb032d60d283e6,
    0x9d7f67c226ad7086,
    0xdda20c830f05678d,
    0xdb26493e677b5dd0,
    0x7c9b8487f6827a7a,
    0x1248cbfe1396c5e7,
    0x7174fcf8b2a4793e,
    0xe20bf4cf11dc72bc,
    0x80fb7820253710f4,
    0x641c39bdbfe54a52,
    0x80f2b2da0c5df780,
    0xe4778643a88e1275,
    0x43b930a7ec163429,
    0x2335ca4a43077ad4,
    0x22e743627cadef9a,
    0xe9c77b3e3784ba1d,
    0x2a12bf4469596651,
    0x74b93a428af43127,
    0x56a39fb867daf989,
    0xe314c941e2a78d6d,
    0xf10554f7eb53dcba,
    0x16cd3ad0df07956e,
    0x4c1eaef1d77a2aba,
    0x4fd4668cccb75965,
    0x0fab3cbfbb5bbfe2,
    0xbfdb700290cd053e,
    0x144f6fdd80812732,
    0x10ead83f5b840eed,
    0xe683b59b626bc57b,
    0xd1fbdde4a32a5b14,
    0x8198fa18f760e770,
    0x8e691b3848701176,
    0x7c44a29d28b03f17,
    0xce8e762a892dbd91,
    0x64c11f2b86753431,
    0xb85997f5aec91e15,
    0xd369fb08f49e7555,
    0x0fe08b49fd030fd6,
    0x510541819d20169f,
    0xc4e052722f6a6004,
    0x60efca887f6e9513,
    0x550ad1bf1595180a,
    0xa12914aa5c7eaeee,
    0xb8b2a8e5a6406a70,
    0xb5e4a10d5d62e53f,
    0x79d03403613318f9,
    0x8231e613c92ff520,
    0x4637680fc504238b,
    0xe38faf733a502f8c,
    0x0e06f5b6bf120ec9,
    0xe5ffefbe8c0bab72,
    0xc1cb495598af0365,
    0xb5c9df9245df1bde,
    0xf998019abcb53d1b,
    0x926b204e6072276a,
    0x2635e17fbf98e961,
    0xf163c2fe45d54712,
    0x3a2d329bf655246b,
    0x6396887f4f3d48bd,
    0x4b074af56bfb0a8a,
    0x80b4f379417785c1,
    0x30d8788dcad4aa1c,
    0x62c9d33c13f8fc48,
    0xbf97d7fbca99e011,
    0xf7547b0aba5f6a46,
    0x8b5df22fbba0906a,
    0x746be4be6150e5cc,
    0xf7e253bd172528f5,
    0xbf96afea7c45c35b,
    0x42c4b14e1238e658,
    0xeff5f2b70574a366,
    0x5cf9dc643a782323,
    0x5a553694611488ec,
    0x81f48d44438e5a3c,
    0xfa61286dfb0301c3,
    0xe0df7142e2289f2b,
    0x6169b4f7d4612ad8,
    0xa970833c4cbc675e,
    0xf8dac283ed5a49e2,
    0xab58d30f0fb9f7fe,
    0x840694de5d94a273,
    0x1fa8927ed7ec671e,
    0x424267f21cdbd77f,
    0x0734408fa1ab7ce7,
    0x26554bb3a10bb2da,
    0x538ec1f40bf053b6,
    0xb719a24f91af9a8c,
    0x07257cef144ad9b4,
    0xafbacc9b98f73206,
    0xcf5df84d99be0a80,
    0x1646832bd5bf18bc,
    0xb79e7afd769a0fea,
    0xeb8c6208c07a1668,
    0x44c54103e43bb011,
    0x8dde70dff1b0677c,
    0x268d2ffb36ac140e,
    0xb87f66476c56d569,
    0xd8c1069bbd257b89,
    0x27caa92ab5749d2e,
    0x2153473fa9e3c25e,
    0x4cd6dfeca30502a9,
    0x6bc641608e568b39,
    0x957724d8cd4c30e7,
    0x34960f604137b5ac,
    0x92e488d12f84589b,
    0xa7aebd9d0a029a47,
    0xa44f5227399f9320,
    0xd5245ded920ddf20,
    0x07f8fc41b0770612,
    0xbca83ef57d0b6219,
    0xec9c8a698f3bc9ef,
    0x0ad3aeaf93ea69fa,
    0x980018fa05d333a3,
    0x95de9d86757f8cfc,
    0x992f0d55e19357ac,
    0xfa41d3e66efe2178,
    0xf5dad9069cf3c2bd,
    0x1da0d1e61118b0e1,
    0x6127d624d5b2aabd,
    0xc5ae68ca2a2b1305,
    0x769e294a22eed27c,
    0x103c190902009a47,
    0xfaaa0182929c8807,
    0xdcce740e471a9d73,
    0xcb2d7d83f40a4a78,
    0x3429390398cc52cc,
    0x810a5d6727890d27,
    0x33871a968d826bb0,
    0x1aa3737fcfea77dc,
    0x6b7654ed8025baeb,
    0xce3680474f0b8828,
    0x697030d92a67be88,
    0x3e1a875b4b1e46ef,
    0x8f6173324e8b917c,
    0x727cfae95310f908,
    0x6ff1ecdd48532d40,
    0x5bccd0f47e130d2d,
    0x4a39125f40094e2f,
    0xd76cff794cc3bf7e,
    0x5d47c4bebc546910,
    0xa4895203f7236008,
    0x338f925baf9ca8fa,
    0xbd67677bb4c91c56,
    0xf772c526c48d01fe,
    0x4ca0c64cfea7ec92,
    0xfb847c992c6e08b7,
    0xa84e7fe0be8ad657,
    0xcd73ff32dd0de78f,
    0x102f597fa0635d38,
    0x849943241ffc3f22,
    0xf2071c2153251749,
    0xee892fa056056ab7,
    0x99278a1ac1f765b0,
    0xec9045156faf4e6b,
    0xf8a011f0e9c848e2,
    0x0650feedd3ef4805,
    0xea53a20149e189a0,
    0xd701f8e4e26f2cb0,
    0x7271ab88d6424ee0,
    0xd974fefb8badc79a,
    0x3e1ded256e05cb48,
    0x53b4f9f463a6e827,
    0xfba6dde3180eb10a,
    0xa6d18124308b5460,
    0x5a5af84a74e48f9e,
    0xda6e9e55ea8c9f75,
    0xf62c0a721ab17717,
    0x7bd5ed4095d86bb8,
    0x5a2ff834302e06e7,
    0xfa816e116d742505,
    0xce565758dd2a9d41,
    0x466f3953d74f5446,
    0x4abb7463664b735f,
    0x76db9c7cce08a55f,
    0xeb29ad9c556b7c2a,
    0xd2fe7131e22487c7,
    0x291d9290b5ca861e,
    0xecb1ae863bb0e9bc,
    0x190ab938a91a0cc1,
    0x85771e5d2fde6f7b,
    0xf8da8a3ed564a31c,
    0x55aaf0996e671ba0,
    0xdd16ae74ab445b2c,
    0xb8b38ca6b85fcd79,
    0x75c27be7def724f1,
    0xba32927b9e37ab57,
    0x4f5f48a08c4814ee,
    0xd7ade4d8c1b62e96,
    0x3fbe24a6dad02559,
    0x9dfb7587ff4f1c31,
    0x24c3197919ba2ff3,
    0x95349be7caf17e2c,
    0x1b07f8451987be3f,
    0x7cf9b530641464af,
    0x4ea118dc9c019cc9,
    0xe05c19f4a3d0240f,
    0x114bfd9278a15cbf,
    0x37859852289b0915,
    0x147b3c47b154a6bd,
    0xd5aa7ee5012aea62,
    0x15c1ea5c9f47eb3e,
    0x8cdb035119858a1d,
    0x6a0cf0939c6076ae,
    0xf2696a2bac54c807,
    0x220b86c224c3a7d3,
    0xc8ae75361a73bba1,
    0x616b7636715e3344,
    0x372167423e06d729,
    0xbcb172edaccd9d6a,
    0x9018157d81b04fac,
    0x21c447a5a2fac014,
    0x706135ed929fd833,
    0x85ddb1f145ceddfe,
    0xc87a1c85ecb6c1cd,
    0xe19ecb5b7065ccf9,
    0xf8320ca6b4616f5a,
    0x2c2b8371859b9e0c,
    0xe4e3235fffc8da3a,
    0x205774eb5ba8fc96,
    0xa16576546f60b4fe,
    0xda1e4c0fa1341c69,
    0x445a7c7a4e0e76a1,
    0xebfe910caac9e9d8,
    0x1c4476d71c44d616,
    0x58e41b2414dc9095,
    0x22662218967501fe,
    0x37c025790fde3825,
    0x4c0f4500910ae566,
    0x6501159782b7db66,
    0x8c2d3dc65705b17e,
    0xafcbd681bf950fee,
    0x1707e85bb4de2844,
    0x3f8ab365afe8531a,
    0x53f419a79f671072,
    0x29a178aa1575f772,
    0x276bcd5247fd7572,
    0x3d6f6dcd68922ec9,
    0x90374006c6858420,
    0xe0b4001154c90b6d,
    0x0299f6f924bbec16,
    0x5b89092625ce0607,
    0x19ee7e076b46a973,
    0x61c0bcca933ec5e8,
    0xd5487f0a58cf7472,
    0xdbe6a113c263f56d,
    0xbbf0e23e7cc0f2b4,
    0x25a273501e7521fe,
    0x27c0a5f2d0c23a6d,
    0x1976476b9466987b,
    0x342058294462f99a,
    0x20f66157de610b49,
    0x57f4052440a12c91,
    0x0f16eb641b49e1f7,
    0x9e9f546726029d87,
    0x32ae362173f8f9a0,
    0xba989d66c4706ce5,
    0xa28a4f59f3fecdd8,
    0xa6530e0778a8cf22,
    0x92515bf0d8ee672b,
    0x20c1213da6cc7158,
    0xa06b0787aaa88019,
    0x6d7d58a4a498d4b3,
    0x7f9512a88180e2a2,
    0x2fa308794dabb4e5,
    0xfaa5f9970b9c2fb4,
    0x190284a823afd72c,
    0xdc7d7ecf1565e7fd,
    0xafda64ea0bb159ed,
    0xf701d6a6f4b17bcd,
    0x36168d0f9ee6234b,
    0xb1945ab0bbd79c70,
    0xea0b49d82be439f3,
    0xaed68dfd4fca2eb5,
    0x7fdf5f454abf8244,
    0xbcdee25f8af06a4a,
    0x70785891d196c522,
    0x45e46e2cafb65a98,
    0x1be577e334eec632,
    0x3bfb21740bcdd36d,
    0x592590b91fe55241,
    0x27041a787c45b344,
    0x3ed2f72323c6bf9a,
    0x81efb743b09be910,
    0x46bcf6e288f3ce92,
    0x5ed298bba0ee8128,
    0x15f64c6d5971beff,
    0x5356b3635af000d8,
    0x1f878525373be74b,
    0x4deedddec4ef0a0d,
    0xc5a19e80e68af77c,
    0x96708995f31e1d85,
    0x8a531195e544b60a,
    0xb3a28d78bcc2894d,
    0xf229a2be9dad9863,
    0xe2134105a0c24d4d,
    0x132bef077a878579,
    0xf1708e9d561af94e,
    0x5748d965997244ba,
    0xe25d1d04e5ffd744,
    0x1eb3600fab278b25,
    0x9a4e9b1c4831fd31,
    0x861c0cd3ab03702c,
    0x780546a93c0bb8a1,
    0x7a52fa72a28d9077,
    0x101d5bd85b0b3203,
    0xf8e516a05f3a8667,
    0x18da161095463efc,
    0x982a23e69a51f64c,
    0x33b271fd3eb46ee6,
    0xadff7e5ca6637b30,
    0xe97accc1e0297c44,
    0x62178713511b3370,
    0x7d1f55b642ff770c,
    0x9e9dc6c19f83eb1d,
    0x8bd6193ab387a057,
    0x71f824c04883639a,
    0x50e02e2a57688b3e,
    0x332bcf606f171421,
    0xdec205cb7f4f41d9,
    0x48d7e71ceb82870d,
    0x1ceb3f29ca4f06b7,
];
const KEYS_EP: [u64; 8] = [
    0x75f916e0c3df8735,
    0x3edaa3f553f13748,
    0x1b9828492bc45ad4,
    0x7e1ebeb8306202b8,
    0xa0fc718f7f0940ee,
    0x1965728d9f6c9649,
    0xf86692369e7b52c0,
    0x0c52bc8732b4538f,
];
const KEYS_CASTLING: [u64; 4] = [
    0xa56301c82aec6a30,
    0x5043144d9086148c,
    0xc49643076e5a96e7,
    0xbe2bd03d39191f65,
];
const KEYS_TURN: u64 = 0x2bddd8fec7473bd5;

#[must_use]
const fn get_index(colour: Colour, piece: Piece, sq: Square) -> usize {
    colour as usize * 6 * 64 + piece as usize * 64 + sq.0 as usize
}

impl Position {
    #[must_use]
    pub fn predict_hash(&self, mv: &Mv) -> u64 {
        let mut hash = self.hash;

        let piece = self.get_piece_on(mv.from);
        let captured = self.get_piece_on(mv.to);
        let from = mv.from.maybe_flip(self.turn == Colour::Black);
        let to = mv.to.maybe_flip(self.turn == Colour::Black);

        // Remove the piece
        hash ^= KEYS[get_index(self.turn, piece.unwrap(), from)];

        // Add the piece
        hash ^= KEYS[get_index(self.turn, piece.unwrap(), to)];

        // Capture
        if self.get_them().is_set(mv.to) {
            hash ^= KEYS[get_index(!self.turn, captured.unwrap(), to)];
        }

        // En passant - unset
        if let Some(ep) = self.ep {
            hash ^= KEYS_EP[ep.file() as usize];
        }

        // En passant - remove pawn
        if piece.unwrap() == Piece::Pawn && mv.from.file() != mv.to.file() && captured.is_none() {
            let sq = Square(mv.to.0 - 8).maybe_flip(self.turn == Colour::Black);
            hash ^= KEYS[get_index(!self.turn, Piece::Pawn, sq)];
        }

        // En passant - set
        if piece.unwrap() == Piece::Pawn && mv.to.0 - mv.from.0 == 16 {
            hash ^= KEYS_EP[mv.to.file() as usize];
        }

        let ksc_sq = Square::from_coords(self.castle_files[0], 0);
        let qsc_sq = Square::from_coords(self.castle_files[1], 0);

        // King side castle
        if piece.unwrap() == Piece::King && self.us_ksc && mv.to == ksc_sq {
            hash ^= KEYS[get_index(self.turn, Piece::King, from)];
            hash ^= KEYS[get_index(self.turn, Piece::King, to)];

            // King
            hash ^= KEYS[get_index(self.turn, Piece::King, from)];
            hash ^= KEYS[get_index(
                self.turn,
                Piece::King,
                G1.maybe_flip(self.turn == Colour::Black),
            )];

            // Rook
            hash ^= KEYS[get_index(
                self.turn,
                Piece::Rook,
                ksc_sq.maybe_flip(self.turn == Colour::Black),
            )];
            hash ^= KEYS[get_index(
                self.turn,
                Piece::Rook,
                F1.maybe_flip(self.turn == Colour::Black),
            )];
        }
        // Queen side castle
        else if piece.unwrap() == Piece::King && self.us_qsc && mv.to == qsc_sq {
            hash ^= KEYS[get_index(self.turn, Piece::King, from)];
            hash ^= KEYS[get_index(self.turn, Piece::King, to)];

            // King
            hash ^= KEYS[get_index(self.turn, Piece::King, from)];
            hash ^= KEYS[get_index(
                self.turn,
                Piece::King,
                C1.maybe_flip(self.turn == Colour::Black),
            )];

            // Rook
            hash ^= KEYS[get_index(
                self.turn,
                Piece::Rook,
                qsc_sq.maybe_flip(self.turn == Colour::Black),
            )];
            hash ^= KEYS[get_index(
                self.turn,
                Piece::Rook,
                D1.maybe_flip(self.turn == Colour::Black),
            )];
        }

        // Promo
        if mv.promo != Piece::None {
            hash ^= KEYS[get_index(self.turn, Piece::Pawn, to)];
            hash ^= KEYS[get_index(self.turn, mv.promo, to)];
        }

        // Castling - us
        if self.us_ksc && mv.from == ksc_sq {
            hash ^= KEYS_CASTLING[2 * self.turn as usize];
        }
        if self.us_qsc && mv.from == qsc_sq {
            hash ^= KEYS_CASTLING[2 * self.turn as usize + 1];
        }
        if self.us_ksc && piece.unwrap() == Piece::King {
            hash ^= KEYS_CASTLING[2 * self.turn as usize];
        }
        if self.us_qsc && piece.unwrap() == Piece::King {
            hash ^= KEYS_CASTLING[2 * self.turn as usize + 1];
        }

        // Castling - them
        if self.them_ksc && mv.to == Square::from_coords(self.castle_files[2], 7) {
            hash ^= KEYS_CASTLING[2 * !self.turn as usize];
        }
        if self.them_qsc && mv.to == Square::from_coords(self.castle_files[3], 7) {
            hash ^= KEYS_CASTLING[2 * !self.turn as usize + 1];
        }

        // Turn key
        hash ^= KEYS_TURN;

        hash
    }

    #[must_use]
    fn white_pov(bb: Bitboard, pov: Colour) -> Bitboard {
        if pov == Colour::White {
            bb
        } else {
            bb.flip()
        }
    }

    #[must_use]
    pub fn calculate_hash(&self) -> u64 {
        let mut hash = 0u64;

        // Pieces - white
        for sq in Self::white_pov(self.get_white() & self.get_pawns(), self.turn) {
            hash ^= KEYS[get_index(Colour::White, Piece::Pawn, sq)];
        }
        for sq in Self::white_pov(self.get_white() & self.get_knights(), self.turn) {
            hash ^= KEYS[get_index(Colour::White, Piece::Knight, sq)];
        }
        for sq in Self::white_pov(self.get_white() & self.get_bishops(), self.turn) {
            hash ^= KEYS[get_index(Colour::White, Piece::Bishop, sq)];
        }
        for sq in Self::white_pov(self.get_white() & self.get_rooks(), self.turn) {
            hash ^= KEYS[get_index(Colour::White, Piece::Rook, sq)];
        }
        for sq in Self::white_pov(self.get_white() & self.get_queens(), self.turn) {
            hash ^= KEYS[get_index(Colour::White, Piece::Queen, sq)];
        }
        for sq in Self::white_pov(self.get_white() & self.get_kings(), self.turn) {
            hash ^= KEYS[get_index(Colour::White, Piece::King, sq)];
        }

        // Pieces - black
        for sq in Self::white_pov(self.get_black() & self.get_pawns(), self.turn) {
            hash ^= KEYS[get_index(Colour::Black, Piece::Pawn, sq)];
        }
        for sq in Self::white_pov(self.get_black() & self.get_knights(), self.turn) {
            hash ^= KEYS[get_index(Colour::Black, Piece::Knight, sq)];
        }
        for sq in Self::white_pov(self.get_black() & self.get_bishops(), self.turn) {
            hash ^= KEYS[get_index(Colour::Black, Piece::Bishop, sq)];
        }
        for sq in Self::white_pov(self.get_black() & self.get_rooks(), self.turn) {
            hash ^= KEYS[get_index(Colour::Black, Piece::Rook, sq)];
        }
        for sq in Self::white_pov(self.get_black() & self.get_queens(), self.turn) {
            hash ^= KEYS[get_index(Colour::Black, Piece::Queen, sq)];
        }
        for sq in Self::white_pov(self.get_black() & self.get_kings(), self.turn) {
            hash ^= KEYS[get_index(Colour::Black, Piece::King, sq)];
        }

        // En passant
        if let Some(sq) = self.ep {
            hash ^= KEYS_EP[sq.file() as usize];
        }

        // Castling
        if self.us_ksc {
            hash ^= KEYS_CASTLING[2 * self.turn as usize];
        }
        if self.us_qsc {
            hash ^= KEYS_CASTLING[2 * self.turn as usize + 1];
        }
        if self.them_ksc {
            hash ^= KEYS_CASTLING[2 * !self.turn as usize];
        }
        if self.them_qsc {
            hash ^= KEYS_CASTLING[2 * !self.turn as usize + 1];
        }

        // Turn key
        if self.turn == Colour::Black {
            hash ^= KEYS_TURN;
        }

        hash
    }
}
