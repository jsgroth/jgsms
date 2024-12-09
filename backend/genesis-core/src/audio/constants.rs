#![allow(clippy::excessive_precision)]

pub const YM2612_HPF_CHARGE_FACTOR: f64 = 0.9966982656608827;

pub const YM2612_LPF_TAPS: usize = 59;

// (2 + 1) * ~53267 Hz = ~159801 Hz
pub const YM2612_ZERO_PADDING: u32 = 2;

// Generated with a cutoff frequency of 16000 Hz (in practice closer to 15000 Hz)
pub const YM2612_SHARP_LPF_COEFFICIENTS: [f64; YM2612_LPF_TAPS] = [
    -0.0005618134777050833,
    -0.0009106280024131886,
    -0.0009951665581006586,
    -0.0006864844596839729,
    8.91095538107836e-05,
    0.001221568452725576,
    0.002324745659973819,
    0.002787295650336696,
    0.002014563783717795,
    -0.0002051972831801398,
    -0.003387373813278261,
    -0.006303762769407587,
    -0.007334947604817382,
    -0.005173973786939684,
    0.0003752815195973398,
    0.007890668566391094,
    0.01446412001895162,
    0.01660131766255972,
    0.01168031407809752,
    -0.000550699136283774,
    -0.01709212045431253,
    -0.03195066049941472,
    -0.03767239416612238,
    -0.02778025195129672,
    0.0006811686365784664,
    0.04544572967355225,
    0.09891321331447055,
    0.1498426236007754,
    0.1864169875221624,
    0.1997135325385104,
    0.1864169875221624,
    0.1498426236007754,
    0.09891321331447055,
    0.04544572967355225,
    0.0006811686365784665,
    -0.02778025195129673,
    -0.03767239416612239,
    -0.03195066049941472,
    -0.01709212045431253,
    -0.0005506991362837743,
    0.01168031407809753,
    0.01660131766255972,
    0.01446412001895162,
    0.007890668566391095,
    0.0003752815195973401,
    -0.005173973786939685,
    -0.007334947604817386,
    -0.006303762769407591,
    -0.00338737381327826,
    -0.0002051972831801399,
    0.002014563783717794,
    0.002787295650336697,
    0.002324745659973819,
    0.001221568452725576,
    8.910955381078367e-05,
    -0.000686484459683973,
    -0.0009951665581006586,
    -0.0009106280024131891,
    -0.0005618134777050833,
];

pub const YM2612_MID_LPF_COEFFICIENTS: [f64; YM2612_LPF_TAPS] = [
    1.140695205115673e-06,
    -0.0003944887749648573,
    -0.0008163338271704408,
    -0.001229990275292423,
    -0.001542982687900274,
    -0.00160572761455264,
    -0.001244968859522525,
    -0.0003260367161178234,
    0.001171508468484629,
    0.003088952672021884,
    0.005060663660700997,
    0.006544270861881121,
    0.006916450055026706,
    0.005624134562857911,
    0.002362856628713952,
    -0.00275943777274042,
    -0.009119048620758614,
    -0.01557247541396984,
    -0.02057235273806586,
    -0.02239824023460479,
    -0.01946924276065617,
    -0.01068209817702342,
    0.004294331626947669,
    0.02483533137809254,
    0.04933728948079393,
    0.0753755418735014,
    0.1000270509024372,
    0.1203068707807085,
    0.1336414562733408,
    0.1382911491052512,
    0.1336414562733408,
    0.1203068707807085,
    0.1000270509024372,
    0.0753755418735014,
    0.04933728948079392,
    0.02483533137809255,
    0.00429433162694767,
    -0.01068209817702342,
    -0.01946924276065617,
    -0.02239824023460479,
    -0.02057235273806586,
    -0.01557247541396985,
    -0.009119048620758614,
    -0.002759437772740421,
    0.002362856628713955,
    0.005624134562857912,
    0.006916450055026712,
    0.006544270861881126,
    0.005060663660700996,
    0.003088952672021885,
    0.001171508468484628,
    -0.0003260367161178234,
    -0.001244968859522524,
    -0.00160572761455264,
    -0.001542982687900274,
    -0.001229990275292423,
    -0.0008163338271704408,
    -0.0003944887749648576,
    1.140695205115687e-06,
];

// Generated with a cutoff frequency of 9200 Hz (in practice closer to 8000 Hz)
pub const YM2612_SOFT_LPF_COEFFICIENTS: [f64; YM2612_LPF_TAPS] = [
    -0.0007278021720442232,
    -0.0005447666205378564,
    -0.000274302576624536,
    0.000126557429888348,
    0.0006900296863461439,
    0.001410249200686746,
    0.002219988616654026,
    0.002979354938473057,
    0.003482655733053416,
    0.003486205837542198,
    0.002755332648172326,
    0.001124105832090061,
    -0.001442642665908662,
    -0.004797716757100719,
    -0.008583026063251255,
    -0.01223417147892489,
    -0.01502329115226756,
    -0.01613885784104896,
    -0.01479420193554657,
    -0.01035041387538206,
    -0.002435151175753401,
    0.00896235761605722,
    0.02343772999213154,
    0.04017162511085463,
    0.05798837352472097,
    0.0754687507918865,
    0.09110388312979517,
    0.1034702600409117,
    0.1114017610577151,
    0.1141342462548234,
    0.1114017610577151,
    0.1034702600409117,
    0.0911038831297952,
    0.0754687507918865,
    0.05798837352472096,
    0.04017162511085465,
    0.02343772999213155,
    0.00896235761605722,
    -0.002435151175753401,
    -0.01035041387538206,
    -0.01479420193554657,
    -0.01613885784104896,
    -0.01502329115226756,
    -0.0122341714789249,
    -0.008583026063251262,
    -0.004797716757100719,
    -0.001442642665908664,
    0.001124105832090062,
    0.002755332648172325,
    0.003486205837542198,
    0.003482655733053415,
    0.002979354938473057,
    0.002219988616654025,
    0.001410249200686746,
    0.0006900296863461442,
    0.000126557429888348,
    -0.000274302576624536,
    -0.0005447666205378566,
    -0.0007278021720442232,
];

// Generated with a cutoff frequency of 6200 Hz (in practice closer to 5000 Hz)
pub const YM2612_VSOFT_LPF_COEFFICIENTS: [f64; YM2612_LPF_TAPS] = [
    0.0005610208144449837,
    0.0004124870242831631,
    0.0002281813864530707,
    -3.132057428989354e-05,
    -0.0004072364260000324,
    -0.0009333961589622227,
    -0.001626524968714066,
    -0.002477215945402887,
    -0.003442838505235802,
    -0.00444350493698885,
    -0.005361957931957869,
    -0.006047868564963306,
    -0.00632658245316093,
    -0.006011867253483951,
    -0.00492174811277304,
    -0.002896120000537329,
    0.0001854575910112248,
    0.004387512407125696,
    0.009705651256630489,
    0.01605827740512947,
    0.02328415255401307,
    0.03114642885130217,
    0.03934320092534987,
    0.0475240253439335,
    0.05531128401452737,
    0.06232478580871179,
    0.06820765632196518,
    0.07265139549460209,
    0.07541800639703727,
    0.07635731647189946,
    0.07541800639703729,
    0.07265139549460209,
    0.0682076563219652,
    0.06232478580871181,
    0.05531128401452736,
    0.04752402534393351,
    0.03934320092534989,
    0.03114642885130217,
    0.02328415255401307,
    0.01605827740512948,
    0.009705651256630487,
    0.004387512407125698,
    0.000185457591011225,
    -0.00289612000053733,
    -0.004921748112773044,
    -0.006011867253483952,
    -0.006326582453160935,
    -0.006047868564963309,
    -0.005361957931957868,
    -0.004443504936988852,
    -0.0034428385052358,
    -0.002477215945402887,
    -0.001626524968714066,
    -0.0009333961589622226,
    -0.0004072364260000325,
    -3.132057428989355e-05,
    0.0002281813864530707,
    0.0004124870242831633,
    0.0005610208144449837,
];
