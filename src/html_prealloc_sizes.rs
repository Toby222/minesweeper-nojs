// If it's stupid and it works, it isn't stupid
// As long as I don't check if this actually works, it therefore cannot be stupid

pub(crate) struct Sizes {
    pub(crate) style: usize,
    pub(crate) table: usize,
    pub(crate) inputs: usize,
}

pub(crate) const fn get_sizes(width: usize, height: usize) -> Sizes {
    match (width, height) {
        (1, 2) => Sizes {
            style: 282,
            inputs: 100,
            table: 81,
        },
        (2, 1) => Sizes {
            style: 282,
            inputs: 100,
            table: 72,
        },
        (2, 2) => Sizes {
            style: 564,
            table: 153,
            inputs: 200,
        },
        (2, 3) => Sizes {
            style: 846,
            table: 234,
            inputs: 300,
        },
        (2, 4) => Sizes {
            style: 1128,
            table: 315,
            inputs: 400,
        },
        (2, 5) => Sizes {
            style: 1410,
            table: 396,
            inputs: 500,
        },
        (2, 6) => Sizes {
            style: 1700,
            table: 479,
            inputs: 602,
        },
        (2, 7) => Sizes {
            style: 1990,
            table: 562,
            inputs: 704,
        },
        (2, 8) => Sizes {
            style: 2280,
            table: 645,
            inputs: 806,
        },
        (2, 9) => Sizes {
            style: 2570,
            table: 728,
            inputs: 908,
        },
        (2, 10) => Sizes {
            style: 2860,
            table: 811,
            inputs: 1010,
        },
        (2, 11) => Sizes {
            style: 3150,
            table: 894,
            inputs: 1112,
        },
        (2, 12) => Sizes {
            style: 3440,
            table: 977,
            inputs: 1214,
        },
        (2, 13) => Sizes {
            style: 3730,
            table: 1060,
            inputs: 1316,
        },
        (2, 14) => Sizes {
            style: 4020,
            table: 1143,
            inputs: 1418,
        },
        (2, 15) => Sizes {
            style: 4310,
            table: 1226,
            inputs: 1520,
        },
        (2, 16) => Sizes {
            style: 4600,
            table: 1309,
            inputs: 1622,
        },
        (2, 17) => Sizes {
            style: 4890,
            table: 1392,
            inputs: 1724,
        },
        (2, 18) => Sizes {
            style: 5180,
            table: 1475,
            inputs: 1826,
        },
        (2, 19) => Sizes {
            style: 5470,
            table: 1558,
            inputs: 1928,
        },
        (2, 20) => Sizes {
            style: 5760,
            table: 1641,
            inputs: 2030,
        },
        (2, 21) => Sizes {
            style: 6050,
            table: 1724,
            inputs: 2132,
        },
        (2, 22) => Sizes {
            style: 6340,
            table: 1807,
            inputs: 2234,
        },
        (2, 23) => Sizes {
            style: 6630,
            table: 1890,
            inputs: 2336,
        },
        (2, 24) => Sizes {
            style: 6920,
            table: 1973,
            inputs: 2438,
        },
        (2, 25) => Sizes {
            style: 7210,
            table: 2056,
            inputs: 2540,
        },
        (2, 26) => Sizes {
            style: 7500,
            table: 2139,
            inputs: 2642,
        },
        (2, 27) => Sizes {
            style: 7790,
            table: 2222,
            inputs: 2744,
        },
        (2, 28) => Sizes {
            style: 8080,
            table: 2305,
            inputs: 2846,
        },
        (2, 29) => Sizes {
            style: 8370,
            table: 2388,
            inputs: 2948,
        },
        (2, 30) => Sizes {
            style: 8660,
            table: 2471,
            inputs: 3050,
        },
        (2, 31) => Sizes {
            style: 8950,
            table: 2554,
            inputs: 3152,
        },
        (2, 32) => Sizes {
            style: 9240,
            table: 2637,
            inputs: 3254,
        },
        (2, 33) => Sizes {
            style: 9530,
            table: 2720,
            inputs: 3356,
        },
        (2, 34) => Sizes {
            style: 9820,
            table: 2803,
            inputs: 3458,
        },
        (2, 35) => Sizes {
            style: 10110,
            table: 2886,
            inputs: 3560,
        },
        (2, 36) => Sizes {
            style: 10400,
            table: 2969,
            inputs: 3662,
        },
        (2, 37) => Sizes {
            style: 10690,
            table: 3052,
            inputs: 3764,
        },
        (2, 38) => Sizes {
            style: 10980,
            table: 3135,
            inputs: 3866,
        },
        (2, 39) => Sizes {
            style: 11270,
            table: 3218,
            inputs: 3968,
        },
        (2, 40) => Sizes {
            style: 11560,
            table: 3301,
            inputs: 4070,
        },
        (2, 41) => Sizes {
            style: 11850,
            table: 3384,
            inputs: 4172,
        },
        (2, 42) => Sizes {
            style: 12140,
            table: 3467,
            inputs: 4274,
        },
        (2, 43) => Sizes {
            style: 12430,
            table: 3550,
            inputs: 4376,
        },
        (2, 44) => Sizes {
            style: 12720,
            table: 3633,
            inputs: 4478,
        },
        (2, 45) => Sizes {
            style: 13010,
            table: 3716,
            inputs: 4580,
        },
        (2, 46) => Sizes {
            style: 13300,
            table: 3799,
            inputs: 4682,
        },
        (2, 47) => Sizes {
            style: 13590,
            table: 3882,
            inputs: 4784,
        },
        (2, 48) => Sizes {
            style: 13880,
            table: 3965,
            inputs: 4886,
        },
        (2, 49) => Sizes {
            style: 14170,
            table: 4048,
            inputs: 4988,
        },
        (2, 50) => Sizes {
            style: 14460,
            table: 4131,
            inputs: 5090,
        },
        (3, 2) => Sizes {
            style: 846,
            table: 225,
            inputs: 300,
        },
        (3, 3) => Sizes {
            style: 1269,
            table: 342,
            inputs: 450,
        },
        (3, 4) => Sizes {
            style: 1700,
            table: 461,
            inputs: 602,
        },
        (3, 5) => Sizes {
            style: 2135,
            table: 581,
            inputs: 755,
        },
        (3, 6) => Sizes {
            style: 2570,
            table: 701,
            inputs: 908,
        },
        (3, 7) => Sizes {
            style: 3005,
            table: 821,
            inputs: 1061,
        },
        (3, 8) => Sizes {
            style: 3440,
            table: 941,
            inputs: 1214,
        },
        (3, 9) => Sizes {
            style: 3875,
            table: 1061,
            inputs: 1367,
        },
        (3, 10) => Sizes {
            style: 4310,
            table: 1181,
            inputs: 1520,
        },
        (3, 11) => Sizes {
            style: 4745,
            table: 1301,
            inputs: 1673,
        },
        (3, 12) => Sizes {
            style: 5180,
            table: 1421,
            inputs: 1826,
        },
        (3, 13) => Sizes {
            style: 5615,
            table: 1541,
            inputs: 1979,
        },
        (3, 14) => Sizes {
            style: 6050,
            table: 1661,
            inputs: 2132,
        },
        (3, 15) => Sizes {
            style: 6485,
            table: 1781,
            inputs: 2285,
        },
        (3, 16) => Sizes {
            style: 6920,
            table: 1901,
            inputs: 2438,
        },
        (3, 17) => Sizes {
            style: 7355,
            table: 2021,
            inputs: 2591,
        },
        (3, 18) => Sizes {
            style: 7790,
            table: 2141,
            inputs: 2744,
        },
        (3, 19) => Sizes {
            style: 8225,
            table: 2261,
            inputs: 2897,
        },
        (3, 20) => Sizes {
            style: 8660,
            table: 2381,
            inputs: 3050,
        },
        (3, 21) => Sizes {
            style: 9095,
            table: 2501,
            inputs: 3203,
        },
        (3, 22) => Sizes {
            style: 9530,
            table: 2621,
            inputs: 3356,
        },
        (3, 23) => Sizes {
            style: 9965,
            table: 2741,
            inputs: 3509,
        },
        (3, 24) => Sizes {
            style: 10400,
            table: 2861,
            inputs: 3662,
        },
        (3, 25) => Sizes {
            style: 10835,
            table: 2981,
            inputs: 3815,
        },
        (3, 26) => Sizes {
            style: 11270,
            table: 3101,
            inputs: 3968,
        },
        (3, 27) => Sizes {
            style: 11705,
            table: 3221,
            inputs: 4121,
        },
        (3, 28) => Sizes {
            style: 12140,
            table: 3341,
            inputs: 4274,
        },
        (3, 29) => Sizes {
            style: 12575,
            table: 3461,
            inputs: 4427,
        },
        (3, 30) => Sizes {
            style: 13010,
            table: 3581,
            inputs: 4580,
        },
        (3, 31) => Sizes {
            style: 13445,
            table: 3701,
            inputs: 4733,
        },
        (3, 32) => Sizes {
            style: 13880,
            table: 3821,
            inputs: 4886,
        },
        (3, 33) => Sizes {
            style: 14315,
            table: 3941,
            inputs: 5039,
        },
        (3, 34) => Sizes {
            style: 14758,
            table: 4063,
            inputs: 5194,
        },
        (3, 35) => Sizes {
            style: 15205,
            table: 4186,
            inputs: 5350,
        },
        (3, 36) => Sizes {
            style: 15652,
            table: 4309,
            inputs: 5506,
        },
        (3, 37) => Sizes {
            style: 16099,
            table: 4432,
            inputs: 5662,
        },
        (3, 38) => Sizes {
            style: 16546,
            table: 4555,
            inputs: 5818,
        },
        (3, 39) => Sizes {
            style: 16993,
            table: 4678,
            inputs: 5974,
        },
        (3, 40) => Sizes {
            style: 17440,
            table: 4801,
            inputs: 6130,
        },
        (3, 41) => Sizes {
            style: 17887,
            table: 4924,
            inputs: 6286,
        },
        (3, 42) => Sizes {
            style: 18334,
            table: 5047,
            inputs: 6442,
        },
        (3, 43) => Sizes {
            style: 18781,
            table: 5170,
            inputs: 6598,
        },
        (3, 44) => Sizes {
            style: 19228,
            table: 5293,
            inputs: 6754,
        },
        (3, 45) => Sizes {
            style: 19675,
            table: 5416,
            inputs: 6910,
        },
        (3, 46) => Sizes {
            style: 20122,
            table: 5539,
            inputs: 7066,
        },
        (3, 47) => Sizes {
            style: 20569,
            table: 5662,
            inputs: 7222,
        },
        (3, 48) => Sizes {
            style: 21016,
            table: 5785,
            inputs: 7378,
        },
        (3, 49) => Sizes {
            style: 21463,
            table: 5908,
            inputs: 7534,
        },
        (3, 50) => Sizes {
            style: 21910,
            table: 6031,
            inputs: 7690,
        },
        (4, 2) => Sizes {
            style: 1128,
            table: 297,
            inputs: 400,
        },
        (4, 3) => Sizes {
            style: 1700,
            table: 452,
            inputs: 602,
        },
        (4, 4) => Sizes {
            style: 2280,
            table: 609,
            inputs: 806,
        },
        (4, 5) => Sizes {
            style: 2860,
            table: 766,
            inputs: 1010,
        },
        (4, 6) => Sizes {
            style: 3440,
            table: 923,
            inputs: 1214,
        },
        (4, 7) => Sizes {
            style: 4020,
            table: 1080,
            inputs: 1418,
        },
        (4, 8) => Sizes {
            style: 4600,
            table: 1237,
            inputs: 1622,
        },
        (4, 9) => Sizes {
            style: 5180,
            table: 1394,
            inputs: 1826,
        },
        (4, 10) => Sizes {
            style: 5760,
            table: 1551,
            inputs: 2030,
        },
        (4, 11) => Sizes {
            style: 6340,
            table: 1708,
            inputs: 2234,
        },
        (4, 12) => Sizes {
            style: 6920,
            table: 1865,
            inputs: 2438,
        },
        (4, 13) => Sizes {
            style: 7500,
            table: 2022,
            inputs: 2642,
        },
        (4, 14) => Sizes {
            style: 8080,
            table: 2179,
            inputs: 2846,
        },
        (4, 15) => Sizes {
            style: 8660,
            table: 2336,
            inputs: 3050,
        },
        (4, 16) => Sizes {
            style: 9240,
            table: 2493,
            inputs: 3254,
        },
        (4, 17) => Sizes {
            style: 9820,
            table: 2650,
            inputs: 3458,
        },
        (4, 18) => Sizes {
            style: 10400,
            table: 2807,
            inputs: 3662,
        },
        (4, 19) => Sizes {
            style: 10980,
            table: 2964,
            inputs: 3866,
        },
        (4, 20) => Sizes {
            style: 11560,
            table: 3121,
            inputs: 4070,
        },
        (4, 21) => Sizes {
            style: 12140,
            table: 3278,
            inputs: 4274,
        },
        (4, 22) => Sizes {
            style: 12720,
            table: 3435,
            inputs: 4478,
        },
        (4, 23) => Sizes {
            style: 13300,
            table: 3592,
            inputs: 4682,
        },
        (4, 24) => Sizes {
            style: 13880,
            table: 3749,
            inputs: 4886,
        },
        (4, 25) => Sizes {
            style: 14460,
            table: 3906,
            inputs: 5090,
        },
        (4, 26) => Sizes {
            style: 15056,
            table: 4067,
            inputs: 5298,
        },
        (4, 27) => Sizes {
            style: 15652,
            table: 4228,
            inputs: 5506,
        },
        (4, 28) => Sizes {
            style: 16248,
            table: 4389,
            inputs: 5714,
        },
        (4, 29) => Sizes {
            style: 16844,
            table: 4550,
            inputs: 5922,
        },
        (4, 30) => Sizes {
            style: 17440,
            table: 4711,
            inputs: 6130,
        },
        (4, 31) => Sizes {
            style: 18036,
            table: 4872,
            inputs: 6338,
        },
        (4, 32) => Sizes {
            style: 18632,
            table: 5033,
            inputs: 6546,
        },
        (4, 33) => Sizes {
            style: 19228,
            table: 5194,
            inputs: 6754,
        },
        (4, 34) => Sizes {
            style: 19824,
            table: 5355,
            inputs: 6962,
        },
        (4, 35) => Sizes {
            style: 20420,
            table: 5516,
            inputs: 7170,
        },
        (4, 36) => Sizes {
            style: 21016,
            table: 5677,
            inputs: 7378,
        },
        (4, 37) => Sizes {
            style: 21612,
            table: 5838,
            inputs: 7586,
        },
        (4, 38) => Sizes {
            style: 22208,
            table: 5999,
            inputs: 7794,
        },
        (4, 39) => Sizes {
            style: 22804,
            table: 6160,
            inputs: 8002,
        },
        (4, 40) => Sizes {
            style: 23400,
            table: 6321,
            inputs: 8210,
        },
        (4, 41) => Sizes {
            style: 23996,
            table: 6482,
            inputs: 8418,
        },
        (4, 42) => Sizes {
            style: 24592,
            table: 6643,
            inputs: 8626,
        },
        (4, 43) => Sizes {
            style: 25188,
            table: 6804,
            inputs: 8834,
        },
        (4, 44) => Sizes {
            style: 25784,
            table: 6965,
            inputs: 9042,
        },
        (4, 45) => Sizes {
            style: 26380,
            table: 7126,
            inputs: 9250,
        },
        (4, 46) => Sizes {
            style: 26976,
            table: 7287,
            inputs: 9458,
        },
        (4, 47) => Sizes {
            style: 27572,
            table: 7448,
            inputs: 9666,
        },
        (4, 48) => Sizes {
            style: 28168,
            table: 7609,
            inputs: 9874,
        },
        (4, 49) => Sizes {
            style: 28764,
            table: 7770,
            inputs: 10082,
        },
        (4, 50) => Sizes {
            style: 29360,
            table: 7931,
            inputs: 10290,
        },
        (5, 2) => Sizes {
            style: 1410,
            table: 369,
            inputs: 500,
        },
        (5, 3) => Sizes {
            style: 2135,
            table: 563,
            inputs: 755,
        },
        (5, 4) => Sizes {
            style: 2860,
            table: 757,
            inputs: 1010,
        },
        (5, 5) => Sizes {
            style: 3585,
            table: 951,
            inputs: 1265,
        },
        (5, 6) => Sizes {
            style: 4310,
            table: 1145,
            inputs: 1520,
        },
        (5, 7) => Sizes {
            style: 5035,
            table: 1339,
            inputs: 1775,
        },
        (5, 8) => Sizes {
            style: 5760,
            table: 1533,
            inputs: 2030,
        },
        (5, 9) => Sizes {
            style: 6485,
            table: 1727,
            inputs: 2285,
        },
        (5, 10) => Sizes {
            style: 7210,
            table: 1921,
            inputs: 2540,
        },
        (5, 11) => Sizes {
            style: 7935,
            table: 2115,
            inputs: 2795,
        },
        (5, 12) => Sizes {
            style: 8660,
            table: 2309,
            inputs: 3050,
        },
        (5, 13) => Sizes {
            style: 9385,
            table: 2503,
            inputs: 3305,
        },
        (5, 14) => Sizes {
            style: 10110,
            table: 2697,
            inputs: 3560,
        },
        (5, 15) => Sizes {
            style: 10835,
            table: 2891,
            inputs: 3815,
        },
        (5, 16) => Sizes {
            style: 11560,
            table: 3085,
            inputs: 4070,
        },
        (5, 17) => Sizes {
            style: 12285,
            table: 3279,
            inputs: 4325,
        },
        (5, 18) => Sizes {
            style: 13010,
            table: 3473,
            inputs: 4580,
        },
        (5, 19) => Sizes {
            style: 13735,
            table: 3667,
            inputs: 4835,
        },
        (5, 20) => Sizes {
            style: 14460,
            table: 3861,
            inputs: 5090,
        },
        (5, 21) => Sizes {
            style: 15205,
            table: 4060,
            inputs: 5350,
        },
        (5, 22) => Sizes {
            style: 15950,
            table: 4259,
            inputs: 5610,
        },
        (5, 23) => Sizes {
            style: 16695,
            table: 4458,
            inputs: 5870,
        },
        (5, 24) => Sizes {
            style: 17440,
            table: 4657,
            inputs: 6130,
        },
        (5, 25) => Sizes {
            style: 18185,
            table: 4856,
            inputs: 6390,
        },
        (5, 26) => Sizes {
            style: 18930,
            table: 5055,
            inputs: 6650,
        },
        (5, 27) => Sizes {
            style: 19675,
            table: 5254,
            inputs: 6910,
        },
        (5, 28) => Sizes {
            style: 20420,
            table: 5453,
            inputs: 7170,
        },
        (5, 29) => Sizes {
            style: 21165,
            table: 5652,
            inputs: 7430,
        },
        (5, 30) => Sizes {
            style: 21910,
            table: 5851,
            inputs: 7690,
        },
        (5, 31) => Sizes {
            style: 22655,
            table: 6050,
            inputs: 7950,
        },
        (5, 32) => Sizes {
            style: 23400,
            table: 6249,
            inputs: 8210,
        },
        (5, 33) => Sizes {
            style: 24145,
            table: 6448,
            inputs: 8470,
        },
        (5, 34) => Sizes {
            style: 24890,
            table: 6647,
            inputs: 8730,
        },
        (5, 35) => Sizes {
            style: 25635,
            table: 6846,
            inputs: 8990,
        },
        (5, 36) => Sizes {
            style: 26380,
            table: 7045,
            inputs: 9250,
        },
        (5, 37) => Sizes {
            style: 27125,
            table: 7244,
            inputs: 9510,
        },
        (5, 38) => Sizes {
            style: 27870,
            table: 7443,
            inputs: 9770,
        },
        (5, 39) => Sizes {
            style: 28615,
            table: 7642,
            inputs: 10030,
        },
        (5, 40) => Sizes {
            style: 29360,
            table: 7841,
            inputs: 10290,
        },
        (5, 41) => Sizes {
            style: 30105,
            table: 8040,
            inputs: 10550,
        },
        (5, 42) => Sizes {
            style: 30850,
            table: 8239,
            inputs: 10810,
        },
        (5, 43) => Sizes {
            style: 31595,
            table: 8438,
            inputs: 11070,
        },
        (5, 44) => Sizes {
            style: 32340,
            table: 8637,
            inputs: 11330,
        },
        (5, 45) => Sizes {
            style: 33085,
            table: 8836,
            inputs: 11590,
        },
        (5, 46) => Sizes {
            style: 33830,
            table: 9035,
            inputs: 11850,
        },
        (5, 47) => Sizes {
            style: 34575,
            table: 9234,
            inputs: 12110,
        },
        (5, 48) => Sizes {
            style: 35320,
            table: 9433,
            inputs: 12370,
        },
        (5, 49) => Sizes {
            style: 36065,
            table: 9632,
            inputs: 12630,
        },
        (5, 50) => Sizes {
            style: 36810,
            table: 9831,
            inputs: 12890,
        },
        (6, 2) => Sizes {
            style: 1700,
            table: 443,
            inputs: 602,
        },
        (6, 3) => Sizes {
            style: 2570,
            table: 674,
            inputs: 908,
        },
        (6, 4) => Sizes {
            style: 3440,
            table: 905,
            inputs: 1214,
        },
        (6, 5) => Sizes {
            style: 4310,
            table: 1136,
            inputs: 1520,
        },
        (6, 6) => Sizes {
            style: 5180,
            table: 1367,
            inputs: 1826,
        },
        (6, 7) => Sizes {
            style: 6050,
            table: 1598,
            inputs: 2132,
        },
        (6, 8) => Sizes {
            style: 6920,
            table: 1829,
            inputs: 2438,
        },
        (6, 9) => Sizes {
            style: 7790,
            table: 2060,
            inputs: 2744,
        },
        (6, 10) => Sizes {
            style: 8660,
            table: 2291,
            inputs: 3050,
        },
        (6, 11) => Sizes {
            style: 9530,
            table: 2522,
            inputs: 3356,
        },
        (6, 12) => Sizes {
            style: 10400,
            table: 2753,
            inputs: 3662,
        },
        (6, 13) => Sizes {
            style: 11270,
            table: 2984,
            inputs: 3968,
        },
        (6, 14) => Sizes {
            style: 12140,
            table: 3215,
            inputs: 4274,
        },
        (6, 15) => Sizes {
            style: 13010,
            table: 3446,
            inputs: 4580,
        },
        (6, 16) => Sizes {
            style: 13880,
            table: 3677,
            inputs: 4886,
        },
        (6, 17) => Sizes {
            style: 14758,
            table: 3910,
            inputs: 5194,
        },
        (6, 18) => Sizes {
            style: 15652,
            table: 4147,
            inputs: 5506,
        },
        (6, 19) => Sizes {
            style: 16546,
            table: 4384,
            inputs: 5818,
        },
        (6, 20) => Sizes {
            style: 17440,
            table: 4621,
            inputs: 6130,
        },
        (6, 21) => Sizes {
            style: 18334,
            table: 4858,
            inputs: 6442,
        },
        (6, 22) => Sizes {
            style: 19228,
            table: 5095,
            inputs: 6754,
        },
        (6, 23) => Sizes {
            style: 20122,
            table: 5332,
            inputs: 7066,
        },
        (6, 24) => Sizes {
            style: 21016,
            table: 5569,
            inputs: 7378,
        },
        (6, 25) => Sizes {
            style: 21910,
            table: 5806,
            inputs: 7690,
        },
        (6, 26) => Sizes {
            style: 22804,
            table: 6043,
            inputs: 8002,
        },
        (6, 27) => Sizes {
            style: 23698,
            table: 6280,
            inputs: 8314,
        },
        (6, 28) => Sizes {
            style: 24592,
            table: 6517,
            inputs: 8626,
        },
        (6, 29) => Sizes {
            style: 25486,
            table: 6754,
            inputs: 8938,
        },
        (6, 30) => Sizes {
            style: 26380,
            table: 6991,
            inputs: 9250,
        },
        (6, 31) => Sizes {
            style: 27274,
            table: 7228,
            inputs: 9562,
        },
        (6, 32) => Sizes {
            style: 28168,
            table: 7465,
            inputs: 9874,
        },
        (6, 33) => Sizes {
            style: 29062,
            table: 7702,
            inputs: 10186,
        },
        (6, 34) => Sizes {
            style: 29956,
            table: 7939,
            inputs: 10498,
        },
        (6, 35) => Sizes {
            style: 30850,
            table: 8176,
            inputs: 10810,
        },
        (6, 36) => Sizes {
            style: 31744,
            table: 8413,
            inputs: 11122,
        },
        (6, 37) => Sizes {
            style: 32638,
            table: 8650,
            inputs: 11434,
        },
        (6, 38) => Sizes {
            style: 33532,
            table: 8887,
            inputs: 11746,
        },
        (6, 39) => Sizes {
            style: 34426,
            table: 9124,
            inputs: 12058,
        },
        (6, 40) => Sizes {
            style: 35320,
            table: 9361,
            inputs: 12370,
        },
        (6, 41) => Sizes {
            style: 36214,
            table: 9598,
            inputs: 12682,
        },
        (6, 42) => Sizes {
            style: 37108,
            table: 9835,
            inputs: 12994,
        },
        (6, 43) => Sizes {
            style: 38002,
            table: 10072,
            inputs: 13306,
        },
        (6, 44) => Sizes {
            style: 38896,
            table: 10309,
            inputs: 13618,
        },
        (6, 45) => Sizes {
            style: 39790,
            table: 10546,
            inputs: 13930,
        },
        (6, 46) => Sizes {
            style: 40684,
            table: 10783,
            inputs: 14242,
        },
        (6, 47) => Sizes {
            style: 41578,
            table: 11020,
            inputs: 14554,
        },
        (6, 48) => Sizes {
            style: 42472,
            table: 11257,
            inputs: 14866,
        },
        (6, 49) => Sizes {
            style: 43366,
            table: 11494,
            inputs: 15178,
        },
        (6, 50) => Sizes {
            style: 44260,
            table: 11731,
            inputs: 15490,
        },
        (7, 2) => Sizes {
            style: 1990,
            table: 517,
            inputs: 704,
        },
        (7, 3) => Sizes {
            style: 3005,
            table: 785,
            inputs: 1061,
        },
        (7, 4) => Sizes {
            style: 4020,
            table: 1053,
            inputs: 1418,
        },
        (7, 5) => Sizes {
            style: 5035,
            table: 1321,
            inputs: 1775,
        },
        (7, 6) => Sizes {
            style: 6050,
            table: 1589,
            inputs: 2132,
        },
        (7, 7) => Sizes {
            style: 7065,
            table: 1857,
            inputs: 2489,
        },
        (7, 8) => Sizes {
            style: 8080,
            table: 2125,
            inputs: 2846,
        },
        (7, 9) => Sizes {
            style: 9095,
            table: 2393,
            inputs: 3203,
        },
        (7, 10) => Sizes {
            style: 10110,
            table: 2661,
            inputs: 3560,
        },
        (7, 11) => Sizes {
            style: 11125,
            table: 2929,
            inputs: 3917,
        },
        (7, 12) => Sizes {
            style: 12140,
            table: 3197,
            inputs: 4274,
        },
        (7, 13) => Sizes {
            style: 13155,
            table: 3465,
            inputs: 4631,
        },
        (7, 14) => Sizes {
            style: 14170,
            table: 3733,
            inputs: 4988,
        },
        (7, 15) => Sizes {
            style: 15205,
            table: 4006,
            inputs: 5350,
        },
        (7, 16) => Sizes {
            style: 16248,
            table: 4281,
            inputs: 5714,
        },
        (7, 17) => Sizes {
            style: 17291,
            table: 4556,
            inputs: 6078,
        },
        (7, 18) => Sizes {
            style: 18334,
            table: 4831,
            inputs: 6442,
        },
        (7, 19) => Sizes {
            style: 19377,
            table: 5106,
            inputs: 6806,
        },
        (7, 20) => Sizes {
            style: 20420,
            table: 5381,
            inputs: 7170,
        },
        (7, 21) => Sizes {
            style: 21463,
            table: 5656,
            inputs: 7534,
        },
        (7, 22) => Sizes {
            style: 22506,
            table: 5931,
            inputs: 7898,
        },
        (7, 23) => Sizes {
            style: 23549,
            table: 6206,
            inputs: 8262,
        },
        (7, 24) => Sizes {
            style: 24592,
            table: 6481,
            inputs: 8626,
        },
        (7, 25) => Sizes {
            style: 25635,
            table: 6756,
            inputs: 8990,
        },
        (7, 26) => Sizes {
            style: 26678,
            table: 7031,
            inputs: 9354,
        },
        (7, 27) => Sizes {
            style: 27721,
            table: 7306,
            inputs: 9718,
        },
        (7, 28) => Sizes {
            style: 28764,
            table: 7581,
            inputs: 10082,
        },
        (7, 29) => Sizes {
            style: 29807,
            table: 7856,
            inputs: 10446,
        },
        (7, 30) => Sizes {
            style: 30850,
            table: 8131,
            inputs: 10810,
        },
        (7, 31) => Sizes {
            style: 31893,
            table: 8406,
            inputs: 11174,
        },
        (7, 32) => Sizes {
            style: 32936,
            table: 8681,
            inputs: 11538,
        },
        (7, 33) => Sizes {
            style: 33979,
            table: 8956,
            inputs: 11902,
        },
        (7, 34) => Sizes {
            style: 35022,
            table: 9231,
            inputs: 12266,
        },
        (7, 35) => Sizes {
            style: 36065,
            table: 9506,
            inputs: 12630,
        },
        (7, 36) => Sizes {
            style: 37108,
            table: 9781,
            inputs: 12994,
        },
        (7, 37) => Sizes {
            style: 38151,
            table: 10056,
            inputs: 13358,
        },
        (7, 38) => Sizes {
            style: 39194,
            table: 10331,
            inputs: 13722,
        },
        (7, 39) => Sizes {
            style: 40237,
            table: 10606,
            inputs: 14086,
        },
        (7, 40) => Sizes {
            style: 41280,
            table: 10881,
            inputs: 14450,
        },
        (7, 41) => Sizes {
            style: 42323,
            table: 11156,
            inputs: 14814,
        },
        (7, 42) => Sizes {
            style: 43366,
            table: 11431,
            inputs: 15178,
        },
        (7, 43) => Sizes {
            style: 44409,
            table: 11706,
            inputs: 15542,
        },
        (7, 44) => Sizes {
            style: 45452,
            table: 11981,
            inputs: 15906,
        },
        (7, 45) => Sizes {
            style: 46495,
            table: 12256,
            inputs: 16270,
        },
        (7, 46) => Sizes {
            style: 47538,
            table: 12531,
            inputs: 16634,
        },
        (7, 47) => Sizes {
            style: 48581,
            table: 12806,
            inputs: 16998,
        },
        (7, 48) => Sizes {
            style: 49624,
            table: 13081,
            inputs: 17362,
        },
        (7, 49) => Sizes {
            style: 50667,
            table: 13356,
            inputs: 17726,
        },
        (7, 50) => Sizes {
            style: 51710,
            table: 13631,
            inputs: 18090,
        },
        (8, 2) => Sizes {
            style: 2280,
            table: 591,
            inputs: 806,
        },
        (8, 3) => Sizes {
            style: 3440,
            table: 896,
            inputs: 1214,
        },
        (8, 4) => Sizes {
            style: 4600,
            table: 1201,
            inputs: 1622,
        },
        (8, 5) => Sizes {
            style: 5760,
            table: 1506,
            inputs: 2030,
        },
        (8, 6) => Sizes {
            style: 6920,
            table: 1811,
            inputs: 2438,
        },
        (8, 7) => Sizes {
            style: 8080,
            table: 2116,
            inputs: 2846,
        },
        (8, 8) => Sizes {
            style: 9240,
            table: 2421,
            inputs: 3254,
        },
        (8, 9) => Sizes {
            style: 10400,
            table: 2726,
            inputs: 3662,
        },
        (8, 10) => Sizes {
            style: 11560,
            table: 3031,
            inputs: 4070,
        },
        (8, 11) => Sizes {
            style: 12720,
            table: 3336,
            inputs: 4478,
        },
        (8, 12) => Sizes {
            style: 13880,
            table: 3641,
            inputs: 4886,
        },
        (8, 13) => Sizes {
            style: 15056,
            table: 3950,
            inputs: 5298,
        },
        (8, 14) => Sizes {
            style: 16248,
            table: 4263,
            inputs: 5714,
        },
        (8, 15) => Sizes {
            style: 17440,
            table: 4576,
            inputs: 6130,
        },
        (8, 16) => Sizes {
            style: 18632,
            table: 4889,
            inputs: 6546,
        },
        (8, 17) => Sizes {
            style: 19824,
            table: 5202,
            inputs: 6962,
        },
        (8, 18) => Sizes {
            style: 21016,
            table: 5515,
            inputs: 7378,
        },
        (8, 19) => Sizes {
            style: 22208,
            table: 5828,
            inputs: 7794,
        },
        (8, 20) => Sizes {
            style: 23400,
            table: 6141,
            inputs: 8210,
        },
        (8, 21) => Sizes {
            style: 24592,
            table: 6454,
            inputs: 8626,
        },
        (8, 22) => Sizes {
            style: 25784,
            table: 6767,
            inputs: 9042,
        },
        (8, 23) => Sizes {
            style: 26976,
            table: 7080,
            inputs: 9458,
        },
        (8, 24) => Sizes {
            style: 28168,
            table: 7393,
            inputs: 9874,
        },
        (8, 25) => Sizes {
            style: 29360,
            table: 7706,
            inputs: 10290,
        },
        (8, 26) => Sizes {
            style: 30552,
            table: 8019,
            inputs: 10706,
        },
        (8, 27) => Sizes {
            style: 31744,
            table: 8332,
            inputs: 11122,
        },
        (8, 28) => Sizes {
            style: 32936,
            table: 8645,
            inputs: 11538,
        },
        (8, 29) => Sizes {
            style: 34128,
            table: 8958,
            inputs: 11954,
        },
        (8, 30) => Sizes {
            style: 35320,
            table: 9271,
            inputs: 12370,
        },
        (8, 31) => Sizes {
            style: 36512,
            table: 9584,
            inputs: 12786,
        },
        (8, 32) => Sizes {
            style: 37704,
            table: 9897,
            inputs: 13202,
        },
        (8, 33) => Sizes {
            style: 38896,
            table: 10210,
            inputs: 13618,
        },
        (8, 34) => Sizes {
            style: 40088,
            table: 10523,
            inputs: 14034,
        },
        (8, 35) => Sizes {
            style: 41280,
            table: 10836,
            inputs: 14450,
        },
        (8, 36) => Sizes {
            style: 42472,
            table: 11149,
            inputs: 14866,
        },
        (8, 37) => Sizes {
            style: 43664,
            table: 11462,
            inputs: 15282,
        },
        (8, 38) => Sizes {
            style: 44856,
            table: 11775,
            inputs: 15698,
        },
        (8, 39) => Sizes {
            style: 46048,
            table: 12088,
            inputs: 16114,
        },
        (8, 40) => Sizes {
            style: 47240,
            table: 12401,
            inputs: 16530,
        },
        (8, 41) => Sizes {
            style: 48432,
            table: 12714,
            inputs: 16946,
        },
        (8, 42) => Sizes {
            style: 49624,
            table: 13027,
            inputs: 17362,
        },
        (8, 43) => Sizes {
            style: 50816,
            table: 13340,
            inputs: 17778,
        },
        (8, 44) => Sizes {
            style: 52008,
            table: 13653,
            inputs: 18194,
        },
        (8, 45) => Sizes {
            style: 53200,
            table: 13966,
            inputs: 18610,
        },
        (8, 46) => Sizes {
            style: 54392,
            table: 14279,
            inputs: 19026,
        },
        (8, 47) => Sizes {
            style: 55584,
            table: 14592,
            inputs: 19442,
        },
        (8, 48) => Sizes {
            style: 56776,
            table: 14905,
            inputs: 19858,
        },
        (8, 49) => Sizes {
            style: 57968,
            table: 15218,
            inputs: 20274,
        },
        (8, 50) => Sizes {
            style: 59160,
            table: 15531,
            inputs: 20690,
        },
        (9, 2) => Sizes {
            style: 2570,
            table: 665,
            inputs: 908,
        },
        (9, 3) => Sizes {
            style: 3875,
            table: 1007,
            inputs: 1367,
        },
        (9, 4) => Sizes {
            style: 5180,
            table: 1349,
            inputs: 1826,
        },
        (9, 5) => Sizes {
            style: 6485,
            table: 1691,
            inputs: 2285,
        },
        (9, 6) => Sizes {
            style: 7790,
            table: 2033,
            inputs: 2744,
        },
        (9, 7) => Sizes {
            style: 9095,
            table: 2375,
            inputs: 3203,
        },
        (9, 8) => Sizes {
            style: 10400,
            table: 2717,
            inputs: 3662,
        },
        (9, 9) => Sizes {
            style: 11705,
            table: 3059,
            inputs: 4121,
        },
        (9, 10) => Sizes {
            style: 13010,
            table: 3401,
            inputs: 4580,
        },
        (9, 11) => Sizes {
            style: 14315,
            table: 3743,
            inputs: 5039,
        },
        (9, 12) => Sizes {
            style: 15652,
            table: 4093,
            inputs: 5506,
        },
        (9, 13) => Sizes {
            style: 16993,
            table: 4444,
            inputs: 5974,
        },
        (9, 14) => Sizes {
            style: 18334,
            table: 4795,
            inputs: 6442,
        },
        (9, 15) => Sizes {
            style: 19675,
            table: 5146,
            inputs: 6910,
        },
        (9, 16) => Sizes {
            style: 21016,
            table: 5497,
            inputs: 7378,
        },
        (9, 17) => Sizes {
            style: 22357,
            table: 5848,
            inputs: 7846,
        },
        (9, 18) => Sizes {
            style: 23698,
            table: 6199,
            inputs: 8314,
        },
        (9, 19) => Sizes {
            style: 25039,
            table: 6550,
            inputs: 8782,
        },
        (9, 20) => Sizes {
            style: 26380,
            table: 6901,
            inputs: 9250,
        },
        (9, 21) => Sizes {
            style: 27721,
            table: 7252,
            inputs: 9718,
        },
        (9, 22) => Sizes {
            style: 29062,
            table: 7603,
            inputs: 10186,
        },
        (9, 23) => Sizes {
            style: 30403,
            table: 7954,
            inputs: 10654,
        },
        (9, 24) => Sizes {
            style: 31744,
            table: 8305,
            inputs: 11122,
        },
        (9, 25) => Sizes {
            style: 33085,
            table: 8656,
            inputs: 11590,
        },
        (9, 26) => Sizes {
            style: 34426,
            table: 9007,
            inputs: 12058,
        },
        (9, 27) => Sizes {
            style: 35767,
            table: 9358,
            inputs: 12526,
        },
        (9, 28) => Sizes {
            style: 37108,
            table: 9709,
            inputs: 12994,
        },
        (9, 29) => Sizes {
            style: 38449,
            table: 10060,
            inputs: 13462,
        },
        (9, 30) => Sizes {
            style: 39790,
            table: 10411,
            inputs: 13930,
        },
        (9, 31) => Sizes {
            style: 41131,
            table: 10762,
            inputs: 14398,
        },
        (9, 32) => Sizes {
            style: 42472,
            table: 11113,
            inputs: 14866,
        },
        (9, 33) => Sizes {
            style: 43813,
            table: 11464,
            inputs: 15334,
        },
        (9, 34) => Sizes {
            style: 45154,
            table: 11815,
            inputs: 15802,
        },
        (9, 35) => Sizes {
            style: 46495,
            table: 12166,
            inputs: 16270,
        },
        (9, 36) => Sizes {
            style: 47836,
            table: 12517,
            inputs: 16738,
        },
        (9, 37) => Sizes {
            style: 49177,
            table: 12868,
            inputs: 17206,
        },
        (9, 38) => Sizes {
            style: 50518,
            table: 13219,
            inputs: 17674,
        },
        (9, 39) => Sizes {
            style: 51859,
            table: 13570,
            inputs: 18142,
        },
        (9, 40) => Sizes {
            style: 53200,
            table: 13921,
            inputs: 18610,
        },
        (9, 41) => Sizes {
            style: 54541,
            table: 14272,
            inputs: 19078,
        },
        (9, 42) => Sizes {
            style: 55882,
            table: 14623,
            inputs: 19546,
        },
        (9, 43) => Sizes {
            style: 57223,
            table: 14974,
            inputs: 20014,
        },
        (9, 44) => Sizes {
            style: 58564,
            table: 15325,
            inputs: 20482,
        },
        (9, 45) => Sizes {
            style: 59905,
            table: 15676,
            inputs: 20950,
        },
        (9, 46) => Sizes {
            style: 61246,
            table: 16027,
            inputs: 21418,
        },
        (9, 47) => Sizes {
            style: 62587,
            table: 16378,
            inputs: 21886,
        },
        (9, 48) => Sizes {
            style: 63928,
            table: 16729,
            inputs: 22354,
        },
        (9, 49) => Sizes {
            style: 65269,
            table: 17080,
            inputs: 22822,
        },
        (9, 50) => Sizes {
            style: 66610,
            table: 17431,
            inputs: 23290,
        },
        (10, 2) => Sizes {
            style: 2860,
            table: 739,
            inputs: 1010,
        },
        (10, 3) => Sizes {
            style: 4310,
            table: 1118,
            inputs: 1520,
        },
        (10, 4) => Sizes {
            style: 5760,
            table: 1497,
            inputs: 2030,
        },
        (10, 5) => Sizes {
            style: 7210,
            table: 1876,
            inputs: 2540,
        },
        (10, 6) => Sizes {
            style: 8660,
            table: 2255,
            inputs: 3050,
        },
        (10, 7) => Sizes {
            style: 10110,
            table: 2634,
            inputs: 3560,
        },
        (10, 8) => Sizes {
            style: 11560,
            table: 3013,
            inputs: 4070,
        },
        (10, 9) => Sizes {
            style: 13010,
            table: 3392,
            inputs: 4580,
        },
        (10, 10) => Sizes {
            style: 14460,
            table: 3771,
            inputs: 5090,
        },
        (10, 11) => Sizes {
            style: 15950,
            table: 4160,
            inputs: 5610,
        },
        (10, 12) => Sizes {
            style: 17440,
            table: 4549,
            inputs: 6130,
        },
        (10, 13) => Sizes {
            style: 18930,
            table: 4938,
            inputs: 6650,
        },
        (10, 14) => Sizes {
            style: 20420,
            table: 5327,
            inputs: 7170,
        },
        (10, 15) => Sizes {
            style: 21910,
            table: 5716,
            inputs: 7690,
        },
        (10, 16) => Sizes {
            style: 23400,
            table: 6105,
            inputs: 8210,
        },
        (10, 17) => Sizes {
            style: 24890,
            table: 6494,
            inputs: 8730,
        },
        (10, 18) => Sizes {
            style: 26380,
            table: 6883,
            inputs: 9250,
        },
        (10, 19) => Sizes {
            style: 27870,
            table: 7272,
            inputs: 9770,
        },
        (10, 20) => Sizes {
            style: 29360,
            table: 7661,
            inputs: 10290,
        },
        (10, 21) => Sizes {
            style: 30850,
            table: 8050,
            inputs: 10810,
        },
        (10, 22) => Sizes {
            style: 32340,
            table: 8439,
            inputs: 11330,
        },
        (10, 23) => Sizes {
            style: 33830,
            table: 8828,
            inputs: 11850,
        },
        (10, 24) => Sizes {
            style: 35320,
            table: 9217,
            inputs: 12370,
        },
        (10, 25) => Sizes {
            style: 36810,
            table: 9606,
            inputs: 12890,
        },
        (10, 26) => Sizes {
            style: 38300,
            table: 9995,
            inputs: 13410,
        },
        (10, 27) => Sizes {
            style: 39790,
            table: 10384,
            inputs: 13930,
        },
        (10, 28) => Sizes {
            style: 41280,
            table: 10773,
            inputs: 14450,
        },
        (10, 29) => Sizes {
            style: 42770,
            table: 11162,
            inputs: 14970,
        },
        (10, 30) => Sizes {
            style: 44260,
            table: 11551,
            inputs: 15490,
        },
        (10, 31) => Sizes {
            style: 45750,
            table: 11940,
            inputs: 16010,
        },
        (10, 32) => Sizes {
            style: 47240,
            table: 12329,
            inputs: 16530,
        },
        (10, 33) => Sizes {
            style: 48730,
            table: 12718,
            inputs: 17050,
        },
        (10, 34) => Sizes {
            style: 50220,
            table: 13107,
            inputs: 17570,
        },
        (10, 35) => Sizes {
            style: 51710,
            table: 13496,
            inputs: 18090,
        },
        (10, 36) => Sizes {
            style: 53200,
            table: 13885,
            inputs: 18610,
        },
        (10, 37) => Sizes {
            style: 54690,
            table: 14274,
            inputs: 19130,
        },
        (10, 38) => Sizes {
            style: 56180,
            table: 14663,
            inputs: 19650,
        },
        (10, 39) => Sizes {
            style: 57670,
            table: 15052,
            inputs: 20170,
        },
        (10, 40) => Sizes {
            style: 59160,
            table: 15441,
            inputs: 20690,
        },
        (10, 41) => Sizes {
            style: 60650,
            table: 15830,
            inputs: 21210,
        },
        (10, 42) => Sizes {
            style: 62140,
            table: 16219,
            inputs: 21730,
        },
        (10, 43) => Sizes {
            style: 63630,
            table: 16608,
            inputs: 22250,
        },
        (10, 44) => Sizes {
            style: 65120,
            table: 16997,
            inputs: 22770,
        },
        (10, 45) => Sizes {
            style: 66610,
            table: 17386,
            inputs: 23290,
        },
        (10, 46) => Sizes {
            style: 68100,
            table: 17775,
            inputs: 23810,
        },
        (10, 47) => Sizes {
            style: 69590,
            table: 18164,
            inputs: 24330,
        },
        (10, 48) => Sizes {
            style: 71080,
            table: 18553,
            inputs: 24850,
        },
        (10, 49) => Sizes {
            style: 72570,
            table: 18942,
            inputs: 25370,
        },
        (10, 50) => Sizes {
            style: 74060,
            table: 19331,
            inputs: 25890,
        },
        (11, 2) => Sizes {
            style: 3150,
            table: 813,
            inputs: 1112,
        },
        (11, 3) => Sizes {
            style: 4745,
            table: 1229,
            inputs: 1673,
        },
        (11, 4) => Sizes {
            style: 6340,
            table: 1645,
            inputs: 2234,
        },
        (11, 5) => Sizes {
            style: 7935,
            table: 2061,
            inputs: 2795,
        },
        (11, 6) => Sizes {
            style: 9530,
            table: 2477,
            inputs: 3356,
        },
        (11, 7) => Sizes {
            style: 11125,
            table: 2893,
            inputs: 3917,
        },
        (11, 8) => Sizes {
            style: 12720,
            table: 3309,
            inputs: 4478,
        },
        (11, 9) => Sizes {
            style: 14315,
            table: 3725,
            inputs: 5039,
        },
        (11, 10) => Sizes {
            style: 15950,
            table: 4151,
            inputs: 5610,
        },
        (11, 11) => Sizes {
            style: 17589,
            table: 4578,
            inputs: 6182,
        },
        (11, 12) => Sizes {
            style: 19228,
            table: 5005,
            inputs: 6754,
        },
        (11, 13) => Sizes {
            style: 20867,
            table: 5432,
            inputs: 7326,
        },
        (11, 14) => Sizes {
            style: 22506,
            table: 5859,
            inputs: 7898,
        },
        (11, 15) => Sizes {
            style: 24145,
            table: 6286,
            inputs: 8470,
        },
        (11, 16) => Sizes {
            style: 25784,
            table: 6713,
            inputs: 9042,
        },
        (11, 17) => Sizes {
            style: 27423,
            table: 7140,
            inputs: 9614,
        },
        (11, 18) => Sizes {
            style: 29062,
            table: 7567,
            inputs: 10186,
        },
        (11, 19) => Sizes {
            style: 30701,
            table: 7994,
            inputs: 10758,
        },
        (11, 20) => Sizes {
            style: 32340,
            table: 8421,
            inputs: 11330,
        },
        (11, 21) => Sizes {
            style: 33979,
            table: 8848,
            inputs: 11902,
        },
        (11, 22) => Sizes {
            style: 35618,
            table: 9275,
            inputs: 12474,
        },
        (11, 23) => Sizes {
            style: 37257,
            table: 9702,
            inputs: 13046,
        },
        (11, 24) => Sizes {
            style: 38896,
            table: 10129,
            inputs: 13618,
        },
        (11, 25) => Sizes {
            style: 40535,
            table: 10556,
            inputs: 14190,
        },
        (11, 26) => Sizes {
            style: 42174,
            table: 10983,
            inputs: 14762,
        },
        (11, 27) => Sizes {
            style: 43813,
            table: 11410,
            inputs: 15334,
        },
        (11, 28) => Sizes {
            style: 45452,
            table: 11837,
            inputs: 15906,
        },
        (11, 29) => Sizes {
            style: 47091,
            table: 12264,
            inputs: 16478,
        },
        (11, 30) => Sizes {
            style: 48730,
            table: 12691,
            inputs: 17050,
        },
        (11, 31) => Sizes {
            style: 50369,
            table: 13118,
            inputs: 17622,
        },
        (11, 32) => Sizes {
            style: 52008,
            table: 13545,
            inputs: 18194,
        },
        (11, 33) => Sizes {
            style: 53647,
            table: 13972,
            inputs: 18766,
        },
        (11, 34) => Sizes {
            style: 55286,
            table: 14399,
            inputs: 19338,
        },
        (11, 35) => Sizes {
            style: 56925,
            table: 14826,
            inputs: 19910,
        },
        (11, 36) => Sizes {
            style: 58564,
            table: 15253,
            inputs: 20482,
        },
        (11, 37) => Sizes {
            style: 60203,
            table: 15680,
            inputs: 21054,
        },
        (11, 38) => Sizes {
            style: 61842,
            table: 16107,
            inputs: 21626,
        },
        (11, 39) => Sizes {
            style: 63481,
            table: 16534,
            inputs: 22198,
        },
        (11, 40) => Sizes {
            style: 65120,
            table: 16961,
            inputs: 22770,
        },
        (11, 41) => Sizes {
            style: 66759,
            table: 17388,
            inputs: 23342,
        },
        (11, 42) => Sizes {
            style: 68398,
            table: 17815,
            inputs: 23914,
        },
        (11, 43) => Sizes {
            style: 70037,
            table: 18242,
            inputs: 24486,
        },
        (11, 44) => Sizes {
            style: 71676,
            table: 18669,
            inputs: 25058,
        },
        (11, 45) => Sizes {
            style: 73315,
            table: 19096,
            inputs: 25630,
        },
        (11, 46) => Sizes {
            style: 74954,
            table: 19523,
            inputs: 26202,
        },
        (11, 47) => Sizes {
            style: 76593,
            table: 19950,
            inputs: 26774,
        },
        (11, 48) => Sizes {
            style: 78232,
            table: 20377,
            inputs: 27346,
        },
        (11, 49) => Sizes {
            style: 79871,
            table: 20804,
            inputs: 27918,
        },
        (11, 50) => Sizes {
            style: 81510,
            table: 21231,
            inputs: 28490,
        },
        (12, 2) => Sizes {
            style: 3440,
            table: 887,
            inputs: 1214,
        },
        (12, 3) => Sizes {
            style: 5180,
            table: 1340,
            inputs: 1826,
        },
        (12, 4) => Sizes {
            style: 6920,
            table: 1793,
            inputs: 2438,
        },
        (12, 5) => Sizes {
            style: 8660,
            table: 2246,
            inputs: 3050,
        },
        (12, 6) => Sizes {
            style: 10400,
            table: 2699,
            inputs: 3662,
        },
        (12, 7) => Sizes {
            style: 12140,
            table: 3152,
            inputs: 4274,
        },
        (12, 8) => Sizes {
            style: 13880,
            table: 3605,
            inputs: 4886,
        },
        (12, 9) => Sizes {
            style: 15652,
            table: 4066,
            inputs: 5506,
        },
        (12, 10) => Sizes {
            style: 17440,
            table: 4531,
            inputs: 6130,
        },
        (12, 11) => Sizes {
            style: 19228,
            table: 4996,
            inputs: 6754,
        },
        (12, 12) => Sizes {
            style: 21016,
            table: 5461,
            inputs: 7378,
        },
        (12, 13) => Sizes {
            style: 22804,
            table: 5926,
            inputs: 8002,
        },
        (12, 14) => Sizes {
            style: 24592,
            table: 6391,
            inputs: 8626,
        },
        (12, 15) => Sizes {
            style: 26380,
            table: 6856,
            inputs: 9250,
        },
        (12, 16) => Sizes {
            style: 28168,
            table: 7321,
            inputs: 9874,
        },
        (12, 17) => Sizes {
            style: 29956,
            table: 7786,
            inputs: 10498,
        },
        (12, 18) => Sizes {
            style: 31744,
            table: 8251,
            inputs: 11122,
        },
        (12, 19) => Sizes {
            style: 33532,
            table: 8716,
            inputs: 11746,
        },
        (12, 20) => Sizes {
            style: 35320,
            table: 9181,
            inputs: 12370,
        },
        (12, 21) => Sizes {
            style: 37108,
            table: 9646,
            inputs: 12994,
        },
        (12, 22) => Sizes {
            style: 38896,
            table: 10111,
            inputs: 13618,
        },
        (12, 23) => Sizes {
            style: 40684,
            table: 10576,
            inputs: 14242,
        },
        (12, 24) => Sizes {
            style: 42472,
            table: 11041,
            inputs: 14866,
        },
        (12, 25) => Sizes {
            style: 44260,
            table: 11506,
            inputs: 15490,
        },
        (12, 26) => Sizes {
            style: 46048,
            table: 11971,
            inputs: 16114,
        },
        (12, 27) => Sizes {
            style: 47836,
            table: 12436,
            inputs: 16738,
        },
        (12, 28) => Sizes {
            style: 49624,
            table: 12901,
            inputs: 17362,
        },
        (12, 29) => Sizes {
            style: 51412,
            table: 13366,
            inputs: 17986,
        },
        (12, 30) => Sizes {
            style: 53200,
            table: 13831,
            inputs: 18610,
        },
        (12, 31) => Sizes {
            style: 54988,
            table: 14296,
            inputs: 19234,
        },
        (12, 32) => Sizes {
            style: 56776,
            table: 14761,
            inputs: 19858,
        },
        (12, 33) => Sizes {
            style: 58564,
            table: 15226,
            inputs: 20482,
        },
        (12, 34) => Sizes {
            style: 60352,
            table: 15691,
            inputs: 21106,
        },
        (12, 35) => Sizes {
            style: 62140,
            table: 16156,
            inputs: 21730,
        },
        (12, 36) => Sizes {
            style: 63928,
            table: 16621,
            inputs: 22354,
        },
        (12, 37) => Sizes {
            style: 65716,
            table: 17086,
            inputs: 22978,
        },
        (12, 38) => Sizes {
            style: 67504,
            table: 17551,
            inputs: 23602,
        },
        (12, 39) => Sizes {
            style: 69292,
            table: 18016,
            inputs: 24226,
        },
        (12, 40) => Sizes {
            style: 71080,
            table: 18481,
            inputs: 24850,
        },
        (12, 41) => Sizes {
            style: 72868,
            table: 18946,
            inputs: 25474,
        },
        (12, 42) => Sizes {
            style: 74656,
            table: 19411,
            inputs: 26098,
        },
        (12, 43) => Sizes {
            style: 76444,
            table: 19876,
            inputs: 26722,
        },
        (12, 44) => Sizes {
            style: 78232,
            table: 20341,
            inputs: 27346,
        },
        (12, 45) => Sizes {
            style: 80020,
            table: 20806,
            inputs: 27970,
        },
        (12, 46) => Sizes {
            style: 81808,
            table: 21271,
            inputs: 28594,
        },
        (12, 47) => Sizes {
            style: 83596,
            table: 21736,
            inputs: 29218,
        },
        (12, 48) => Sizes {
            style: 85384,
            table: 22201,
            inputs: 29842,
        },
        (12, 49) => Sizes {
            style: 87172,
            table: 22666,
            inputs: 30466,
        },
        (12, 50) => Sizes {
            style: 88960,
            table: 23131,
            inputs: 31090,
        },
        (13, 2) => Sizes {
            style: 3730,
            table: 961,
            inputs: 1316,
        },
        (13, 3) => Sizes {
            style: 5615,
            table: 1451,
            inputs: 1979,
        },
        (13, 4) => Sizes {
            style: 7500,
            table: 1941,
            inputs: 2642,
        },
        (13, 5) => Sizes {
            style: 9385,
            table: 2431,
            inputs: 3305,
        },
        (13, 6) => Sizes {
            style: 11270,
            table: 2921,
            inputs: 3968,
        },
        (13, 7) => Sizes {
            style: 13155,
            table: 3411,
            inputs: 4631,
        },
        (13, 8) => Sizes {
            style: 15056,
            table: 3905,
            inputs: 5298,
        },
        (13, 9) => Sizes {
            style: 16993,
            table: 4408,
            inputs: 5974,
        },
        (13, 10) => Sizes {
            style: 18930,
            table: 4911,
            inputs: 6650,
        },
        (13, 11) => Sizes {
            style: 20867,
            table: 5414,
            inputs: 7326,
        },
        (13, 12) => Sizes {
            style: 22804,
            table: 5917,
            inputs: 8002,
        },
        (13, 13) => Sizes {
            style: 24741,
            table: 6420,
            inputs: 8678,
        },
        (13, 14) => Sizes {
            style: 26678,
            table: 6923,
            inputs: 9354,
        },
        (13, 15) => Sizes {
            style: 28615,
            table: 7426,
            inputs: 10030,
        },
        (13, 16) => Sizes {
            style: 30552,
            table: 7929,
            inputs: 10706,
        },
        (13, 17) => Sizes {
            style: 32489,
            table: 8432,
            inputs: 11382,
        },
        (13, 18) => Sizes {
            style: 34426,
            table: 8935,
            inputs: 12058,
        },
        (13, 19) => Sizes {
            style: 36363,
            table: 9438,
            inputs: 12734,
        },
        (13, 20) => Sizes {
            style: 38300,
            table: 9941,
            inputs: 13410,
        },
        (13, 21) => Sizes {
            style: 40237,
            table: 10444,
            inputs: 14086,
        },
        (13, 22) => Sizes {
            style: 42174,
            table: 10947,
            inputs: 14762,
        },
        (13, 23) => Sizes {
            style: 44111,
            table: 11450,
            inputs: 15438,
        },
        (13, 24) => Sizes {
            style: 46048,
            table: 11953,
            inputs: 16114,
        },
        (13, 25) => Sizes {
            style: 47985,
            table: 12456,
            inputs: 16790,
        },
        (13, 26) => Sizes {
            style: 49922,
            table: 12959,
            inputs: 17466,
        },
        (13, 27) => Sizes {
            style: 51859,
            table: 13462,
            inputs: 18142,
        },
        (13, 28) => Sizes {
            style: 53796,
            table: 13965,
            inputs: 18818,
        },
        (13, 29) => Sizes {
            style: 55733,
            table: 14468,
            inputs: 19494,
        },
        (13, 30) => Sizes {
            style: 57670,
            table: 14971,
            inputs: 20170,
        },
        (13, 31) => Sizes {
            style: 59607,
            table: 15474,
            inputs: 20846,
        },
        (13, 32) => Sizes {
            style: 61544,
            table: 15977,
            inputs: 21522,
        },
        (13, 33) => Sizes {
            style: 63481,
            table: 16480,
            inputs: 22198,
        },
        (13, 34) => Sizes {
            style: 65418,
            table: 16983,
            inputs: 22874,
        },
        (13, 35) => Sizes {
            style: 67355,
            table: 17486,
            inputs: 23550,
        },
        (13, 36) => Sizes {
            style: 69292,
            table: 17989,
            inputs: 24226,
        },
        (13, 37) => Sizes {
            style: 71229,
            table: 18492,
            inputs: 24902,
        },
        (13, 38) => Sizes {
            style: 73166,
            table: 18995,
            inputs: 25578,
        },
        (13, 39) => Sizes {
            style: 75103,
            table: 19498,
            inputs: 26254,
        },
        (13, 40) => Sizes {
            style: 77040,
            table: 20001,
            inputs: 26930,
        },
        (13, 41) => Sizes {
            style: 78977,
            table: 20504,
            inputs: 27606,
        },
        (13, 42) => Sizes {
            style: 80914,
            table: 21007,
            inputs: 28282,
        },
        (13, 43) => Sizes {
            style: 82851,
            table: 21510,
            inputs: 28958,
        },
        (13, 44) => Sizes {
            style: 84788,
            table: 22013,
            inputs: 29634,
        },
        (13, 45) => Sizes {
            style: 86725,
            table: 22516,
            inputs: 30310,
        },
        (13, 46) => Sizes {
            style: 88662,
            table: 23019,
            inputs: 30986,
        },
        (13, 47) => Sizes {
            style: 90599,
            table: 23522,
            inputs: 31662,
        },
        (13, 48) => Sizes {
            style: 92536,
            table: 24025,
            inputs: 32338,
        },
        (13, 49) => Sizes {
            style: 94473,
            table: 24528,
            inputs: 33014,
        },
        (13, 50) => Sizes {
            style: 96410,
            table: 25031,
            inputs: 33690,
        },
        (14, 2) => Sizes {
            style: 4020,
            table: 1035,
            inputs: 1418,
        },
        (14, 3) => Sizes {
            style: 6050,
            table: 1562,
            inputs: 2132,
        },
        (14, 4) => Sizes {
            style: 8080,
            table: 2089,
            inputs: 2846,
        },
        (14, 5) => Sizes {
            style: 10110,
            table: 2616,
            inputs: 3560,
        },
        (14, 6) => Sizes {
            style: 12140,
            table: 3143,
            inputs: 4274,
        },
        (14, 7) => Sizes {
            style: 14170,
            table: 3670,
            inputs: 4988,
        },
        (14, 8) => Sizes {
            style: 16248,
            table: 4209,
            inputs: 5714,
        },
        (14, 9) => Sizes {
            style: 18334,
            table: 4750,
            inputs: 6442,
        },
        (14, 10) => Sizes {
            style: 20420,
            table: 5291,
            inputs: 7170,
        },
        (14, 11) => Sizes {
            style: 22506,
            table: 5832,
            inputs: 7898,
        },
        (14, 12) => Sizes {
            style: 24592,
            table: 6373,
            inputs: 8626,
        },
        (14, 13) => Sizes {
            style: 26678,
            table: 6914,
            inputs: 9354,
        },
        (14, 14) => Sizes {
            style: 28764,
            table: 7455,
            inputs: 10082,
        },
        (14, 15) => Sizes {
            style: 30850,
            table: 7996,
            inputs: 10810,
        },
        (14, 16) => Sizes {
            style: 32936,
            table: 8537,
            inputs: 11538,
        },
        (14, 17) => Sizes {
            style: 35022,
            table: 9078,
            inputs: 12266,
        },
        (14, 18) => Sizes {
            style: 37108,
            table: 9619,
            inputs: 12994,
        },
        (14, 19) => Sizes {
            style: 39194,
            table: 10160,
            inputs: 13722,
        },
        (14, 20) => Sizes {
            style: 41280,
            table: 10701,
            inputs: 14450,
        },
        (14, 21) => Sizes {
            style: 43366,
            table: 11242,
            inputs: 15178,
        },
        (14, 22) => Sizes {
            style: 45452,
            table: 11783,
            inputs: 15906,
        },
        (14, 23) => Sizes {
            style: 47538,
            table: 12324,
            inputs: 16634,
        },
        (14, 24) => Sizes {
            style: 49624,
            table: 12865,
            inputs: 17362,
        },
        (14, 25) => Sizes {
            style: 51710,
            table: 13406,
            inputs: 18090,
        },
        (14, 26) => Sizes {
            style: 53796,
            table: 13947,
            inputs: 18818,
        },
        (14, 27) => Sizes {
            style: 55882,
            table: 14488,
            inputs: 19546,
        },
        (14, 28) => Sizes {
            style: 57968,
            table: 15029,
            inputs: 20274,
        },
        (14, 29) => Sizes {
            style: 60054,
            table: 15570,
            inputs: 21002,
        },
        (14, 30) => Sizes {
            style: 62140,
            table: 16111,
            inputs: 21730,
        },
        (14, 31) => Sizes {
            style: 64226,
            table: 16652,
            inputs: 22458,
        },
        (14, 32) => Sizes {
            style: 66312,
            table: 17193,
            inputs: 23186,
        },
        (14, 33) => Sizes {
            style: 68398,
            table: 17734,
            inputs: 23914,
        },
        (14, 34) => Sizes {
            style: 70484,
            table: 18275,
            inputs: 24642,
        },
        (14, 35) => Sizes {
            style: 72570,
            table: 18816,
            inputs: 25370,
        },
        (14, 36) => Sizes {
            style: 74656,
            table: 19357,
            inputs: 26098,
        },
        (14, 37) => Sizes {
            style: 76742,
            table: 19898,
            inputs: 26826,
        },
        (14, 38) => Sizes {
            style: 78828,
            table: 20439,
            inputs: 27554,
        },
        (14, 39) => Sizes {
            style: 80914,
            table: 20980,
            inputs: 28282,
        },
        (14, 40) => Sizes {
            style: 83000,
            table: 21521,
            inputs: 29010,
        },
        (14, 41) => Sizes {
            style: 85086,
            table: 22062,
            inputs: 29738,
        },
        (14, 42) => Sizes {
            style: 87172,
            table: 22603,
            inputs: 30466,
        },
        (14, 43) => Sizes {
            style: 89258,
            table: 23144,
            inputs: 31194,
        },
        (14, 44) => Sizes {
            style: 91344,
            table: 23685,
            inputs: 31922,
        },
        (14, 45) => Sizes {
            style: 93430,
            table: 24226,
            inputs: 32650,
        },
        (14, 46) => Sizes {
            style: 95516,
            table: 24767,
            inputs: 33378,
        },
        (14, 47) => Sizes {
            style: 97602,
            table: 25308,
            inputs: 34106,
        },
        (14, 48) => Sizes {
            style: 99688,
            table: 25849,
            inputs: 34834,
        },
        (14, 49) => Sizes {
            style: 101774,
            table: 26390,
            inputs: 35562,
        },
        (14, 50) => Sizes {
            style: 103860,
            table: 26931,
            inputs: 36290,
        },
        (15, 2) => Sizes {
            style: 4310,
            table: 1109,
            inputs: 1520,
        },
        (15, 3) => Sizes {
            style: 6485,
            table: 1673,
            inputs: 2285,
        },
        (15, 4) => Sizes {
            style: 8660,
            table: 2237,
            inputs: 3050,
        },
        (15, 5) => Sizes {
            style: 10835,
            table: 2801,
            inputs: 3815,
        },
        (15, 6) => Sizes {
            style: 13010,
            table: 3365,
            inputs: 4580,
        },
        (15, 7) => Sizes {
            style: 15205,
            table: 3934,
            inputs: 5350,
        },
        (15, 8) => Sizes {
            style: 17440,
            table: 4513,
            inputs: 6130,
        },
        (15, 9) => Sizes {
            style: 19675,
            table: 5092,
            inputs: 6910,
        },
        (15, 10) => Sizes {
            style: 21910,
            table: 5671,
            inputs: 7690,
        },
        (15, 11) => Sizes {
            style: 24145,
            table: 6250,
            inputs: 8470,
        },
        (15, 12) => Sizes {
            style: 26380,
            table: 6829,
            inputs: 9250,
        },
        (15, 13) => Sizes {
            style: 28615,
            table: 7408,
            inputs: 10030,
        },
        (15, 14) => Sizes {
            style: 30850,
            table: 7987,
            inputs: 10810,
        },
        (15, 15) => Sizes {
            style: 33085,
            table: 8566,
            inputs: 11590,
        },
        (15, 16) => Sizes {
            style: 35320,
            table: 9145,
            inputs: 12370,
        },
        (15, 17) => Sizes {
            style: 37555,
            table: 9724,
            inputs: 13150,
        },
        (15, 18) => Sizes {
            style: 39790,
            table: 10303,
            inputs: 13930,
        },
        (15, 19) => Sizes {
            style: 42025,
            table: 10882,
            inputs: 14710,
        },
        (15, 20) => Sizes {
            style: 44260,
            table: 11461,
            inputs: 15490,
        },
        (15, 21) => Sizes {
            style: 46495,
            table: 12040,
            inputs: 16270,
        },
        (15, 22) => Sizes {
            style: 48730,
            table: 12619,
            inputs: 17050,
        },
        (15, 23) => Sizes {
            style: 50965,
            table: 13198,
            inputs: 17830,
        },
        (15, 24) => Sizes {
            style: 53200,
            table: 13777,
            inputs: 18610,
        },
        (15, 25) => Sizes {
            style: 55435,
            table: 14356,
            inputs: 19390,
        },
        (15, 26) => Sizes {
            style: 57670,
            table: 14935,
            inputs: 20170,
        },
        (15, 27) => Sizes {
            style: 59905,
            table: 15514,
            inputs: 20950,
        },
        (15, 28) => Sizes {
            style: 62140,
            table: 16093,
            inputs: 21730,
        },
        (15, 29) => Sizes {
            style: 64375,
            table: 16672,
            inputs: 22510,
        },
        (15, 30) => Sizes {
            style: 66610,
            table: 17251,
            inputs: 23290,
        },
        (15, 31) => Sizes {
            style: 68845,
            table: 17830,
            inputs: 24070,
        },
        (15, 32) => Sizes {
            style: 71080,
            table: 18409,
            inputs: 24850,
        },
        (15, 33) => Sizes {
            style: 73315,
            table: 18988,
            inputs: 25630,
        },
        (15, 34) => Sizes {
            style: 75550,
            table: 19567,
            inputs: 26410,
        },
        (15, 35) => Sizes {
            style: 77785,
            table: 20146,
            inputs: 27190,
        },
        (15, 36) => Sizes {
            style: 80020,
            table: 20725,
            inputs: 27970,
        },
        (15, 37) => Sizes {
            style: 82255,
            table: 21304,
            inputs: 28750,
        },
        (15, 38) => Sizes {
            style: 84490,
            table: 21883,
            inputs: 29530,
        },
        (15, 39) => Sizes {
            style: 86725,
            table: 22462,
            inputs: 30310,
        },
        (15, 40) => Sizes {
            style: 88960,
            table: 23041,
            inputs: 31090,
        },
        (15, 41) => Sizes {
            style: 91195,
            table: 23620,
            inputs: 31870,
        },
        (15, 42) => Sizes {
            style: 93430,
            table: 24199,
            inputs: 32650,
        },
        (15, 43) => Sizes {
            style: 95665,
            table: 24778,
            inputs: 33430,
        },
        (15, 44) => Sizes {
            style: 97900,
            table: 25357,
            inputs: 34210,
        },
        (15, 45) => Sizes {
            style: 100135,
            table: 25936,
            inputs: 34990,
        },
        (15, 46) => Sizes {
            style: 102370,
            table: 26515,
            inputs: 35770,
        },
        (15, 47) => Sizes {
            style: 104605,
            table: 27094,
            inputs: 36550,
        },
        (15, 48) => Sizes {
            style: 106840,
            table: 27673,
            inputs: 37330,
        },
        (15, 49) => Sizes {
            style: 109075,
            table: 28252,
            inputs: 38110,
        },
        (15, 50) => Sizes {
            style: 111310,
            table: 28831,
            inputs: 38890,
        },
        (16, 2) => Sizes {
            style: 4600,
            table: 1183,
            inputs: 1622,
        },
        (16, 3) => Sizes {
            style: 6920,
            table: 1784,
            inputs: 2438,
        },
        (16, 4) => Sizes {
            style: 9240,
            table: 2385,
            inputs: 3254,
        },
        (16, 5) => Sizes {
            style: 11560,
            table: 2986,
            inputs: 4070,
        },
        (16, 6) => Sizes {
            style: 13880,
            table: 3587,
            inputs: 4886,
        },
        (16, 7) => Sizes {
            style: 16248,
            table: 4200,
            inputs: 5714,
        },
        (16, 8) => Sizes {
            style: 18632,
            table: 4817,
            inputs: 6546,
        },
        (16, 9) => Sizes {
            style: 21016,
            table: 5434,
            inputs: 7378,
        },
        (16, 10) => Sizes {
            style: 23400,
            table: 6051,
            inputs: 8210,
        },
        (16, 11) => Sizes {
            style: 25784,
            table: 6668,
            inputs: 9042,
        },
        (16, 12) => Sizes {
            style: 28168,
            table: 7285,
            inputs: 9874,
        },
        (16, 13) => Sizes {
            style: 30552,
            table: 7902,
            inputs: 10706,
        },
        (16, 14) => Sizes {
            style: 32936,
            table: 8519,
            inputs: 11538,
        },
        (16, 15) => Sizes {
            style: 35320,
            table: 9136,
            inputs: 12370,
        },
        (16, 16) => Sizes {
            style: 37704,
            table: 9753,
            inputs: 13202,
        },
        (16, 17) => Sizes {
            style: 40088,
            table: 10370,
            inputs: 14034,
        },
        (16, 18) => Sizes {
            style: 42472,
            table: 10987,
            inputs: 14866,
        },
        (16, 19) => Sizes {
            style: 44856,
            table: 11604,
            inputs: 15698,
        },
        (16, 20) => Sizes {
            style: 47240,
            table: 12221,
            inputs: 16530,
        },
        (16, 21) => Sizes {
            style: 49624,
            table: 12838,
            inputs: 17362,
        },
        (16, 22) => Sizes {
            style: 52008,
            table: 13455,
            inputs: 18194,
        },
        (16, 23) => Sizes {
            style: 54392,
            table: 14072,
            inputs: 19026,
        },
        (16, 24) => Sizes {
            style: 56776,
            table: 14689,
            inputs: 19858,
        },
        (16, 25) => Sizes {
            style: 59160,
            table: 15306,
            inputs: 20690,
        },
        (16, 26) => Sizes {
            style: 61544,
            table: 15923,
            inputs: 21522,
        },
        (16, 27) => Sizes {
            style: 63928,
            table: 16540,
            inputs: 22354,
        },
        (16, 28) => Sizes {
            style: 66312,
            table: 17157,
            inputs: 23186,
        },
        (16, 29) => Sizes {
            style: 68696,
            table: 17774,
            inputs: 24018,
        },
        (16, 30) => Sizes {
            style: 71080,
            table: 18391,
            inputs: 24850,
        },
        (16, 31) => Sizes {
            style: 73464,
            table: 19008,
            inputs: 25682,
        },
        (16, 32) => Sizes {
            style: 75848,
            table: 19625,
            inputs: 26514,
        },
        (16, 33) => Sizes {
            style: 78232,
            table: 20242,
            inputs: 27346,
        },
        (16, 34) => Sizes {
            style: 80616,
            table: 20859,
            inputs: 28178,
        },
        (16, 35) => Sizes {
            style: 83000,
            table: 21476,
            inputs: 29010,
        },
        (16, 36) => Sizes {
            style: 85384,
            table: 22093,
            inputs: 29842,
        },
        (16, 37) => Sizes {
            style: 87768,
            table: 22710,
            inputs: 30674,
        },
        (16, 38) => Sizes {
            style: 90152,
            table: 23327,
            inputs: 31506,
        },
        (16, 39) => Sizes {
            style: 92536,
            table: 23944,
            inputs: 32338,
        },
        (16, 40) => Sizes {
            style: 94920,
            table: 24561,
            inputs: 33170,
        },
        (16, 41) => Sizes {
            style: 97304,
            table: 25178,
            inputs: 34002,
        },
        (16, 42) => Sizes {
            style: 99688,
            table: 25795,
            inputs: 34834,
        },
        (16, 43) => Sizes {
            style: 102072,
            table: 26412,
            inputs: 35666,
        },
        (16, 44) => Sizes {
            style: 104456,
            table: 27029,
            inputs: 36498,
        },
        (16, 45) => Sizes {
            style: 106840,
            table: 27646,
            inputs: 37330,
        },
        (16, 46) => Sizes {
            style: 109224,
            table: 28263,
            inputs: 38162,
        },
        (16, 47) => Sizes {
            style: 111608,
            table: 28880,
            inputs: 38994,
        },
        (16, 48) => Sizes {
            style: 113992,
            table: 29497,
            inputs: 39826,
        },
        (16, 49) => Sizes {
            style: 116376,
            table: 30114,
            inputs: 40658,
        },
        (16, 50) => Sizes {
            style: 118760,
            table: 30731,
            inputs: 41490,
        },
        (17, 2) => Sizes {
            style: 4890,
            table: 1257,
            inputs: 1724,
        },
        (17, 3) => Sizes {
            style: 7355,
            table: 1895,
            inputs: 2591,
        },
        (17, 4) => Sizes {
            style: 9820,
            table: 2533,
            inputs: 3458,
        },
        (17, 5) => Sizes {
            style: 12285,
            table: 3171,
            inputs: 4325,
        },
        (17, 6) => Sizes {
            style: 14758,
            table: 3811,
            inputs: 5194,
        },
        (17, 7) => Sizes {
            style: 17291,
            table: 4466,
            inputs: 6078,
        },
        (17, 8) => Sizes {
            style: 19824,
            table: 5121,
            inputs: 6962,
        },
        (17, 9) => Sizes {
            style: 22357,
            table: 5776,
            inputs: 7846,
        },
        (17, 10) => Sizes {
            style: 24890,
            table: 6431,
            inputs: 8730,
        },
        (17, 11) => Sizes {
            style: 27423,
            table: 7086,
            inputs: 9614,
        },
        (17, 12) => Sizes {
            style: 29956,
            table: 7741,
            inputs: 10498,
        },
        (17, 13) => Sizes {
            style: 32489,
            table: 8396,
            inputs: 11382,
        },
        (17, 14) => Sizes {
            style: 35022,
            table: 9051,
            inputs: 12266,
        },
        (17, 15) => Sizes {
            style: 37555,
            table: 9706,
            inputs: 13150,
        },
        (17, 16) => Sizes {
            style: 40088,
            table: 10361,
            inputs: 14034,
        },
        (17, 17) => Sizes {
            style: 42621,
            table: 11016,
            inputs: 14918,
        },
        (17, 18) => Sizes {
            style: 45154,
            table: 11671,
            inputs: 15802,
        },
        (17, 19) => Sizes {
            style: 47687,
            table: 12326,
            inputs: 16686,
        },
        (17, 20) => Sizes {
            style: 50220,
            table: 12981,
            inputs: 17570,
        },
        (17, 21) => Sizes {
            style: 52753,
            table: 13636,
            inputs: 18454,
        },
        (17, 22) => Sizes {
            style: 55286,
            table: 14291,
            inputs: 19338,
        },
        (17, 23) => Sizes {
            style: 57819,
            table: 14946,
            inputs: 20222,
        },
        (17, 24) => Sizes {
            style: 60352,
            table: 15601,
            inputs: 21106,
        },
        (17, 25) => Sizes {
            style: 62885,
            table: 16256,
            inputs: 21990,
        },
        (17, 26) => Sizes {
            style: 65418,
            table: 16911,
            inputs: 22874,
        },
        (17, 27) => Sizes {
            style: 67951,
            table: 17566,
            inputs: 23758,
        },
        (17, 28) => Sizes {
            style: 70484,
            table: 18221,
            inputs: 24642,
        },
        (17, 29) => Sizes {
            style: 73017,
            table: 18876,
            inputs: 25526,
        },
        (17, 30) => Sizes {
            style: 75550,
            table: 19531,
            inputs: 26410,
        },
        (17, 31) => Sizes {
            style: 78083,
            table: 20186,
            inputs: 27294,
        },
        (17, 32) => Sizes {
            style: 80616,
            table: 20841,
            inputs: 28178,
        },
        (17, 33) => Sizes {
            style: 83149,
            table: 21496,
            inputs: 29062,
        },
        (17, 34) => Sizes {
            style: 85682,
            table: 22151,
            inputs: 29946,
        },
        (17, 35) => Sizes {
            style: 88215,
            table: 22806,
            inputs: 30830,
        },
        (17, 36) => Sizes {
            style: 90748,
            table: 23461,
            inputs: 31714,
        },
        (17, 37) => Sizes {
            style: 93281,
            table: 24116,
            inputs: 32598,
        },
        (17, 38) => Sizes {
            style: 95814,
            table: 24771,
            inputs: 33482,
        },
        (17, 39) => Sizes {
            style: 98347,
            table: 25426,
            inputs: 34366,
        },
        (17, 40) => Sizes {
            style: 100880,
            table: 26081,
            inputs: 35250,
        },
        (17, 41) => Sizes {
            style: 103413,
            table: 26736,
            inputs: 36134,
        },
        (17, 42) => Sizes {
            style: 105946,
            table: 27391,
            inputs: 37018,
        },
        (17, 43) => Sizes {
            style: 108479,
            table: 28046,
            inputs: 37902,
        },
        (17, 44) => Sizes {
            style: 111012,
            table: 28701,
            inputs: 38786,
        },
        (17, 45) => Sizes {
            style: 113545,
            table: 29356,
            inputs: 39670,
        },
        (17, 46) => Sizes {
            style: 116078,
            table: 30011,
            inputs: 40554,
        },
        (17, 47) => Sizes {
            style: 118611,
            table: 30666,
            inputs: 41438,
        },
        (17, 48) => Sizes {
            style: 121144,
            table: 31321,
            inputs: 42322,
        },
        (17, 49) => Sizes {
            style: 123677,
            table: 31976,
            inputs: 43206,
        },
        (17, 50) => Sizes {
            style: 126210,
            table: 32631,
            inputs: 44090,
        },
        (18, 2) => Sizes {
            style: 5180,
            table: 1331,
            inputs: 1826,
        },
        (18, 3) => Sizes {
            style: 7790,
            table: 2006,
            inputs: 2744,
        },
        (18, 4) => Sizes {
            style: 10400,
            table: 2681,
            inputs: 3662,
        },
        (18, 5) => Sizes {
            style: 13010,
            table: 3356,
            inputs: 4580,
        },
        (18, 6) => Sizes {
            style: 15652,
            table: 4039,
            inputs: 5506,
        },
        (18, 7) => Sizes {
            style: 18334,
            table: 4732,
            inputs: 6442,
        },
        (18, 8) => Sizes {
            style: 21016,
            table: 5425,
            inputs: 7378,
        },
        (18, 9) => Sizes {
            style: 23698,
            table: 6118,
            inputs: 8314,
        },
        (18, 10) => Sizes {
            style: 26380,
            table: 6811,
            inputs: 9250,
        },
        (18, 11) => Sizes {
            style: 29062,
            table: 7504,
            inputs: 10186,
        },
        (18, 12) => Sizes {
            style: 31744,
            table: 8197,
            inputs: 11122,
        },
        (18, 13) => Sizes {
            style: 34426,
            table: 8890,
            inputs: 12058,
        },
        (18, 14) => Sizes {
            style: 37108,
            table: 9583,
            inputs: 12994,
        },
        (18, 15) => Sizes {
            style: 39790,
            table: 10276,
            inputs: 13930,
        },
        (18, 16) => Sizes {
            style: 42472,
            table: 10969,
            inputs: 14866,
        },
        (18, 17) => Sizes {
            style: 45154,
            table: 11662,
            inputs: 15802,
        },
        (18, 18) => Sizes {
            style: 47836,
            table: 12355,
            inputs: 16738,
        },
        (18, 19) => Sizes {
            style: 50518,
            table: 13048,
            inputs: 17674,
        },
        (18, 20) => Sizes {
            style: 53200,
            table: 13741,
            inputs: 18610,
        },
        (18, 21) => Sizes {
            style: 55882,
            table: 14434,
            inputs: 19546,
        },
        (18, 22) => Sizes {
            style: 58564,
            table: 15127,
            inputs: 20482,
        },
        (18, 23) => Sizes {
            style: 61246,
            table: 15820,
            inputs: 21418,
        },
        (18, 24) => Sizes {
            style: 63928,
            table: 16513,
            inputs: 22354,
        },
        (18, 25) => Sizes {
            style: 66610,
            table: 17206,
            inputs: 23290,
        },
        (18, 26) => Sizes {
            style: 69292,
            table: 17899,
            inputs: 24226,
        },
        (18, 27) => Sizes {
            style: 71974,
            table: 18592,
            inputs: 25162,
        },
        (18, 28) => Sizes {
            style: 74656,
            table: 19285,
            inputs: 26098,
        },
        (18, 29) => Sizes {
            style: 77338,
            table: 19978,
            inputs: 27034,
        },
        (18, 30) => Sizes {
            style: 80020,
            table: 20671,
            inputs: 27970,
        },
        (18, 31) => Sizes {
            style: 82702,
            table: 21364,
            inputs: 28906,
        },
        (18, 32) => Sizes {
            style: 85384,
            table: 22057,
            inputs: 29842,
        },
        (18, 33) => Sizes {
            style: 88066,
            table: 22750,
            inputs: 30778,
        },
        (18, 34) => Sizes {
            style: 90748,
            table: 23443,
            inputs: 31714,
        },
        (18, 35) => Sizes {
            style: 93430,
            table: 24136,
            inputs: 32650,
        },
        (18, 36) => Sizes {
            style: 96112,
            table: 24829,
            inputs: 33586,
        },
        (18, 37) => Sizes {
            style: 98794,
            table: 25522,
            inputs: 34522,
        },
        (18, 38) => Sizes {
            style: 101476,
            table: 26215,
            inputs: 35458,
        },
        (18, 39) => Sizes {
            style: 104158,
            table: 26908,
            inputs: 36394,
        },
        (18, 40) => Sizes {
            style: 106840,
            table: 27601,
            inputs: 37330,
        },
        (18, 41) => Sizes {
            style: 109522,
            table: 28294,
            inputs: 38266,
        },
        (18, 42) => Sizes {
            style: 112204,
            table: 28987,
            inputs: 39202,
        },
        (18, 43) => Sizes {
            style: 114886,
            table: 29680,
            inputs: 40138,
        },
        (18, 44) => Sizes {
            style: 117568,
            table: 30373,
            inputs: 41074,
        },
        (18, 45) => Sizes {
            style: 120250,
            table: 31066,
            inputs: 42010,
        },
        (18, 46) => Sizes {
            style: 122932,
            table: 31759,
            inputs: 42946,
        },
        (18, 47) => Sizes {
            style: 125614,
            table: 32452,
            inputs: 43882,
        },
        (18, 48) => Sizes {
            style: 128296,
            table: 33145,
            inputs: 44818,
        },
        (18, 49) => Sizes {
            style: 130978,
            table: 33838,
            inputs: 45754,
        },
        (18, 50) => Sizes {
            style: 133660,
            table: 34531,
            inputs: 46690,
        },
        (19, 2) => Sizes {
            style: 5470,
            table: 1405,
            inputs: 1928,
        },
        (19, 3) => Sizes {
            style: 8225,
            table: 2117,
            inputs: 2897,
        },
        (19, 4) => Sizes {
            style: 10980,
            table: 2829,
            inputs: 3866,
        },
        (19, 5) => Sizes {
            style: 13735,
            table: 3541,
            inputs: 4835,
        },
        (19, 6) => Sizes {
            style: 16546,
            table: 4267,
            inputs: 5818,
        },
        (19, 7) => Sizes {
            style: 19377,
            table: 4998,
            inputs: 6806,
        },
        (19, 8) => Sizes {
            style: 22208,
            table: 5729,
            inputs: 7794,
        },
        (19, 9) => Sizes {
            style: 25039,
            table: 6460,
            inputs: 8782,
        },
        (19, 10) => Sizes {
            style: 27870,
            table: 7191,
            inputs: 9770,
        },
        (19, 11) => Sizes {
            style: 30701,
            table: 7922,
            inputs: 10758,
        },
        (19, 12) => Sizes {
            style: 33532,
            table: 8653,
            inputs: 11746,
        },
        (19, 13) => Sizes {
            style: 36363,
            table: 9384,
            inputs: 12734,
        },
        (19, 14) => Sizes {
            style: 39194,
            table: 10115,
            inputs: 13722,
        },
        (19, 15) => Sizes {
            style: 42025,
            table: 10846,
            inputs: 14710,
        },
        (19, 16) => Sizes {
            style: 44856,
            table: 11577,
            inputs: 15698,
        },
        (19, 17) => Sizes {
            style: 47687,
            table: 12308,
            inputs: 16686,
        },
        (19, 18) => Sizes {
            style: 50518,
            table: 13039,
            inputs: 17674,
        },
        (19, 19) => Sizes {
            style: 53349,
            table: 13770,
            inputs: 18662,
        },
        (19, 20) => Sizes {
            style: 56180,
            table: 14501,
            inputs: 19650,
        },
        (19, 21) => Sizes {
            style: 59011,
            table: 15232,
            inputs: 20638,
        },
        (19, 22) => Sizes {
            style: 61842,
            table: 15963,
            inputs: 21626,
        },
        (19, 23) => Sizes {
            style: 64673,
            table: 16694,
            inputs: 22614,
        },
        (19, 24) => Sizes {
            style: 67504,
            table: 17425,
            inputs: 23602,
        },
        (19, 25) => Sizes {
            style: 70335,
            table: 18156,
            inputs: 24590,
        },
        (19, 26) => Sizes {
            style: 73166,
            table: 18887,
            inputs: 25578,
        },
        (19, 27) => Sizes {
            style: 75997,
            table: 19618,
            inputs: 26566,
        },
        (19, 28) => Sizes {
            style: 78828,
            table: 20349,
            inputs: 27554,
        },
        (19, 29) => Sizes {
            style: 81659,
            table: 21080,
            inputs: 28542,
        },
        (19, 30) => Sizes {
            style: 84490,
            table: 21811,
            inputs: 29530,
        },
        (19, 31) => Sizes {
            style: 87321,
            table: 22542,
            inputs: 30518,
        },
        (19, 32) => Sizes {
            style: 90152,
            table: 23273,
            inputs: 31506,
        },
        (19, 33) => Sizes {
            style: 92983,
            table: 24004,
            inputs: 32494,
        },
        (19, 34) => Sizes {
            style: 95814,
            table: 24735,
            inputs: 33482,
        },
        (19, 35) => Sizes {
            style: 98645,
            table: 25466,
            inputs: 34470,
        },
        (19, 36) => Sizes {
            style: 101476,
            table: 26197,
            inputs: 35458,
        },
        (19, 37) => Sizes {
            style: 104307,
            table: 26928,
            inputs: 36446,
        },
        (19, 38) => Sizes {
            style: 107138,
            table: 27659,
            inputs: 37434,
        },
        (19, 39) => Sizes {
            style: 109969,
            table: 28390,
            inputs: 38422,
        },
        (19, 40) => Sizes {
            style: 112800,
            table: 29121,
            inputs: 39410,
        },
        (19, 41) => Sizes {
            style: 115631,
            table: 29852,
            inputs: 40398,
        },
        (19, 42) => Sizes {
            style: 118462,
            table: 30583,
            inputs: 41386,
        },
        (19, 43) => Sizes {
            style: 121293,
            table: 31314,
            inputs: 42374,
        },
        (19, 44) => Sizes {
            style: 124124,
            table: 32045,
            inputs: 43362,
        },
        (19, 45) => Sizes {
            style: 126955,
            table: 32776,
            inputs: 44350,
        },
        (19, 46) => Sizes {
            style: 129786,
            table: 33507,
            inputs: 45338,
        },
        (19, 47) => Sizes {
            style: 132617,
            table: 34238,
            inputs: 46326,
        },
        (19, 48) => Sizes {
            style: 135448,
            table: 34969,
            inputs: 47314,
        },
        (19, 49) => Sizes {
            style: 138279,
            table: 35700,
            inputs: 48302,
        },
        (19, 50) => Sizes {
            style: 141110,
            table: 36431,
            inputs: 49290,
        },
        (20, 2) => Sizes {
            style: 5760,
            table: 1479,
            inputs: 2030,
        },
        (20, 3) => Sizes {
            style: 8660,
            table: 2228,
            inputs: 3050,
        },
        (20, 4) => Sizes {
            style: 11560,
            table: 2977,
            inputs: 4070,
        },
        (20, 5) => Sizes {
            style: 14460,
            table: 3726,
            inputs: 5090,
        },
        (20, 6) => Sizes {
            style: 17440,
            table: 4495,
            inputs: 6130,
        },
        (20, 7) => Sizes {
            style: 20420,
            table: 5264,
            inputs: 7170,
        },
        (20, 8) => Sizes {
            style: 23400,
            table: 6033,
            inputs: 8210,
        },
        (20, 9) => Sizes {
            style: 26380,
            table: 6802,
            inputs: 9250,
        },
        (20, 10) => Sizes {
            style: 29360,
            table: 7571,
            inputs: 10290,
        },
        (20, 11) => Sizes {
            style: 32340,
            table: 8340,
            inputs: 11330,
        },
        (20, 12) => Sizes {
            style: 35320,
            table: 9109,
            inputs: 12370,
        },
        (20, 13) => Sizes {
            style: 38300,
            table: 9878,
            inputs: 13410,
        },
        (20, 14) => Sizes {
            style: 41280,
            table: 10647,
            inputs: 14450,
        },
        (20, 15) => Sizes {
            style: 44260,
            table: 11416,
            inputs: 15490,
        },
        (20, 16) => Sizes {
            style: 47240,
            table: 12185,
            inputs: 16530,
        },
        (20, 17) => Sizes {
            style: 50220,
            table: 12954,
            inputs: 17570,
        },
        (20, 18) => Sizes {
            style: 53200,
            table: 13723,
            inputs: 18610,
        },
        (20, 19) => Sizes {
            style: 56180,
            table: 14492,
            inputs: 19650,
        },
        (20, 20) => Sizes {
            style: 59160,
            table: 15261,
            inputs: 20690,
        },
        (20, 21) => Sizes {
            style: 62140,
            table: 16030,
            inputs: 21730,
        },
        (20, 22) => Sizes {
            style: 65120,
            table: 16799,
            inputs: 22770,
        },
        (20, 23) => Sizes {
            style: 68100,
            table: 17568,
            inputs: 23810,
        },
        (20, 24) => Sizes {
            style: 71080,
            table: 18337,
            inputs: 24850,
        },
        (20, 25) => Sizes {
            style: 74060,
            table: 19106,
            inputs: 25890,
        },
        (20, 26) => Sizes {
            style: 77040,
            table: 19875,
            inputs: 26930,
        },
        (20, 27) => Sizes {
            style: 80020,
            table: 20644,
            inputs: 27970,
        },
        (20, 28) => Sizes {
            style: 83000,
            table: 21413,
            inputs: 29010,
        },
        (20, 29) => Sizes {
            style: 85980,
            table: 22182,
            inputs: 30050,
        },
        (20, 30) => Sizes {
            style: 88960,
            table: 22951,
            inputs: 31090,
        },
        (20, 31) => Sizes {
            style: 91940,
            table: 23720,
            inputs: 32130,
        },
        (20, 32) => Sizes {
            style: 94920,
            table: 24489,
            inputs: 33170,
        },
        (20, 33) => Sizes {
            style: 97900,
            table: 25258,
            inputs: 34210,
        },
        (20, 34) => Sizes {
            style: 100880,
            table: 26027,
            inputs: 35250,
        },
        (20, 35) => Sizes {
            style: 103860,
            table: 26796,
            inputs: 36290,
        },
        (20, 36) => Sizes {
            style: 106840,
            table: 27565,
            inputs: 37330,
        },
        (20, 37) => Sizes {
            style: 109820,
            table: 28334,
            inputs: 38370,
        },
        (20, 38) => Sizes {
            style: 112800,
            table: 29103,
            inputs: 39410,
        },
        (20, 39) => Sizes {
            style: 115780,
            table: 29872,
            inputs: 40450,
        },
        (20, 40) => Sizes {
            style: 118760,
            table: 30641,
            inputs: 41490,
        },
        (20, 41) => Sizes {
            style: 121740,
            table: 31410,
            inputs: 42530,
        },
        (20, 42) => Sizes {
            style: 124720,
            table: 32179,
            inputs: 43570,
        },
        (20, 43) => Sizes {
            style: 127700,
            table: 32948,
            inputs: 44610,
        },
        (20, 44) => Sizes {
            style: 130680,
            table: 33717,
            inputs: 45650,
        },
        (20, 45) => Sizes {
            style: 133660,
            table: 34486,
            inputs: 46690,
        },
        (20, 46) => Sizes {
            style: 136640,
            table: 35255,
            inputs: 47730,
        },
        (20, 47) => Sizes {
            style: 139620,
            table: 36024,
            inputs: 48770,
        },
        (20, 48) => Sizes {
            style: 142600,
            table: 36793,
            inputs: 49810,
        },
        (20, 49) => Sizes {
            style: 145580,
            table: 37562,
            inputs: 50850,
        },
        (20, 50) => Sizes {
            style: 148560,
            table: 38331,
            inputs: 51890,
        },
        (21, 2) => Sizes {
            style: 6050,
            table: 1553,
            inputs: 2132,
        },
        (21, 3) => Sizes {
            style: 9095,
            table: 2339,
            inputs: 3203,
        },
        (21, 4) => Sizes {
            style: 12140,
            table: 3125,
            inputs: 4274,
        },
        (21, 5) => Sizes {
            style: 15205,
            table: 3916,
            inputs: 5350,
        },
        (21, 6) => Sizes {
            style: 18334,
            table: 4723,
            inputs: 6442,
        },
        (21, 7) => Sizes {
            style: 21463,
            table: 5530,
            inputs: 7534,
        },
        (21, 8) => Sizes {
            style: 24592,
            table: 6337,
            inputs: 8626,
        },
        (21, 9) => Sizes {
            style: 27721,
            table: 7144,
            inputs: 9718,
        },
        (21, 10) => Sizes {
            style: 30850,
            table: 7951,
            inputs: 10810,
        },
        (21, 11) => Sizes {
            style: 33979,
            table: 8758,
            inputs: 11902,
        },
        (21, 12) => Sizes {
            style: 37108,
            table: 9565,
            inputs: 12994,
        },
        (21, 13) => Sizes {
            style: 40237,
            table: 10372,
            inputs: 14086,
        },
        (21, 14) => Sizes {
            style: 43366,
            table: 11179,
            inputs: 15178,
        },
        (21, 15) => Sizes {
            style: 46495,
            table: 11986,
            inputs: 16270,
        },
        (21, 16) => Sizes {
            style: 49624,
            table: 12793,
            inputs: 17362,
        },
        (21, 17) => Sizes {
            style: 52753,
            table: 13600,
            inputs: 18454,
        },
        (21, 18) => Sizes {
            style: 55882,
            table: 14407,
            inputs: 19546,
        },
        (21, 19) => Sizes {
            style: 59011,
            table: 15214,
            inputs: 20638,
        },
        (21, 20) => Sizes {
            style: 62140,
            table: 16021,
            inputs: 21730,
        },
        (21, 21) => Sizes {
            style: 65269,
            table: 16828,
            inputs: 22822,
        },
        (21, 22) => Sizes {
            style: 68398,
            table: 17635,
            inputs: 23914,
        },
        (21, 23) => Sizes {
            style: 71527,
            table: 18442,
            inputs: 25006,
        },
        (21, 24) => Sizes {
            style: 74656,
            table: 19249,
            inputs: 26098,
        },
        (21, 25) => Sizes {
            style: 77785,
            table: 20056,
            inputs: 27190,
        },
        (21, 26) => Sizes {
            style: 80914,
            table: 20863,
            inputs: 28282,
        },
        (21, 27) => Sizes {
            style: 84043,
            table: 21670,
            inputs: 29374,
        },
        (21, 28) => Sizes {
            style: 87172,
            table: 22477,
            inputs: 30466,
        },
        (21, 29) => Sizes {
            style: 90301,
            table: 23284,
            inputs: 31558,
        },
        (21, 30) => Sizes {
            style: 93430,
            table: 24091,
            inputs: 32650,
        },
        (21, 31) => Sizes {
            style: 96559,
            table: 24898,
            inputs: 33742,
        },
        (21, 32) => Sizes {
            style: 99688,
            table: 25705,
            inputs: 34834,
        },
        (21, 33) => Sizes {
            style: 102817,
            table: 26512,
            inputs: 35926,
        },
        (21, 34) => Sizes {
            style: 105946,
            table: 27319,
            inputs: 37018,
        },
        (21, 35) => Sizes {
            style: 109075,
            table: 28126,
            inputs: 38110,
        },
        (21, 36) => Sizes {
            style: 112204,
            table: 28933,
            inputs: 39202,
        },
        (21, 37) => Sizes {
            style: 115333,
            table: 29740,
            inputs: 40294,
        },
        (21, 38) => Sizes {
            style: 118462,
            table: 30547,
            inputs: 41386,
        },
        (21, 39) => Sizes {
            style: 121591,
            table: 31354,
            inputs: 42478,
        },
        (21, 40) => Sizes {
            style: 124720,
            table: 32161,
            inputs: 43570,
        },
        (21, 41) => Sizes {
            style: 127849,
            table: 32968,
            inputs: 44662,
        },
        (21, 42) => Sizes {
            style: 130978,
            table: 33775,
            inputs: 45754,
        },
        (21, 43) => Sizes {
            style: 134107,
            table: 34582,
            inputs: 46846,
        },
        (21, 44) => Sizes {
            style: 137236,
            table: 35389,
            inputs: 47938,
        },
        (21, 45) => Sizes {
            style: 140365,
            table: 36196,
            inputs: 49030,
        },
        (21, 46) => Sizes {
            style: 143494,
            table: 37003,
            inputs: 50122,
        },
        (21, 47) => Sizes {
            style: 146623,
            table: 37810,
            inputs: 51214,
        },
        (21, 48) => Sizes {
            style: 149784,
            table: 38625,
            inputs: 52314,
        },
        (21, 49) => Sizes {
            style: 152997,
            table: 39453,
            inputs: 53427,
        },
        (21, 50) => Sizes {
            style: 156210,
            table: 40281,
            inputs: 54540,
        },
        (22, 2) => Sizes {
            style: 6340,
            table: 1627,
            inputs: 2234,
        },
        (22, 3) => Sizes {
            style: 9530,
            table: 2450,
            inputs: 3356,
        },
        (22, 4) => Sizes {
            style: 12720,
            table: 3273,
            inputs: 4478,
        },
        (22, 5) => Sizes {
            style: 15950,
            table: 4106,
            inputs: 5610,
        },
        (22, 6) => Sizes {
            style: 19228,
            table: 4951,
            inputs: 6754,
        },
        (22, 7) => Sizes {
            style: 22506,
            table: 5796,
            inputs: 7898,
        },
        (22, 8) => Sizes {
            style: 25784,
            table: 6641,
            inputs: 9042,
        },
        (22, 9) => Sizes {
            style: 29062,
            table: 7486,
            inputs: 10186,
        },
        (22, 10) => Sizes {
            style: 32340,
            table: 8331,
            inputs: 11330,
        },
        (22, 11) => Sizes {
            style: 35618,
            table: 9176,
            inputs: 12474,
        },
        (22, 12) => Sizes {
            style: 38896,
            table: 10021,
            inputs: 13618,
        },
        (22, 13) => Sizes {
            style: 42174,
            table: 10866,
            inputs: 14762,
        },
        (22, 14) => Sizes {
            style: 45452,
            table: 11711,
            inputs: 15906,
        },
        (22, 15) => Sizes {
            style: 48730,
            table: 12556,
            inputs: 17050,
        },
        (22, 16) => Sizes {
            style: 52008,
            table: 13401,
            inputs: 18194,
        },
        (22, 17) => Sizes {
            style: 55286,
            table: 14246,
            inputs: 19338,
        },
        (22, 18) => Sizes {
            style: 58564,
            table: 15091,
            inputs: 20482,
        },
        (22, 19) => Sizes {
            style: 61842,
            table: 15936,
            inputs: 21626,
        },
        (22, 20) => Sizes {
            style: 65120,
            table: 16781,
            inputs: 22770,
        },
        (22, 21) => Sizes {
            style: 68398,
            table: 17626,
            inputs: 23914,
        },
        (22, 22) => Sizes {
            style: 71676,
            table: 18471,
            inputs: 25058,
        },
        (22, 23) => Sizes {
            style: 74954,
            table: 19316,
            inputs: 26202,
        },
        (22, 24) => Sizes {
            style: 78232,
            table: 20161,
            inputs: 27346,
        },
        (22, 25) => Sizes {
            style: 81510,
            table: 21006,
            inputs: 28490,
        },
        (22, 26) => Sizes {
            style: 84788,
            table: 21851,
            inputs: 29634,
        },
        (22, 27) => Sizes {
            style: 88066,
            table: 22696,
            inputs: 30778,
        },
        (22, 28) => Sizes {
            style: 91344,
            table: 23541,
            inputs: 31922,
        },
        (22, 29) => Sizes {
            style: 94622,
            table: 24386,
            inputs: 33066,
        },
        (22, 30) => Sizes {
            style: 97900,
            table: 25231,
            inputs: 34210,
        },
        (22, 31) => Sizes {
            style: 101178,
            table: 26076,
            inputs: 35354,
        },
        (22, 32) => Sizes {
            style: 104456,
            table: 26921,
            inputs: 36498,
        },
        (22, 33) => Sizes {
            style: 107734,
            table: 27766,
            inputs: 37642,
        },
        (22, 34) => Sizes {
            style: 111012,
            table: 28611,
            inputs: 38786,
        },
        (22, 35) => Sizes {
            style: 114290,
            table: 29456,
            inputs: 39930,
        },
        (22, 36) => Sizes {
            style: 117568,
            table: 30301,
            inputs: 41074,
        },
        (22, 37) => Sizes {
            style: 120846,
            table: 31146,
            inputs: 42218,
        },
        (22, 38) => Sizes {
            style: 124124,
            table: 31991,
            inputs: 43362,
        },
        (22, 39) => Sizes {
            style: 127402,
            table: 32836,
            inputs: 44506,
        },
        (22, 40) => Sizes {
            style: 130680,
            table: 33681,
            inputs: 45650,
        },
        (22, 41) => Sizes {
            style: 133958,
            table: 34526,
            inputs: 46794,
        },
        (22, 42) => Sizes {
            style: 137236,
            table: 35371,
            inputs: 47938,
        },
        (22, 43) => Sizes {
            style: 140514,
            table: 36216,
            inputs: 49082,
        },
        (22, 44) => Sizes {
            style: 143792,
            table: 37061,
            inputs: 50226,
        },
        (22, 45) => Sizes {
            style: 147070,
            table: 37906,
            inputs: 51370,
        },
        (22, 46) => Sizes {
            style: 150396,
            table: 38763,
            inputs: 52526,
        },
        (22, 47) => Sizes {
            style: 153762,
            table: 39630,
            inputs: 53692,
        },
        (22, 48) => Sizes {
            style: 157128,
            table: 40497,
            inputs: 54858,
        },
        (22, 49) => Sizes {
            style: 160494,
            table: 41364,
            inputs: 56024,
        },
        (22, 50) => Sizes {
            style: 163860,
            table: 42231,
            inputs: 57190,
        },
        (23, 2) => Sizes {
            style: 6630,
            table: 1701,
            inputs: 2336,
        },
        (23, 3) => Sizes {
            style: 9965,
            table: 2561,
            inputs: 3509,
        },
        (23, 4) => Sizes {
            style: 13300,
            table: 3421,
            inputs: 4682,
        },
        (23, 5) => Sizes {
            style: 16695,
            table: 4296,
            inputs: 5870,
        },
        (23, 6) => Sizes {
            style: 20122,
            table: 5179,
            inputs: 7066,
        },
        (23, 7) => Sizes {
            style: 23549,
            table: 6062,
            inputs: 8262,
        },
        (23, 8) => Sizes {
            style: 26976,
            table: 6945,
            inputs: 9458,
        },
        (23, 9) => Sizes {
            style: 30403,
            table: 7828,
            inputs: 10654,
        },
        (23, 10) => Sizes {
            style: 33830,
            table: 8711,
            inputs: 11850,
        },
        (23, 11) => Sizes {
            style: 37257,
            table: 9594,
            inputs: 13046,
        },
        (23, 12) => Sizes {
            style: 40684,
            table: 10477,
            inputs: 14242,
        },
        (23, 13) => Sizes {
            style: 44111,
            table: 11360,
            inputs: 15438,
        },
        (23, 14) => Sizes {
            style: 47538,
            table: 12243,
            inputs: 16634,
        },
        (23, 15) => Sizes {
            style: 50965,
            table: 13126,
            inputs: 17830,
        },
        (23, 16) => Sizes {
            style: 54392,
            table: 14009,
            inputs: 19026,
        },
        (23, 17) => Sizes {
            style: 57819,
            table: 14892,
            inputs: 20222,
        },
        (23, 18) => Sizes {
            style: 61246,
            table: 15775,
            inputs: 21418,
        },
        (23, 19) => Sizes {
            style: 64673,
            table: 16658,
            inputs: 22614,
        },
        (23, 20) => Sizes {
            style: 68100,
            table: 17541,
            inputs: 23810,
        },
        (23, 21) => Sizes {
            style: 71527,
            table: 18424,
            inputs: 25006,
        },
        (23, 22) => Sizes {
            style: 74954,
            table: 19307,
            inputs: 26202,
        },
        (23, 23) => Sizes {
            style: 78381,
            table: 20190,
            inputs: 27398,
        },
        (23, 24) => Sizes {
            style: 81808,
            table: 21073,
            inputs: 28594,
        },
        (23, 25) => Sizes {
            style: 85235,
            table: 21956,
            inputs: 29790,
        },
        (23, 26) => Sizes {
            style: 88662,
            table: 22839,
            inputs: 30986,
        },
        (23, 27) => Sizes {
            style: 92089,
            table: 23722,
            inputs: 32182,
        },
        (23, 28) => Sizes {
            style: 95516,
            table: 24605,
            inputs: 33378,
        },
        (23, 29) => Sizes {
            style: 98943,
            table: 25488,
            inputs: 34574,
        },
        (23, 30) => Sizes {
            style: 102370,
            table: 26371,
            inputs: 35770,
        },
        (23, 31) => Sizes {
            style: 105797,
            table: 27254,
            inputs: 36966,
        },
        (23, 32) => Sizes {
            style: 109224,
            table: 28137,
            inputs: 38162,
        },
        (23, 33) => Sizes {
            style: 112651,
            table: 29020,
            inputs: 39358,
        },
        (23, 34) => Sizes {
            style: 116078,
            table: 29903,
            inputs: 40554,
        },
        (23, 35) => Sizes {
            style: 119505,
            table: 30786,
            inputs: 41750,
        },
        (23, 36) => Sizes {
            style: 122932,
            table: 31669,
            inputs: 42946,
        },
        (23, 37) => Sizes {
            style: 126359,
            table: 32552,
            inputs: 44142,
        },
        (23, 38) => Sizes {
            style: 129786,
            table: 33435,
            inputs: 45338,
        },
        (23, 39) => Sizes {
            style: 133213,
            table: 34318,
            inputs: 46534,
        },
        (23, 40) => Sizes {
            style: 136640,
            table: 35201,
            inputs: 47730,
        },
        (23, 41) => Sizes {
            style: 140067,
            table: 36084,
            inputs: 48926,
        },
        (23, 42) => Sizes {
            style: 143494,
            table: 36967,
            inputs: 50122,
        },
        (23, 43) => Sizes {
            style: 146921,
            table: 37850,
            inputs: 51318,
        },
        (23, 44) => Sizes {
            style: 150396,
            table: 38745,
            inputs: 52526,
        },
        (23, 45) => Sizes {
            style: 153915,
            table: 39651,
            inputs: 53745,
        },
        (23, 46) => Sizes {
            style: 157434,
            table: 40557,
            inputs: 54964,
        },
        (23, 47) => Sizes {
            style: 160953,
            table: 41463,
            inputs: 56183,
        },
        (23, 48) => Sizes {
            style: 164472,
            table: 42369,
            inputs: 57402,
        },
        (23, 49) => Sizes {
            style: 167991,
            table: 43275,
            inputs: 58621,
        },
        (23, 50) => Sizes {
            style: 171510,
            table: 44181,
            inputs: 59840,
        },
        (24, 2) => Sizes {
            style: 6920,
            table: 1775,
            inputs: 2438,
        },
        (24, 3) => Sizes {
            style: 10400,
            table: 2672,
            inputs: 3662,
        },
        (24, 4) => Sizes {
            style: 13880,
            table: 3569,
            inputs: 4886,
        },
        (24, 5) => Sizes {
            style: 17440,
            table: 4486,
            inputs: 6130,
        },
        (24, 6) => Sizes {
            style: 21016,
            table: 5407,
            inputs: 7378,
        },
        (24, 7) => Sizes {
            style: 24592,
            table: 6328,
            inputs: 8626,
        },
        (24, 8) => Sizes {
            style: 28168,
            table: 7249,
            inputs: 9874,
        },
        (24, 9) => Sizes {
            style: 31744,
            table: 8170,
            inputs: 11122,
        },
        (24, 10) => Sizes {
            style: 35320,
            table: 9091,
            inputs: 12370,
        },
        (24, 11) => Sizes {
            style: 38896,
            table: 10012,
            inputs: 13618,
        },
        (24, 12) => Sizes {
            style: 42472,
            table: 10933,
            inputs: 14866,
        },
        (24, 13) => Sizes {
            style: 46048,
            table: 11854,
            inputs: 16114,
        },
        (24, 14) => Sizes {
            style: 49624,
            table: 12775,
            inputs: 17362,
        },
        (24, 15) => Sizes {
            style: 53200,
            table: 13696,
            inputs: 18610,
        },
        (24, 16) => Sizes {
            style: 56776,
            table: 14617,
            inputs: 19858,
        },
        (24, 17) => Sizes {
            style: 60352,
            table: 15538,
            inputs: 21106,
        },
        (24, 18) => Sizes {
            style: 63928,
            table: 16459,
            inputs: 22354,
        },
        (24, 19) => Sizes {
            style: 67504,
            table: 17380,
            inputs: 23602,
        },
        (24, 20) => Sizes {
            style: 71080,
            table: 18301,
            inputs: 24850,
        },
        (24, 21) => Sizes {
            style: 74656,
            table: 19222,
            inputs: 26098,
        },
        (24, 22) => Sizes {
            style: 78232,
            table: 20143,
            inputs: 27346,
        },
        (24, 23) => Sizes {
            style: 81808,
            table: 21064,
            inputs: 28594,
        },
        (24, 24) => Sizes {
            style: 85384,
            table: 21985,
            inputs: 29842,
        },
        (24, 25) => Sizes {
            style: 88960,
            table: 22906,
            inputs: 31090,
        },
        (24, 26) => Sizes {
            style: 92536,
            table: 23827,
            inputs: 32338,
        },
        (24, 27) => Sizes {
            style: 96112,
            table: 24748,
            inputs: 33586,
        },
        (24, 28) => Sizes {
            style: 99688,
            table: 25669,
            inputs: 34834,
        },
        (24, 29) => Sizes {
            style: 103264,
            table: 26590,
            inputs: 36082,
        },
        (24, 30) => Sizes {
            style: 106840,
            table: 27511,
            inputs: 37330,
        },
        (24, 31) => Sizes {
            style: 110416,
            table: 28432,
            inputs: 38578,
        },
        (24, 32) => Sizes {
            style: 113992,
            table: 29353,
            inputs: 39826,
        },
        (24, 33) => Sizes {
            style: 117568,
            table: 30274,
            inputs: 41074,
        },
        (24, 34) => Sizes {
            style: 121144,
            table: 31195,
            inputs: 42322,
        },
        (24, 35) => Sizes {
            style: 124720,
            table: 32116,
            inputs: 43570,
        },
        (24, 36) => Sizes {
            style: 128296,
            table: 33037,
            inputs: 44818,
        },
        (24, 37) => Sizes {
            style: 131872,
            table: 33958,
            inputs: 46066,
        },
        (24, 38) => Sizes {
            style: 135448,
            table: 34879,
            inputs: 47314,
        },
        (24, 39) => Sizes {
            style: 139024,
            table: 35800,
            inputs: 48562,
        },
        (24, 40) => Sizes {
            style: 142600,
            table: 36721,
            inputs: 49810,
        },
        (24, 41) => Sizes {
            style: 146176,
            table: 37642,
            inputs: 51058,
        },
        (24, 42) => Sizes {
            style: 149784,
            table: 38571,
            inputs: 52314,
        },
        (24, 43) => Sizes {
            style: 153456,
            table: 39516,
            inputs: 53586,
        },
        (24, 44) => Sizes {
            style: 157128,
            table: 40461,
            inputs: 54858,
        },
        (24, 45) => Sizes {
            style: 160800,
            table: 41406,
            inputs: 56130,
        },
        (24, 46) => Sizes {
            style: 164472,
            table: 42351,
            inputs: 57402,
        },
        (24, 47) => Sizes {
            style: 168144,
            table: 43296,
            inputs: 58674,
        },
        (24, 48) => Sizes {
            style: 171816,
            table: 44241,
            inputs: 59946,
        },
        (24, 49) => Sizes {
            style: 175488,
            table: 45186,
            inputs: 61218,
        },
        (24, 50) => Sizes {
            style: 179160,
            table: 46131,
            inputs: 62490,
        },
        (25, 2) => Sizes {
            style: 7210,
            table: 1849,
            inputs: 2540,
        },
        (25, 3) => Sizes {
            style: 10835,
            table: 2783,
            inputs: 3815,
        },
        (25, 4) => Sizes {
            style: 14460,
            table: 3717,
            inputs: 5090,
        },
        (25, 5) => Sizes {
            style: 18185,
            table: 4676,
            inputs: 6390,
        },
        (25, 6) => Sizes {
            style: 21910,
            table: 5635,
            inputs: 7690,
        },
        (25, 7) => Sizes {
            style: 25635,
            table: 6594,
            inputs: 8990,
        },
        (25, 8) => Sizes {
            style: 29360,
            table: 7553,
            inputs: 10290,
        },
        (25, 9) => Sizes {
            style: 33085,
            table: 8512,
            inputs: 11590,
        },
        (25, 10) => Sizes {
            style: 36810,
            table: 9471,
            inputs: 12890,
        },
        (25, 11) => Sizes {
            style: 40535,
            table: 10430,
            inputs: 14190,
        },
        (25, 12) => Sizes {
            style: 44260,
            table: 11389,
            inputs: 15490,
        },
        (25, 13) => Sizes {
            style: 47985,
            table: 12348,
            inputs: 16790,
        },
        (25, 14) => Sizes {
            style: 51710,
            table: 13307,
            inputs: 18090,
        },
        (25, 15) => Sizes {
            style: 55435,
            table: 14266,
            inputs: 19390,
        },
        (25, 16) => Sizes {
            style: 59160,
            table: 15225,
            inputs: 20690,
        },
        (25, 17) => Sizes {
            style: 62885,
            table: 16184,
            inputs: 21990,
        },
        (25, 18) => Sizes {
            style: 66610,
            table: 17143,
            inputs: 23290,
        },
        (25, 19) => Sizes {
            style: 70335,
            table: 18102,
            inputs: 24590,
        },
        (25, 20) => Sizes {
            style: 74060,
            table: 19061,
            inputs: 25890,
        },
        (25, 21) => Sizes {
            style: 77785,
            table: 20020,
            inputs: 27190,
        },
        (25, 22) => Sizes {
            style: 81510,
            table: 20979,
            inputs: 28490,
        },
        (25, 23) => Sizes {
            style: 85235,
            table: 21938,
            inputs: 29790,
        },
        (25, 24) => Sizes {
            style: 88960,
            table: 22897,
            inputs: 31090,
        },
        (25, 25) => Sizes {
            style: 92685,
            table: 23856,
            inputs: 32390,
        },
        (25, 26) => Sizes {
            style: 96410,
            table: 24815,
            inputs: 33690,
        },
        (25, 27) => Sizes {
            style: 100135,
            table: 25774,
            inputs: 34990,
        },
        (25, 28) => Sizes {
            style: 103860,
            table: 26733,
            inputs: 36290,
        },
        (25, 29) => Sizes {
            style: 107585,
            table: 27692,
            inputs: 37590,
        },
        (25, 30) => Sizes {
            style: 111310,
            table: 28651,
            inputs: 38890,
        },
        (25, 31) => Sizes {
            style: 115035,
            table: 29610,
            inputs: 40190,
        },
        (25, 32) => Sizes {
            style: 118760,
            table: 30569,
            inputs: 41490,
        },
        (25, 33) => Sizes {
            style: 122485,
            table: 31528,
            inputs: 42790,
        },
        (25, 34) => Sizes {
            style: 126210,
            table: 32487,
            inputs: 44090,
        },
        (25, 35) => Sizes {
            style: 129935,
            table: 33446,
            inputs: 45390,
        },
        (25, 36) => Sizes {
            style: 133660,
            table: 34405,
            inputs: 46690,
        },
        (25, 37) => Sizes {
            style: 137385,
            table: 35364,
            inputs: 47990,
        },
        (25, 38) => Sizes {
            style: 141110,
            table: 36323,
            inputs: 49290,
        },
        (25, 39) => Sizes {
            style: 144835,
            table: 37282,
            inputs: 50590,
        },
        (25, 40) => Sizes {
            style: 148560,
            table: 38241,
            inputs: 51890,
        },
        (25, 41) => Sizes {
            style: 152385,
            table: 39225,
            inputs: 53215,
        },
        (25, 42) => Sizes {
            style: 156210,
            table: 40209,
            inputs: 54540,
        },
        (25, 43) => Sizes {
            style: 160035,
            table: 41193,
            inputs: 55865,
        },
        (25, 44) => Sizes {
            style: 163860,
            table: 42177,
            inputs: 57190,
        },
        (25, 45) => Sizes {
            style: 167685,
            table: 43161,
            inputs: 58515,
        },
        (25, 46) => Sizes {
            style: 171510,
            table: 44145,
            inputs: 59840,
        },
        (25, 47) => Sizes {
            style: 175335,
            table: 45129,
            inputs: 61165,
        },
        (25, 48) => Sizes {
            style: 179160,
            table: 46113,
            inputs: 62490,
        },
        (25, 49) => Sizes {
            style: 182985,
            table: 47097,
            inputs: 63815,
        },
        (25, 50) => Sizes {
            style: 186810,
            table: 48081,
            inputs: 65140,
        },
        (26, 2) => Sizes {
            style: 7500,
            table: 1923,
            inputs: 2642,
        },
        (26, 3) => Sizes {
            style: 11270,
            table: 2894,
            inputs: 3968,
        },
        (26, 4) => Sizes {
            style: 15056,
            table: 3869,
            inputs: 5298,
        },
        (26, 5) => Sizes {
            style: 18930,
            table: 4866,
            inputs: 6650,
        },
        (26, 6) => Sizes {
            style: 22804,
            table: 5863,
            inputs: 8002,
        },
        (26, 7) => Sizes {
            style: 26678,
            table: 6860,
            inputs: 9354,
        },
        (26, 8) => Sizes {
            style: 30552,
            table: 7857,
            inputs: 10706,
        },
        (26, 9) => Sizes {
            style: 34426,
            table: 8854,
            inputs: 12058,
        },
        (26, 10) => Sizes {
            style: 38300,
            table: 9851,
            inputs: 13410,
        },
        (26, 11) => Sizes {
            style: 42174,
            table: 10848,
            inputs: 14762,
        },
        (26, 12) => Sizes {
            style: 46048,
            table: 11845,
            inputs: 16114,
        },
        (26, 13) => Sizes {
            style: 49922,
            table: 12842,
            inputs: 17466,
        },
        (26, 14) => Sizes {
            style: 53796,
            table: 13839,
            inputs: 18818,
        },
        (26, 15) => Sizes {
            style: 57670,
            table: 14836,
            inputs: 20170,
        },
        (26, 16) => Sizes {
            style: 61544,
            table: 15833,
            inputs: 21522,
        },
        (26, 17) => Sizes {
            style: 65418,
            table: 16830,
            inputs: 22874,
        },
        (26, 18) => Sizes {
            style: 69292,
            table: 17827,
            inputs: 24226,
        },
        (26, 19) => Sizes {
            style: 73166,
            table: 18824,
            inputs: 25578,
        },
        (26, 20) => Sizes {
            style: 77040,
            table: 19821,
            inputs: 26930,
        },
        (26, 21) => Sizes {
            style: 80914,
            table: 20818,
            inputs: 28282,
        },
        (26, 22) => Sizes {
            style: 84788,
            table: 21815,
            inputs: 29634,
        },
        (26, 23) => Sizes {
            style: 88662,
            table: 22812,
            inputs: 30986,
        },
        (26, 24) => Sizes {
            style: 92536,
            table: 23809,
            inputs: 32338,
        },
        (26, 25) => Sizes {
            style: 96410,
            table: 24806,
            inputs: 33690,
        },
        (26, 26) => Sizes {
            style: 100284,
            table: 25803,
            inputs: 35042,
        },
        (26, 27) => Sizes {
            style: 104158,
            table: 26800,
            inputs: 36394,
        },
        (26, 28) => Sizes {
            style: 108032,
            table: 27797,
            inputs: 37746,
        },
        (26, 29) => Sizes {
            style: 111906,
            table: 28794,
            inputs: 39098,
        },
        (26, 30) => Sizes {
            style: 115780,
            table: 29791,
            inputs: 40450,
        },
        (26, 31) => Sizes {
            style: 119654,
            table: 30788,
            inputs: 41802,
        },
        (26, 32) => Sizes {
            style: 123528,
            table: 31785,
            inputs: 43154,
        },
        (26, 33) => Sizes {
            style: 127402,
            table: 32782,
            inputs: 44506,
        },
        (26, 34) => Sizes {
            style: 131276,
            table: 33779,
            inputs: 45858,
        },
        (26, 35) => Sizes {
            style: 135150,
            table: 34776,
            inputs: 47210,
        },
        (26, 36) => Sizes {
            style: 139024,
            table: 35773,
            inputs: 48562,
        },
        (26, 37) => Sizes {
            style: 142898,
            table: 36770,
            inputs: 49914,
        },
        (26, 38) => Sizes {
            style: 146772,
            table: 37767,
            inputs: 51266,
        },
        (26, 39) => Sizes {
            style: 150702,
            table: 38778,
            inputs: 52632,
        },
        (26, 40) => Sizes {
            style: 154680,
            table: 39801,
            inputs: 54010,
        },
        (26, 41) => Sizes {
            style: 158658,
            table: 40824,
            inputs: 55388,
        },
        (26, 42) => Sizes {
            style: 162636,
            table: 41847,
            inputs: 56766,
        },
        (26, 43) => Sizes {
            style: 166614,
            table: 42870,
            inputs: 58144,
        },
        (26, 44) => Sizes {
            style: 170592,
            table: 43893,
            inputs: 59522,
        },
        (26, 45) => Sizes {
            style: 174570,
            table: 44916,
            inputs: 60900,
        },
        (26, 46) => Sizes {
            style: 178548,
            table: 45939,
            inputs: 62278,
        },
        (26, 47) => Sizes {
            style: 182526,
            table: 46962,
            inputs: 63656,
        },
        (26, 48) => Sizes {
            style: 186504,
            table: 47985,
            inputs: 65034,
        },
        (26, 49) => Sizes {
            style: 190482,
            table: 49008,
            inputs: 66412,
        },
        (26, 50) => Sizes {
            style: 194460,
            table: 50031,
            inputs: 67790,
        },
        (27, 2) => Sizes {
            style: 7790,
            table: 1997,
            inputs: 2744,
        },
        (27, 3) => Sizes {
            style: 11705,
            table: 3005,
            inputs: 4121,
        },
        (27, 4) => Sizes {
            style: 15652,
            table: 4021,
            inputs: 5506,
        },
        (27, 5) => Sizes {
            style: 19675,
            table: 5056,
            inputs: 6910,
        },
        (27, 6) => Sizes {
            style: 23698,
            table: 6091,
            inputs: 8314,
        },
        (27, 7) => Sizes {
            style: 27721,
            table: 7126,
            inputs: 9718,
        },
        (27, 8) => Sizes {
            style: 31744,
            table: 8161,
            inputs: 11122,
        },
        (27, 9) => Sizes {
            style: 35767,
            table: 9196,
            inputs: 12526,
        },
        (27, 10) => Sizes {
            style: 39790,
            table: 10231,
            inputs: 13930,
        },
        (27, 11) => Sizes {
            style: 43813,
            table: 11266,
            inputs: 15334,
        },
        (27, 12) => Sizes {
            style: 47836,
            table: 12301,
            inputs: 16738,
        },
        (27, 13) => Sizes {
            style: 51859,
            table: 13336,
            inputs: 18142,
        },
        (27, 14) => Sizes {
            style: 55882,
            table: 14371,
            inputs: 19546,
        },
        (27, 15) => Sizes {
            style: 59905,
            table: 15406,
            inputs: 20950,
        },
        (27, 16) => Sizes {
            style: 63928,
            table: 16441,
            inputs: 22354,
        },
        (27, 17) => Sizes {
            style: 67951,
            table: 17476,
            inputs: 23758,
        },
        (27, 18) => Sizes {
            style: 71974,
            table: 18511,
            inputs: 25162,
        },
        (27, 19) => Sizes {
            style: 75997,
            table: 19546,
            inputs: 26566,
        },
        (27, 20) => Sizes {
            style: 80020,
            table: 20581,
            inputs: 27970,
        },
        (27, 21) => Sizes {
            style: 84043,
            table: 21616,
            inputs: 29374,
        },
        (27, 22) => Sizes {
            style: 88066,
            table: 22651,
            inputs: 30778,
        },
        (27, 23) => Sizes {
            style: 92089,
            table: 23686,
            inputs: 32182,
        },
        (27, 24) => Sizes {
            style: 96112,
            table: 24721,
            inputs: 33586,
        },
        (27, 25) => Sizes {
            style: 100135,
            table: 25756,
            inputs: 34990,
        },
        (27, 26) => Sizes {
            style: 104158,
            table: 26791,
            inputs: 36394,
        },
        (27, 27) => Sizes {
            style: 108181,
            table: 27826,
            inputs: 37798,
        },
        (27, 28) => Sizes {
            style: 112204,
            table: 28861,
            inputs: 39202,
        },
        (27, 29) => Sizes {
            style: 116227,
            table: 29896,
            inputs: 40606,
        },
        (27, 30) => Sizes {
            style: 120250,
            table: 30931,
            inputs: 42010,
        },
        (27, 31) => Sizes {
            style: 124273,
            table: 31966,
            inputs: 43414,
        },
        (27, 32) => Sizes {
            style: 128296,
            table: 33001,
            inputs: 44818,
        },
        (27, 33) => Sizes {
            style: 132319,
            table: 34036,
            inputs: 46222,
        },
        (27, 34) => Sizes {
            style: 136342,
            table: 35071,
            inputs: 47626,
        },
        (27, 35) => Sizes {
            style: 140365,
            table: 36106,
            inputs: 49030,
        },
        (27, 36) => Sizes {
            style: 144388,
            table: 37141,
            inputs: 50434,
        },
        (27, 37) => Sizes {
            style: 148411,
            table: 38176,
            inputs: 51838,
        },
        (27, 38) => Sizes {
            style: 152538,
            table: 39237,
            inputs: 53268,
        },
        (27, 39) => Sizes {
            style: 156669,
            table: 40299,
            inputs: 54699,
        },
        (27, 40) => Sizes {
            style: 160800,
            table: 41361,
            inputs: 56130,
        },
        (27, 41) => Sizes {
            style: 164931,
            table: 42423,
            inputs: 57561,
        },
        (27, 42) => Sizes {
            style: 169062,
            table: 43485,
            inputs: 58992,
        },
        (27, 43) => Sizes {
            style: 173193,
            table: 44547,
            inputs: 60423,
        },
        (27, 44) => Sizes {
            style: 177324,
            table: 45609,
            inputs: 61854,
        },
        (27, 45) => Sizes {
            style: 181455,
            table: 46671,
            inputs: 63285,
        },
        (27, 46) => Sizes {
            style: 185586,
            table: 47733,
            inputs: 64716,
        },
        (27, 47) => Sizes {
            style: 189717,
            table: 48795,
            inputs: 66147,
        },
        (27, 48) => Sizes {
            style: 193848,
            table: 49857,
            inputs: 67578,
        },
        (27, 49) => Sizes {
            style: 197979,
            table: 50919,
            inputs: 69009,
        },
        (27, 50) => Sizes {
            style: 202110,
            table: 51981,
            inputs: 70440,
        },
        (28, 2) => Sizes {
            style: 8080,
            table: 2071,
            inputs: 2846,
        },
        (28, 3) => Sizes {
            style: 12140,
            table: 3116,
            inputs: 4274,
        },
        (28, 4) => Sizes {
            style: 16248,
            table: 4173,
            inputs: 5714,
        },
        (28, 5) => Sizes {
            style: 20420,
            table: 5246,
            inputs: 7170,
        },
        (28, 6) => Sizes {
            style: 24592,
            table: 6319,
            inputs: 8626,
        },
        (28, 7) => Sizes {
            style: 28764,
            table: 7392,
            inputs: 10082,
        },
        (28, 8) => Sizes {
            style: 32936,
            table: 8465,
            inputs: 11538,
        },
        (28, 9) => Sizes {
            style: 37108,
            table: 9538,
            inputs: 12994,
        },
        (28, 10) => Sizes {
            style: 41280,
            table: 10611,
            inputs: 14450,
        },
        (28, 11) => Sizes {
            style: 45452,
            table: 11684,
            inputs: 15906,
        },
        (28, 12) => Sizes {
            style: 49624,
            table: 12757,
            inputs: 17362,
        },
        (28, 13) => Sizes {
            style: 53796,
            table: 13830,
            inputs: 18818,
        },
        (28, 14) => Sizes {
            style: 57968,
            table: 14903,
            inputs: 20274,
        },
        (28, 15) => Sizes {
            style: 62140,
            table: 15976,
            inputs: 21730,
        },
        (28, 16) => Sizes {
            style: 66312,
            table: 17049,
            inputs: 23186,
        },
        (28, 17) => Sizes {
            style: 70484,
            table: 18122,
            inputs: 24642,
        },
        (28, 18) => Sizes {
            style: 74656,
            table: 19195,
            inputs: 26098,
        },
        (28, 19) => Sizes {
            style: 78828,
            table: 20268,
            inputs: 27554,
        },
        (28, 20) => Sizes {
            style: 83000,
            table: 21341,
            inputs: 29010,
        },
        (28, 21) => Sizes {
            style: 87172,
            table: 22414,
            inputs: 30466,
        },
        (28, 22) => Sizes {
            style: 91344,
            table: 23487,
            inputs: 31922,
        },
        (28, 23) => Sizes {
            style: 95516,
            table: 24560,
            inputs: 33378,
        },
        (28, 24) => Sizes {
            style: 99688,
            table: 25633,
            inputs: 34834,
        },
        (28, 25) => Sizes {
            style: 103860,
            table: 26706,
            inputs: 36290,
        },
        (28, 26) => Sizes {
            style: 108032,
            table: 27779,
            inputs: 37746,
        },
        (28, 27) => Sizes {
            style: 112204,
            table: 28852,
            inputs: 39202,
        },
        (28, 28) => Sizes {
            style: 116376,
            table: 29925,
            inputs: 40658,
        },
        (28, 29) => Sizes {
            style: 120548,
            table: 30998,
            inputs: 42114,
        },
        (28, 30) => Sizes {
            style: 124720,
            table: 32071,
            inputs: 43570,
        },
        (28, 31) => Sizes {
            style: 128892,
            table: 33144,
            inputs: 45026,
        },
        (28, 32) => Sizes {
            style: 133064,
            table: 34217,
            inputs: 46482,
        },
        (28, 33) => Sizes {
            style: 137236,
            table: 35290,
            inputs: 47938,
        },
        (28, 34) => Sizes {
            style: 141408,
            table: 36363,
            inputs: 49394,
        },
        (28, 35) => Sizes {
            style: 145580,
            table: 37436,
            inputs: 50850,
        },
        (28, 36) => Sizes {
            style: 149784,
            table: 38517,
            inputs: 52314,
        },
        (28, 37) => Sizes {
            style: 154068,
            table: 39618,
            inputs: 53798,
        },
        (28, 38) => Sizes {
            style: 158352,
            table: 40719,
            inputs: 55282,
        },
        (28, 39) => Sizes {
            style: 162636,
            table: 41820,
            inputs: 56766,
        },
        (28, 40) => Sizes {
            style: 166920,
            table: 42921,
            inputs: 58250,
        },
        (28, 41) => Sizes {
            style: 171204,
            table: 44022,
            inputs: 59734,
        },
        (28, 42) => Sizes {
            style: 175488,
            table: 45123,
            inputs: 61218,
        },
        (28, 43) => Sizes {
            style: 179772,
            table: 46224,
            inputs: 62702,
        },
        (28, 44) => Sizes {
            style: 184056,
            table: 47325,
            inputs: 64186,
        },
        (28, 45) => Sizes {
            style: 188340,
            table: 48426,
            inputs: 65670,
        },
        (28, 46) => Sizes {
            style: 192624,
            table: 49527,
            inputs: 67154,
        },
        (28, 47) => Sizes {
            style: 196908,
            table: 50628,
            inputs: 68638,
        },
        (28, 48) => Sizes {
            style: 201192,
            table: 51729,
            inputs: 70122,
        },
        (28, 49) => Sizes {
            style: 205476,
            table: 52830,
            inputs: 71606,
        },
        (28, 50) => Sizes {
            style: 209760,
            table: 53931,
            inputs: 73090,
        },
        (29, 2) => Sizes {
            style: 8370,
            table: 2145,
            inputs: 2948,
        },
        (29, 3) => Sizes {
            style: 12575,
            table: 3227,
            inputs: 4427,
        },
        (29, 4) => Sizes {
            style: 16844,
            table: 4325,
            inputs: 5922,
        },
        (29, 5) => Sizes {
            style: 21165,
            table: 5436,
            inputs: 7430,
        },
        (29, 6) => Sizes {
            style: 25486,
            table: 6547,
            inputs: 8938,
        },
        (29, 7) => Sizes {
            style: 29807,
            table: 7658,
            inputs: 10446,
        },
        (29, 8) => Sizes {
            style: 34128,
            table: 8769,
            inputs: 11954,
        },
        (29, 9) => Sizes {
            style: 38449,
            table: 9880,
            inputs: 13462,
        },
        (29, 10) => Sizes {
            style: 42770,
            table: 10991,
            inputs: 14970,
        },
        (29, 11) => Sizes {
            style: 47091,
            table: 12102,
            inputs: 16478,
        },
        (29, 12) => Sizes {
            style: 51412,
            table: 13213,
            inputs: 17986,
        },
        (29, 13) => Sizes {
            style: 55733,
            table: 14324,
            inputs: 19494,
        },
        (29, 14) => Sizes {
            style: 60054,
            table: 15435,
            inputs: 21002,
        },
        (29, 15) => Sizes {
            style: 64375,
            table: 16546,
            inputs: 22510,
        },
        (29, 16) => Sizes {
            style: 68696,
            table: 17657,
            inputs: 24018,
        },
        (29, 17) => Sizes {
            style: 73017,
            table: 18768,
            inputs: 25526,
        },
        (29, 18) => Sizes {
            style: 77338,
            table: 19879,
            inputs: 27034,
        },
        (29, 19) => Sizes {
            style: 81659,
            table: 20990,
            inputs: 28542,
        },
        (29, 20) => Sizes {
            style: 85980,
            table: 22101,
            inputs: 30050,
        },
        (29, 21) => Sizes {
            style: 90301,
            table: 23212,
            inputs: 31558,
        },
        (29, 22) => Sizes {
            style: 94622,
            table: 24323,
            inputs: 33066,
        },
        (29, 23) => Sizes {
            style: 98943,
            table: 25434,
            inputs: 34574,
        },
        (29, 24) => Sizes {
            style: 103264,
            table: 26545,
            inputs: 36082,
        },
        (29, 25) => Sizes {
            style: 107585,
            table: 27656,
            inputs: 37590,
        },
        (29, 26) => Sizes {
            style: 111906,
            table: 28767,
            inputs: 39098,
        },
        (29, 27) => Sizes {
            style: 116227,
            table: 29878,
            inputs: 40606,
        },
        (29, 28) => Sizes {
            style: 120548,
            table: 30989,
            inputs: 42114,
        },
        (29, 29) => Sizes {
            style: 124869,
            table: 32100,
            inputs: 43622,
        },
        (29, 30) => Sizes {
            style: 129190,
            table: 33211,
            inputs: 45130,
        },
        (29, 31) => Sizes {
            style: 133511,
            table: 34322,
            inputs: 46638,
        },
        (29, 32) => Sizes {
            style: 137832,
            table: 35433,
            inputs: 48146,
        },
        (29, 33) => Sizes {
            style: 142153,
            table: 36544,
            inputs: 49654,
        },
        (29, 34) => Sizes {
            style: 146474,
            table: 37655,
            inputs: 51162,
        },
        (29, 35) => Sizes {
            style: 150855,
            table: 38781,
            inputs: 52685,
        },
        (29, 36) => Sizes {
            style: 155292,
            table: 39921,
            inputs: 54222,
        },
        (29, 37) => Sizes {
            style: 159729,
            table: 41061,
            inputs: 55759,
        },
        (29, 38) => Sizes {
            style: 164166,
            table: 42201,
            inputs: 57296,
        },
        (29, 39) => Sizes {
            style: 168603,
            table: 43341,
            inputs: 58833,
        },
        (29, 40) => Sizes {
            style: 173040,
            table: 44481,
            inputs: 60370,
        },
        (29, 41) => Sizes {
            style: 177477,
            table: 45621,
            inputs: 61907,
        },
        (29, 42) => Sizes {
            style: 181914,
            table: 46761,
            inputs: 63444,
        },
        (29, 43) => Sizes {
            style: 186351,
            table: 47901,
            inputs: 64981,
        },
        (29, 44) => Sizes {
            style: 190788,
            table: 49041,
            inputs: 66518,
        },
        (29, 45) => Sizes {
            style: 195225,
            table: 50181,
            inputs: 68055,
        },
        (29, 46) => Sizes {
            style: 199662,
            table: 51321,
            inputs: 69592,
        },
        (29, 47) => Sizes {
            style: 204099,
            table: 52461,
            inputs: 71129,
        },
        (29, 48) => Sizes {
            style: 208536,
            table: 53601,
            inputs: 72666,
        },
        (29, 49) => Sizes {
            style: 212973,
            table: 54741,
            inputs: 74203,
        },
        (29, 50) => Sizes {
            style: 217410,
            table: 55881,
            inputs: 75740,
        },
        (30, 2) => Sizes {
            style: 8660,
            table: 2219,
            inputs: 3050,
        },
        (30, 3) => Sizes {
            style: 13010,
            table: 3338,
            inputs: 4580,
        },
        (30, 4) => Sizes {
            style: 17440,
            table: 4477,
            inputs: 6130,
        },
        (30, 5) => Sizes {
            style: 21910,
            table: 5626,
            inputs: 7690,
        },
        (30, 6) => Sizes {
            style: 26380,
            table: 6775,
            inputs: 9250,
        },
        (30, 7) => Sizes {
            style: 30850,
            table: 7924,
            inputs: 10810,
        },
        (30, 8) => Sizes {
            style: 35320,
            table: 9073,
            inputs: 12370,
        },
        (30, 9) => Sizes {
            style: 39790,
            table: 10222,
            inputs: 13930,
        },
        (30, 10) => Sizes {
            style: 44260,
            table: 11371,
            inputs: 15490,
        },
        (30, 11) => Sizes {
            style: 48730,
            table: 12520,
            inputs: 17050,
        },
        (30, 12) => Sizes {
            style: 53200,
            table: 13669,
            inputs: 18610,
        },
        (30, 13) => Sizes {
            style: 57670,
            table: 14818,
            inputs: 20170,
        },
        (30, 14) => Sizes {
            style: 62140,
            table: 15967,
            inputs: 21730,
        },
        (30, 15) => Sizes {
            style: 66610,
            table: 17116,
            inputs: 23290,
        },
        (30, 16) => Sizes {
            style: 71080,
            table: 18265,
            inputs: 24850,
        },
        (30, 17) => Sizes {
            style: 75550,
            table: 19414,
            inputs: 26410,
        },
        (30, 18) => Sizes {
            style: 80020,
            table: 20563,
            inputs: 27970,
        },
        (30, 19) => Sizes {
            style: 84490,
            table: 21712,
            inputs: 29530,
        },
        (30, 20) => Sizes {
            style: 88960,
            table: 22861,
            inputs: 31090,
        },
        (30, 21) => Sizes {
            style: 93430,
            table: 24010,
            inputs: 32650,
        },
        (30, 22) => Sizes {
            style: 97900,
            table: 25159,
            inputs: 34210,
        },
        (30, 23) => Sizes {
            style: 102370,
            table: 26308,
            inputs: 35770,
        },
        (30, 24) => Sizes {
            style: 106840,
            table: 27457,
            inputs: 37330,
        },
        (30, 25) => Sizes {
            style: 111310,
            table: 28606,
            inputs: 38890,
        },
        (30, 26) => Sizes {
            style: 115780,
            table: 29755,
            inputs: 40450,
        },
        (30, 27) => Sizes {
            style: 120250,
            table: 30904,
            inputs: 42010,
        },
        (30, 28) => Sizes {
            style: 124720,
            table: 32053,
            inputs: 43570,
        },
        (30, 29) => Sizes {
            style: 129190,
            table: 33202,
            inputs: 45130,
        },
        (30, 30) => Sizes {
            style: 133660,
            table: 34351,
            inputs: 46690,
        },
        (30, 31) => Sizes {
            style: 138130,
            table: 35500,
            inputs: 48250,
        },
        (30, 32) => Sizes {
            style: 142600,
            table: 36649,
            inputs: 49810,
        },
        (30, 33) => Sizes {
            style: 147070,
            table: 37798,
            inputs: 51370,
        },
        (30, 34) => Sizes {
            style: 151620,
            table: 38967,
            inputs: 52950,
        },
        (30, 35) => Sizes {
            style: 156210,
            table: 40146,
            inputs: 54540,
        },
        (30, 36) => Sizes {
            style: 160800,
            table: 41325,
            inputs: 56130,
        },
        (30, 37) => Sizes {
            style: 165390,
            table: 42504,
            inputs: 57720,
        },
        (30, 38) => Sizes {
            style: 169980,
            table: 43683,
            inputs: 59310,
        },
        (30, 39) => Sizes {
            style: 174570,
            table: 44862,
            inputs: 60900,
        },
        (30, 40) => Sizes {
            style: 179160,
            table: 46041,
            inputs: 62490,
        },
        (30, 41) => Sizes {
            style: 183750,
            table: 47220,
            inputs: 64080,
        },
        (30, 42) => Sizes {
            style: 188340,
            table: 48399,
            inputs: 65670,
        },
        (30, 43) => Sizes {
            style: 192930,
            table: 49578,
            inputs: 67260,
        },
        (30, 44) => Sizes {
            style: 197520,
            table: 50757,
            inputs: 68850,
        },
        (30, 45) => Sizes {
            style: 202110,
            table: 51936,
            inputs: 70440,
        },
        (30, 46) => Sizes {
            style: 206700,
            table: 53115,
            inputs: 72030,
        },
        (30, 47) => Sizes {
            style: 211290,
            table: 54294,
            inputs: 73620,
        },
        (30, 48) => Sizes {
            style: 215880,
            table: 55473,
            inputs: 75210,
        },
        (30, 49) => Sizes {
            style: 220470,
            table: 56652,
            inputs: 76800,
        },
        (30, 50) => Sizes {
            style: 225060,
            table: 57831,
            inputs: 78390,
        },
        (31, 2) => Sizes {
            style: 8950,
            table: 2293,
            inputs: 3152,
        },
        (31, 3) => Sizes {
            style: 13445,
            table: 3449,
            inputs: 4733,
        },
        (31, 4) => Sizes {
            style: 18036,
            table: 4629,
            inputs: 6338,
        },
        (31, 5) => Sizes {
            style: 22655,
            table: 5816,
            inputs: 7950,
        },
        (31, 6) => Sizes {
            style: 27274,
            table: 7003,
            inputs: 9562,
        },
        (31, 7) => Sizes {
            style: 31893,
            table: 8190,
            inputs: 11174,
        },
        (31, 8) => Sizes {
            style: 36512,
            table: 9377,
            inputs: 12786,
        },
        (31, 9) => Sizes {
            style: 41131,
            table: 10564,
            inputs: 14398,
        },
        (31, 10) => Sizes {
            style: 45750,
            table: 11751,
            inputs: 16010,
        },
        (31, 11) => Sizes {
            style: 50369,
            table: 12938,
            inputs: 17622,
        },
        (31, 12) => Sizes {
            style: 54988,
            table: 14125,
            inputs: 19234,
        },
        (31, 13) => Sizes {
            style: 59607,
            table: 15312,
            inputs: 20846,
        },
        (31, 14) => Sizes {
            style: 64226,
            table: 16499,
            inputs: 22458,
        },
        (31, 15) => Sizes {
            style: 68845,
            table: 17686,
            inputs: 24070,
        },
        (31, 16) => Sizes {
            style: 73464,
            table: 18873,
            inputs: 25682,
        },
        (31, 17) => Sizes {
            style: 78083,
            table: 20060,
            inputs: 27294,
        },
        (31, 18) => Sizes {
            style: 82702,
            table: 21247,
            inputs: 28906,
        },
        (31, 19) => Sizes {
            style: 87321,
            table: 22434,
            inputs: 30518,
        },
        (31, 20) => Sizes {
            style: 91940,
            table: 23621,
            inputs: 32130,
        },
        (31, 21) => Sizes {
            style: 96559,
            table: 24808,
            inputs: 33742,
        },
        (31, 22) => Sizes {
            style: 101178,
            table: 25995,
            inputs: 35354,
        },
        (31, 23) => Sizes {
            style: 105797,
            table: 27182,
            inputs: 36966,
        },
        (31, 24) => Sizes {
            style: 110416,
            table: 28369,
            inputs: 38578,
        },
        (31, 25) => Sizes {
            style: 115035,
            table: 29556,
            inputs: 40190,
        },
        (31, 26) => Sizes {
            style: 119654,
            table: 30743,
            inputs: 41802,
        },
        (31, 27) => Sizes {
            style: 124273,
            table: 31930,
            inputs: 43414,
        },
        (31, 28) => Sizes {
            style: 128892,
            table: 33117,
            inputs: 45026,
        },
        (31, 29) => Sizes {
            style: 133511,
            table: 34304,
            inputs: 46638,
        },
        (31, 30) => Sizes {
            style: 138130,
            table: 35491,
            inputs: 48250,
        },
        (31, 31) => Sizes {
            style: 142749,
            table: 36678,
            inputs: 49862,
        },
        (31, 32) => Sizes {
            style: 147368,
            table: 37865,
            inputs: 51474,
        },
        (31, 33) => Sizes {
            style: 152079,
            table: 39075,
            inputs: 53109,
        },
        (31, 34) => Sizes {
            style: 156822,
            table: 40293,
            inputs: 54752,
        },
        (31, 35) => Sizes {
            style: 161565,
            table: 41511,
            inputs: 56395,
        },
        (31, 36) => Sizes {
            style: 166308,
            table: 42729,
            inputs: 58038,
        },
        (31, 37) => Sizes {
            style: 171051,
            table: 43947,
            inputs: 59681,
        },
        (31, 38) => Sizes {
            style: 175794,
            table: 45165,
            inputs: 61324,
        },
        (31, 39) => Sizes {
            style: 180537,
            table: 46383,
            inputs: 62967,
        },
        (31, 40) => Sizes {
            style: 185280,
            table: 47601,
            inputs: 64610,
        },
        (31, 41) => Sizes {
            style: 190023,
            table: 48819,
            inputs: 66253,
        },
        (31, 42) => Sizes {
            style: 194766,
            table: 50037,
            inputs: 67896,
        },
        (31, 43) => Sizes {
            style: 199509,
            table: 51255,
            inputs: 69539,
        },
        (31, 44) => Sizes {
            style: 204252,
            table: 52473,
            inputs: 71182,
        },
        (31, 45) => Sizes {
            style: 208995,
            table: 53691,
            inputs: 72825,
        },
        (31, 46) => Sizes {
            style: 213738,
            table: 54909,
            inputs: 74468,
        },
        (31, 47) => Sizes {
            style: 218481,
            table: 56127,
            inputs: 76111,
        },
        (31, 48) => Sizes {
            style: 223224,
            table: 57345,
            inputs: 77754,
        },
        (31, 49) => Sizes {
            style: 227967,
            table: 58563,
            inputs: 79397,
        },
        (31, 50) => Sizes {
            style: 232710,
            table: 59781,
            inputs: 81040,
        },
        (32, 2) => Sizes {
            style: 9240,
            table: 2367,
            inputs: 3254,
        },
        (32, 3) => Sizes {
            style: 13880,
            table: 3560,
            inputs: 4886,
        },
        (32, 4) => Sizes {
            style: 18632,
            table: 4781,
            inputs: 6546,
        },
        (32, 5) => Sizes {
            style: 23400,
            table: 6006,
            inputs: 8210,
        },
        (32, 6) => Sizes {
            style: 28168,
            table: 7231,
            inputs: 9874,
        },
        (32, 7) => Sizes {
            style: 32936,
            table: 8456,
            inputs: 11538,
        },
        (32, 8) => Sizes {
            style: 37704,
            table: 9681,
            inputs: 13202,
        },
        (32, 9) => Sizes {
            style: 42472,
            table: 10906,
            inputs: 14866,
        },
        (32, 10) => Sizes {
            style: 47240,
            table: 12131,
            inputs: 16530,
        },
        (32, 11) => Sizes {
            style: 52008,
            table: 13356,
            inputs: 18194,
        },
        (32, 12) => Sizes {
            style: 56776,
            table: 14581,
            inputs: 19858,
        },
        (32, 13) => Sizes {
            style: 61544,
            table: 15806,
            inputs: 21522,
        },
        (32, 14) => Sizes {
            style: 66312,
            table: 17031,
            inputs: 23186,
        },
        (32, 15) => Sizes {
            style: 71080,
            table: 18256,
            inputs: 24850,
        },
        (32, 16) => Sizes {
            style: 75848,
            table: 19481,
            inputs: 26514,
        },
        (32, 17) => Sizes {
            style: 80616,
            table: 20706,
            inputs: 28178,
        },
        (32, 18) => Sizes {
            style: 85384,
            table: 21931,
            inputs: 29842,
        },
        (32, 19) => Sizes {
            style: 90152,
            table: 23156,
            inputs: 31506,
        },
        (32, 20) => Sizes {
            style: 94920,
            table: 24381,
            inputs: 33170,
        },
        (32, 21) => Sizes {
            style: 99688,
            table: 25606,
            inputs: 34834,
        },
        (32, 22) => Sizes {
            style: 104456,
            table: 26831,
            inputs: 36498,
        },
        (32, 23) => Sizes {
            style: 109224,
            table: 28056,
            inputs: 38162,
        },
        (32, 24) => Sizes {
            style: 113992,
            table: 29281,
            inputs: 39826,
        },
        (32, 25) => Sizes {
            style: 118760,
            table: 30506,
            inputs: 41490,
        },
        (32, 26) => Sizes {
            style: 123528,
            table: 31731,
            inputs: 43154,
        },
        (32, 27) => Sizes {
            style: 128296,
            table: 32956,
            inputs: 44818,
        },
        (32, 28) => Sizes {
            style: 133064,
            table: 34181,
            inputs: 46482,
        },
        (32, 29) => Sizes {
            style: 137832,
            table: 35406,
            inputs: 48146,
        },
        (32, 30) => Sizes {
            style: 142600,
            table: 36631,
            inputs: 49810,
        },
        (32, 31) => Sizes {
            style: 147368,
            table: 37856,
            inputs: 51474,
        },
        (32, 32) => Sizes {
            style: 152232,
            table: 39105,
            inputs: 53162,
        },
        (32, 33) => Sizes {
            style: 157128,
            table: 40362,
            inputs: 54858,
        },
        (32, 34) => Sizes {
            style: 162024,
            table: 41619,
            inputs: 56554,
        },
        (32, 35) => Sizes {
            style: 166920,
            table: 42876,
            inputs: 58250,
        },
        (32, 36) => Sizes {
            style: 171816,
            table: 44133,
            inputs: 59946,
        },
        (32, 37) => Sizes {
            style: 176712,
            table: 45390,
            inputs: 61642,
        },
        (32, 38) => Sizes {
            style: 181608,
            table: 46647,
            inputs: 63338,
        },
        (32, 39) => Sizes {
            style: 186504,
            table: 47904,
            inputs: 65034,
        },
        (32, 40) => Sizes {
            style: 191400,
            table: 49161,
            inputs: 66730,
        },
        (32, 41) => Sizes {
            style: 196296,
            table: 50418,
            inputs: 68426,
        },
        (32, 42) => Sizes {
            style: 201192,
            table: 51675,
            inputs: 70122,
        },
        (32, 43) => Sizes {
            style: 206088,
            table: 52932,
            inputs: 71818,
        },
        (32, 44) => Sizes {
            style: 210984,
            table: 54189,
            inputs: 73514,
        },
        (32, 45) => Sizes {
            style: 215880,
            table: 55446,
            inputs: 75210,
        },
        (32, 46) => Sizes {
            style: 220776,
            table: 56703,
            inputs: 76906,
        },
        (32, 47) => Sizes {
            style: 225672,
            table: 57960,
            inputs: 78602,
        },
        (32, 48) => Sizes {
            style: 230568,
            table: 59217,
            inputs: 80298,
        },
        (32, 49) => Sizes {
            style: 235464,
            table: 60474,
            inputs: 81994,
        },
        (32, 50) => Sizes {
            style: 240360,
            table: 61731,
            inputs: 83690,
        },
        (33, 2) => Sizes {
            style: 9530,
            table: 2441,
            inputs: 3356,
        },
        (33, 3) => Sizes {
            style: 14315,
            table: 3671,
            inputs: 5039,
        },
        (33, 4) => Sizes {
            style: 19228,
            table: 4933,
            inputs: 6754,
        },
        (33, 5) => Sizes {
            style: 24145,
            table: 6196,
            inputs: 8470,
        },
        (33, 6) => Sizes {
            style: 29062,
            table: 7459,
            inputs: 10186,
        },
        (33, 7) => Sizes {
            style: 33979,
            table: 8722,
            inputs: 11902,
        },
        (33, 8) => Sizes {
            style: 38896,
            table: 9985,
            inputs: 13618,
        },
        (33, 9) => Sizes {
            style: 43813,
            table: 11248,
            inputs: 15334,
        },
        (33, 10) => Sizes {
            style: 48730,
            table: 12511,
            inputs: 17050,
        },
        (33, 11) => Sizes {
            style: 53647,
            table: 13774,
            inputs: 18766,
        },
        (33, 12) => Sizes {
            style: 58564,
            table: 15037,
            inputs: 20482,
        },
        (33, 13) => Sizes {
            style: 63481,
            table: 16300,
            inputs: 22198,
        },
        (33, 14) => Sizes {
            style: 68398,
            table: 17563,
            inputs: 23914,
        },
        (33, 15) => Sizes {
            style: 73315,
            table: 18826,
            inputs: 25630,
        },
        (33, 16) => Sizes {
            style: 78232,
            table: 20089,
            inputs: 27346,
        },
        (33, 17) => Sizes {
            style: 83149,
            table: 21352,
            inputs: 29062,
        },
        (33, 18) => Sizes {
            style: 88066,
            table: 22615,
            inputs: 30778,
        },
        (33, 19) => Sizes {
            style: 92983,
            table: 23878,
            inputs: 32494,
        },
        (33, 20) => Sizes {
            style: 97900,
            table: 25141,
            inputs: 34210,
        },
        (33, 21) => Sizes {
            style: 102817,
            table: 26404,
            inputs: 35926,
        },
        (33, 22) => Sizes {
            style: 107734,
            table: 27667,
            inputs: 37642,
        },
        (33, 23) => Sizes {
            style: 112651,
            table: 28930,
            inputs: 39358,
        },
        (33, 24) => Sizes {
            style: 117568,
            table: 30193,
            inputs: 41074,
        },
        (33, 25) => Sizes {
            style: 122485,
            table: 31456,
            inputs: 42790,
        },
        (33, 26) => Sizes {
            style: 127402,
            table: 32719,
            inputs: 44506,
        },
        (33, 27) => Sizes {
            style: 132319,
            table: 33982,
            inputs: 46222,
        },
        (33, 28) => Sizes {
            style: 137236,
            table: 35245,
            inputs: 47938,
        },
        (33, 29) => Sizes {
            style: 142153,
            table: 36508,
            inputs: 49654,
        },
        (33, 30) => Sizes {
            style: 147070,
            table: 37771,
            inputs: 51370,
        },
        (33, 31) => Sizes {
            style: 152079,
            table: 39057,
            inputs: 53109,
        },
        (33, 32) => Sizes {
            style: 157128,
            table: 40353,
            inputs: 54858,
        },
        (33, 33) => Sizes {
            style: 162177,
            table: 41649,
            inputs: 56607,
        },
        (33, 34) => Sizes {
            style: 167226,
            table: 42945,
            inputs: 58356,
        },
        (33, 35) => Sizes {
            style: 172275,
            table: 44241,
            inputs: 60105,
        },
        (33, 36) => Sizes {
            style: 177324,
            table: 45537,
            inputs: 61854,
        },
        (33, 37) => Sizes {
            style: 182373,
            table: 46833,
            inputs: 63603,
        },
        (33, 38) => Sizes {
            style: 187422,
            table: 48129,
            inputs: 65352,
        },
        (33, 39) => Sizes {
            style: 192471,
            table: 49425,
            inputs: 67101,
        },
        (33, 40) => Sizes {
            style: 197520,
            table: 50721,
            inputs: 68850,
        },
        (33, 41) => Sizes {
            style: 202569,
            table: 52017,
            inputs: 70599,
        },
        (33, 42) => Sizes {
            style: 207618,
            table: 53313,
            inputs: 72348,
        },
        (33, 43) => Sizes {
            style: 212667,
            table: 54609,
            inputs: 74097,
        },
        (33, 44) => Sizes {
            style: 217716,
            table: 55905,
            inputs: 75846,
        },
        (33, 45) => Sizes {
            style: 222765,
            table: 57201,
            inputs: 77595,
        },
        (33, 46) => Sizes {
            style: 227814,
            table: 58497,
            inputs: 79344,
        },
        (33, 47) => Sizes {
            style: 232863,
            table: 59793,
            inputs: 81093,
        },
        (33, 48) => Sizes {
            style: 237912,
            table: 61089,
            inputs: 82842,
        },
        (33, 49) => Sizes {
            style: 242961,
            table: 62385,
            inputs: 84591,
        },
        (33, 50) => Sizes {
            style: 248010,
            table: 63681,
            inputs: 86340,
        },
        (34, 2) => Sizes {
            style: 9820,
            table: 2515,
            inputs: 3458,
        },
        (34, 3) => Sizes {
            style: 14758,
            table: 3784,
            inputs: 5194,
        },
        (34, 4) => Sizes {
            style: 19824,
            table: 5085,
            inputs: 6962,
        },
        (34, 5) => Sizes {
            style: 24890,
            table: 6386,
            inputs: 8730,
        },
        (34, 6) => Sizes {
            style: 29956,
            table: 7687,
            inputs: 10498,
        },
        (34, 7) => Sizes {
            style: 35022,
            table: 8988,
            inputs: 12266,
        },
        (34, 8) => Sizes {
            style: 40088,
            table: 10289,
            inputs: 14034,
        },
        (34, 9) => Sizes {
            style: 45154,
            table: 11590,
            inputs: 15802,
        },
        (34, 10) => Sizes {
            style: 50220,
            table: 12891,
            inputs: 17570,
        },
        (34, 11) => Sizes {
            style: 55286,
            table: 14192,
            inputs: 19338,
        },
        (34, 12) => Sizes {
            style: 60352,
            table: 15493,
            inputs: 21106,
        },
        (34, 13) => Sizes {
            style: 65418,
            table: 16794,
            inputs: 22874,
        },
        (34, 14) => Sizes {
            style: 70484,
            table: 18095,
            inputs: 24642,
        },
        (34, 15) => Sizes {
            style: 75550,
            table: 19396,
            inputs: 26410,
        },
        (34, 16) => Sizes {
            style: 80616,
            table: 20697,
            inputs: 28178,
        },
        (34, 17) => Sizes {
            style: 85682,
            table: 21998,
            inputs: 29946,
        },
        (34, 18) => Sizes {
            style: 90748,
            table: 23299,
            inputs: 31714,
        },
        (34, 19) => Sizes {
            style: 95814,
            table: 24600,
            inputs: 33482,
        },
        (34, 20) => Sizes {
            style: 100880,
            table: 25901,
            inputs: 35250,
        },
        (34, 21) => Sizes {
            style: 105946,
            table: 27202,
            inputs: 37018,
        },
        (34, 22) => Sizes {
            style: 111012,
            table: 28503,
            inputs: 38786,
        },
        (34, 23) => Sizes {
            style: 116078,
            table: 29804,
            inputs: 40554,
        },
        (34, 24) => Sizes {
            style: 121144,
            table: 31105,
            inputs: 42322,
        },
        (34, 25) => Sizes {
            style: 126210,
            table: 32406,
            inputs: 44090,
        },
        (34, 26) => Sizes {
            style: 131276,
            table: 33707,
            inputs: 45858,
        },
        (34, 27) => Sizes {
            style: 136342,
            table: 35008,
            inputs: 47626,
        },
        (34, 28) => Sizes {
            style: 141408,
            table: 36309,
            inputs: 49394,
        },
        (34, 29) => Sizes {
            style: 146474,
            table: 37610,
            inputs: 51162,
        },
        (34, 30) => Sizes {
            style: 151620,
            table: 38931,
            inputs: 52950,
        },
        (34, 31) => Sizes {
            style: 156822,
            table: 40266,
            inputs: 54752,
        },
        (34, 32) => Sizes {
            style: 162024,
            table: 41601,
            inputs: 56554,
        },
        (34, 33) => Sizes {
            style: 167226,
            table: 42936,
            inputs: 58356,
        },
        (34, 34) => Sizes {
            style: 172428,
            table: 44271,
            inputs: 60158,
        },
        (34, 35) => Sizes {
            style: 177630,
            table: 45606,
            inputs: 61960,
        },
        (34, 36) => Sizes {
            style: 182832,
            table: 46941,
            inputs: 63762,
        },
        (34, 37) => Sizes {
            style: 188034,
            table: 48276,
            inputs: 65564,
        },
        (34, 38) => Sizes {
            style: 193236,
            table: 49611,
            inputs: 67366,
        },
        (34, 39) => Sizes {
            style: 198438,
            table: 50946,
            inputs: 69168,
        },
        (34, 40) => Sizes {
            style: 203640,
            table: 52281,
            inputs: 70970,
        },
        (34, 41) => Sizes {
            style: 208842,
            table: 53616,
            inputs: 72772,
        },
        (34, 42) => Sizes {
            style: 214044,
            table: 54951,
            inputs: 74574,
        },
        (34, 43) => Sizes {
            style: 219246,
            table: 56286,
            inputs: 76376,
        },
        (34, 44) => Sizes {
            style: 224448,
            table: 57621,
            inputs: 78178,
        },
        (34, 45) => Sizes {
            style: 229650,
            table: 58956,
            inputs: 79980,
        },
        (34, 46) => Sizes {
            style: 234852,
            table: 60291,
            inputs: 81782,
        },
        (34, 47) => Sizes {
            style: 240054,
            table: 61626,
            inputs: 83584,
        },
        (34, 48) => Sizes {
            style: 245256,
            table: 62961,
            inputs: 85386,
        },
        (34, 49) => Sizes {
            style: 250458,
            table: 64296,
            inputs: 87188,
        },
        (34, 50) => Sizes {
            style: 255660,
            table: 65631,
            inputs: 88990,
        },
        (35, 2) => Sizes {
            style: 10110,
            table: 2589,
            inputs: 3560,
        },
        (35, 3) => Sizes {
            style: 15205,
            table: 3898,
            inputs: 5350,
        },
        (35, 4) => Sizes {
            style: 20420,
            table: 5237,
            inputs: 7170,
        },
        (35, 5) => Sizes {
            style: 25635,
            table: 6576,
            inputs: 8990,
        },
        (35, 6) => Sizes {
            style: 30850,
            table: 7915,
            inputs: 10810,
        },
        (35, 7) => Sizes {
            style: 36065,
            table: 9254,
            inputs: 12630,
        },
        (35, 8) => Sizes {
            style: 41280,
            table: 10593,
            inputs: 14450,
        },
        (35, 9) => Sizes {
            style: 46495,
            table: 11932,
            inputs: 16270,
        },
        (35, 10) => Sizes {
            style: 51710,
            table: 13271,
            inputs: 18090,
        },
        (35, 11) => Sizes {
            style: 56925,
            table: 14610,
            inputs: 19910,
        },
        (35, 12) => Sizes {
            style: 62140,
            table: 15949,
            inputs: 21730,
        },
        (35, 13) => Sizes {
            style: 67355,
            table: 17288,
            inputs: 23550,
        },
        (35, 14) => Sizes {
            style: 72570,
            table: 18627,
            inputs: 25370,
        },
        (35, 15) => Sizes {
            style: 77785,
            table: 19966,
            inputs: 27190,
        },
        (35, 16) => Sizes {
            style: 83000,
            table: 21305,
            inputs: 29010,
        },
        (35, 17) => Sizes {
            style: 88215,
            table: 22644,
            inputs: 30830,
        },
        (35, 18) => Sizes {
            style: 93430,
            table: 23983,
            inputs: 32650,
        },
        (35, 19) => Sizes {
            style: 98645,
            table: 25322,
            inputs: 34470,
        },
        (35, 20) => Sizes {
            style: 103860,
            table: 26661,
            inputs: 36290,
        },
        (35, 21) => Sizes {
            style: 109075,
            table: 28000,
            inputs: 38110,
        },
        (35, 22) => Sizes {
            style: 114290,
            table: 29339,
            inputs: 39930,
        },
        (35, 23) => Sizes {
            style: 119505,
            table: 30678,
            inputs: 41750,
        },
        (35, 24) => Sizes {
            style: 124720,
            table: 32017,
            inputs: 43570,
        },
        (35, 25) => Sizes {
            style: 129935,
            table: 33356,
            inputs: 45390,
        },
        (35, 26) => Sizes {
            style: 135150,
            table: 34695,
            inputs: 47210,
        },
        (35, 27) => Sizes {
            style: 140365,
            table: 36034,
            inputs: 49030,
        },
        (35, 28) => Sizes {
            style: 145580,
            table: 37373,
            inputs: 50850,
        },
        (35, 29) => Sizes {
            style: 150855,
            table: 38727,
            inputs: 52685,
        },
        (35, 30) => Sizes {
            style: 156210,
            table: 40101,
            inputs: 54540,
        },
        (35, 31) => Sizes {
            style: 161565,
            table: 41475,
            inputs: 56395,
        },
        (35, 32) => Sizes {
            style: 166920,
            table: 42849,
            inputs: 58250,
        },
        (35, 33) => Sizes {
            style: 172275,
            table: 44223,
            inputs: 60105,
        },
        (35, 34) => Sizes {
            style: 177630,
            table: 45597,
            inputs: 61960,
        },
        (35, 35) => Sizes {
            style: 182985,
            table: 46971,
            inputs: 63815,
        },
        (35, 36) => Sizes {
            style: 188340,
            table: 48345,
            inputs: 65670,
        },
        (35, 37) => Sizes {
            style: 193695,
            table: 49719,
            inputs: 67525,
        },
        (35, 38) => Sizes {
            style: 199050,
            table: 51093,
            inputs: 69380,
        },
        (35, 39) => Sizes {
            style: 204405,
            table: 52467,
            inputs: 71235,
        },
        (35, 40) => Sizes {
            style: 209760,
            table: 53841,
            inputs: 73090,
        },
        (35, 41) => Sizes {
            style: 215115,
            table: 55215,
            inputs: 74945,
        },
        (35, 42) => Sizes {
            style: 220470,
            table: 56589,
            inputs: 76800,
        },
        (35, 43) => Sizes {
            style: 225825,
            table: 57963,
            inputs: 78655,
        },
        (35, 44) => Sizes {
            style: 231180,
            table: 59337,
            inputs: 80510,
        },
        (35, 45) => Sizes {
            style: 236535,
            table: 60711,
            inputs: 82365,
        },
        (35, 46) => Sizes {
            style: 241890,
            table: 62085,
            inputs: 84220,
        },
        (35, 47) => Sizes {
            style: 247245,
            table: 63459,
            inputs: 86075,
        },
        (35, 48) => Sizes {
            style: 252600,
            table: 64833,
            inputs: 87930,
        },
        (35, 49) => Sizes {
            style: 257955,
            table: 66207,
            inputs: 89785,
        },
        (35, 50) => Sizes {
            style: 263310,
            table: 67581,
            inputs: 91640,
        },
        (36, 2) => Sizes {
            style: 10400,
            table: 2663,
            inputs: 3662,
        },
        (36, 3) => Sizes {
            style: 15652,
            table: 4012,
            inputs: 5506,
        },
        (36, 4) => Sizes {
            style: 21016,
            table: 5389,
            inputs: 7378,
        },
        (36, 5) => Sizes {
            style: 26380,
            table: 6766,
            inputs: 9250,
        },
        (36, 6) => Sizes {
            style: 31744,
            table: 8143,
            inputs: 11122,
        },
        (36, 7) => Sizes {
            style: 37108,
            table: 9520,
            inputs: 12994,
        },
        (36, 8) => Sizes {
            style: 42472,
            table: 10897,
            inputs: 14866,
        },
        (36, 9) => Sizes {
            style: 47836,
            table: 12274,
            inputs: 16738,
        },
        (36, 10) => Sizes {
            style: 53200,
            table: 13651,
            inputs: 18610,
        },
        (36, 11) => Sizes {
            style: 58564,
            table: 15028,
            inputs: 20482,
        },
        (36, 12) => Sizes {
            style: 63928,
            table: 16405,
            inputs: 22354,
        },
        (36, 13) => Sizes {
            style: 69292,
            table: 17782,
            inputs: 24226,
        },
        (36, 14) => Sizes {
            style: 74656,
            table: 19159,
            inputs: 26098,
        },
        (36, 15) => Sizes {
            style: 80020,
            table: 20536,
            inputs: 27970,
        },
        (36, 16) => Sizes {
            style: 85384,
            table: 21913,
            inputs: 29842,
        },
        (36, 17) => Sizes {
            style: 90748,
            table: 23290,
            inputs: 31714,
        },
        (36, 18) => Sizes {
            style: 96112,
            table: 24667,
            inputs: 33586,
        },
        (36, 19) => Sizes {
            style: 101476,
            table: 26044,
            inputs: 35458,
        },
        (36, 20) => Sizes {
            style: 106840,
            table: 27421,
            inputs: 37330,
        },
        (36, 21) => Sizes {
            style: 112204,
            table: 28798,
            inputs: 39202,
        },
        (36, 22) => Sizes {
            style: 117568,
            table: 30175,
            inputs: 41074,
        },
        (36, 23) => Sizes {
            style: 122932,
            table: 31552,
            inputs: 42946,
        },
        (36, 24) => Sizes {
            style: 128296,
            table: 32929,
            inputs: 44818,
        },
        (36, 25) => Sizes {
            style: 133660,
            table: 34306,
            inputs: 46690,
        },
        (36, 26) => Sizes {
            style: 139024,
            table: 35683,
            inputs: 48562,
        },
        (36, 27) => Sizes {
            style: 144388,
            table: 37060,
            inputs: 50434,
        },
        (36, 28) => Sizes {
            style: 149784,
            table: 38445,
            inputs: 52314,
        },
        (36, 29) => Sizes {
            style: 155292,
            table: 39858,
            inputs: 54222,
        },
        (36, 30) => Sizes {
            style: 160800,
            table: 41271,
            inputs: 56130,
        },
        (36, 31) => Sizes {
            style: 166308,
            table: 42684,
            inputs: 58038,
        },
        (36, 32) => Sizes {
            style: 171816,
            table: 44097,
            inputs: 59946,
        },
        (36, 33) => Sizes {
            style: 177324,
            table: 45510,
            inputs: 61854,
        },
        (36, 34) => Sizes {
            style: 182832,
            table: 46923,
            inputs: 63762,
        },
        (36, 35) => Sizes {
            style: 188340,
            table: 48336,
            inputs: 65670,
        },
        (36, 36) => Sizes {
            style: 193848,
            table: 49749,
            inputs: 67578,
        },
        (36, 37) => Sizes {
            style: 199356,
            table: 51162,
            inputs: 69486,
        },
        (36, 38) => Sizes {
            style: 204864,
            table: 52575,
            inputs: 71394,
        },
        (36, 39) => Sizes {
            style: 210372,
            table: 53988,
            inputs: 73302,
        },
        (36, 40) => Sizes {
            style: 215880,
            table: 55401,
            inputs: 75210,
        },
        (36, 41) => Sizes {
            style: 221388,
            table: 56814,
            inputs: 77118,
        },
        (36, 42) => Sizes {
            style: 226896,
            table: 58227,
            inputs: 79026,
        },
        (36, 43) => Sizes {
            style: 232404,
            table: 59640,
            inputs: 80934,
        },
        (36, 44) => Sizes {
            style: 237912,
            table: 61053,
            inputs: 82842,
        },
        (36, 45) => Sizes {
            style: 243420,
            table: 62466,
            inputs: 84750,
        },
        (36, 46) => Sizes {
            style: 248928,
            table: 63879,
            inputs: 86658,
        },
        (36, 47) => Sizes {
            style: 254436,
            table: 65292,
            inputs: 88566,
        },
        (36, 48) => Sizes {
            style: 259944,
            table: 66705,
            inputs: 90474,
        },
        (36, 49) => Sizes {
            style: 265452,
            table: 68118,
            inputs: 92382,
        },
        (36, 50) => Sizes {
            style: 270960,
            table: 69531,
            inputs: 94290,
        },
        (37, 2) => Sizes {
            style: 10690,
            table: 2737,
            inputs: 3764,
        },
        (37, 3) => Sizes {
            style: 16099,
            table: 4126,
            inputs: 5662,
        },
        (37, 4) => Sizes {
            style: 21612,
            table: 5541,
            inputs: 7586,
        },
        (37, 5) => Sizes {
            style: 27125,
            table: 6956,
            inputs: 9510,
        },
        (37, 6) => Sizes {
            style: 32638,
            table: 8371,
            inputs: 11434,
        },
        (37, 7) => Sizes {
            style: 38151,
            table: 9786,
            inputs: 13358,
        },
        (37, 8) => Sizes {
            style: 43664,
            table: 11201,
            inputs: 15282,
        },
        (37, 9) => Sizes {
            style: 49177,
            table: 12616,
            inputs: 17206,
        },
        (37, 10) => Sizes {
            style: 54690,
            table: 14031,
            inputs: 19130,
        },
        (37, 11) => Sizes {
            style: 60203,
            table: 15446,
            inputs: 21054,
        },
        (37, 12) => Sizes {
            style: 65716,
            table: 16861,
            inputs: 22978,
        },
        (37, 13) => Sizes {
            style: 71229,
            table: 18276,
            inputs: 24902,
        },
        (37, 14) => Sizes {
            style: 76742,
            table: 19691,
            inputs: 26826,
        },
        (37, 15) => Sizes {
            style: 82255,
            table: 21106,
            inputs: 28750,
        },
        (37, 16) => Sizes {
            style: 87768,
            table: 22521,
            inputs: 30674,
        },
        (37, 17) => Sizes {
            style: 93281,
            table: 23936,
            inputs: 32598,
        },
        (37, 18) => Sizes {
            style: 98794,
            table: 25351,
            inputs: 34522,
        },
        (37, 19) => Sizes {
            style: 104307,
            table: 26766,
            inputs: 36446,
        },
        (37, 20) => Sizes {
            style: 109820,
            table: 28181,
            inputs: 38370,
        },
        (37, 21) => Sizes {
            style: 115333,
            table: 29596,
            inputs: 40294,
        },
        (37, 22) => Sizes {
            style: 120846,
            table: 31011,
            inputs: 42218,
        },
        (37, 23) => Sizes {
            style: 126359,
            table: 32426,
            inputs: 44142,
        },
        (37, 24) => Sizes {
            style: 131872,
            table: 33841,
            inputs: 46066,
        },
        (37, 25) => Sizes {
            style: 137385,
            table: 35256,
            inputs: 47990,
        },
        (37, 26) => Sizes {
            style: 142898,
            table: 36671,
            inputs: 49914,
        },
        (37, 27) => Sizes {
            style: 148411,
            table: 38086,
            inputs: 51838,
        },
        (37, 28) => Sizes {
            style: 154068,
            table: 39537,
            inputs: 53798,
        },
        (37, 29) => Sizes {
            style: 159729,
            table: 40989,
            inputs: 55759,
        },
        (37, 30) => Sizes {
            style: 165390,
            table: 42441,
            inputs: 57720,
        },
        (37, 31) => Sizes {
            style: 171051,
            table: 43893,
            inputs: 59681,
        },
        (37, 32) => Sizes {
            style: 176712,
            table: 45345,
            inputs: 61642,
        },
        (37, 33) => Sizes {
            style: 182373,
            table: 46797,
            inputs: 63603,
        },
        (37, 34) => Sizes {
            style: 188034,
            table: 48249,
            inputs: 65564,
        },
        (37, 35) => Sizes {
            style: 193695,
            table: 49701,
            inputs: 67525,
        },
        (37, 36) => Sizes {
            style: 199356,
            table: 51153,
            inputs: 69486,
        },
        (37, 37) => Sizes {
            style: 205017,
            table: 52605,
            inputs: 71447,
        },
        (37, 38) => Sizes {
            style: 210678,
            table: 54057,
            inputs: 73408,
        },
        (37, 39) => Sizes {
            style: 216339,
            table: 55509,
            inputs: 75369,
        },
        (37, 40) => Sizes {
            style: 222000,
            table: 56961,
            inputs: 77330,
        },
        (37, 41) => Sizes {
            style: 227661,
            table: 58413,
            inputs: 79291,
        },
        (37, 42) => Sizes {
            style: 233322,
            table: 59865,
            inputs: 81252,
        },
        (37, 43) => Sizes {
            style: 238983,
            table: 61317,
            inputs: 83213,
        },
        (37, 44) => Sizes {
            style: 244644,
            table: 62769,
            inputs: 85174,
        },
        (37, 45) => Sizes {
            style: 250305,
            table: 64221,
            inputs: 87135,
        },
        (37, 46) => Sizes {
            style: 255966,
            table: 65673,
            inputs: 89096,
        },
        (37, 47) => Sizes {
            style: 261627,
            table: 67125,
            inputs: 91057,
        },
        (37, 48) => Sizes {
            style: 267288,
            table: 68577,
            inputs: 93018,
        },
        (37, 49) => Sizes {
            style: 272949,
            table: 70029,
            inputs: 94979,
        },
        (37, 50) => Sizes {
            style: 278610,
            table: 71481,
            inputs: 96940,
        },
        (38, 2) => Sizes {
            style: 10980,
            table: 2811,
            inputs: 3866,
        },
        (38, 3) => Sizes {
            style: 16546,
            table: 4240,
            inputs: 5818,
        },
        (38, 4) => Sizes {
            style: 22208,
            table: 5693,
            inputs: 7794,
        },
        (38, 5) => Sizes {
            style: 27870,
            table: 7146,
            inputs: 9770,
        },
        (38, 6) => Sizes {
            style: 33532,
            table: 8599,
            inputs: 11746,
        },
        (38, 7) => Sizes {
            style: 39194,
            table: 10052,
            inputs: 13722,
        },
        (38, 8) => Sizes {
            style: 44856,
            table: 11505,
            inputs: 15698,
        },
        (38, 9) => Sizes {
            style: 50518,
            table: 12958,
            inputs: 17674,
        },
        (38, 10) => Sizes {
            style: 56180,
            table: 14411,
            inputs: 19650,
        },
        (38, 11) => Sizes {
            style: 61842,
            table: 15864,
            inputs: 21626,
        },
        (38, 12) => Sizes {
            style: 67504,
            table: 17317,
            inputs: 23602,
        },
        (38, 13) => Sizes {
            style: 73166,
            table: 18770,
            inputs: 25578,
        },
        (38, 14) => Sizes {
            style: 78828,
            table: 20223,
            inputs: 27554,
        },
        (38, 15) => Sizes {
            style: 84490,
            table: 21676,
            inputs: 29530,
        },
        (38, 16) => Sizes {
            style: 90152,
            table: 23129,
            inputs: 31506,
        },
        (38, 17) => Sizes {
            style: 95814,
            table: 24582,
            inputs: 33482,
        },
        (38, 18) => Sizes {
            style: 101476,
            table: 26035,
            inputs: 35458,
        },
        (38, 19) => Sizes {
            style: 107138,
            table: 27488,
            inputs: 37434,
        },
        (38, 20) => Sizes {
            style: 112800,
            table: 28941,
            inputs: 39410,
        },
        (38, 21) => Sizes {
            style: 118462,
            table: 30394,
            inputs: 41386,
        },
        (38, 22) => Sizes {
            style: 124124,
            table: 31847,
            inputs: 43362,
        },
        (38, 23) => Sizes {
            style: 129786,
            table: 33300,
            inputs: 45338,
        },
        (38, 24) => Sizes {
            style: 135448,
            table: 34753,
            inputs: 47314,
        },
        (38, 25) => Sizes {
            style: 141110,
            table: 36206,
            inputs: 49290,
        },
        (38, 26) => Sizes {
            style: 146772,
            table: 37659,
            inputs: 51266,
        },
        (38, 27) => Sizes {
            style: 152538,
            table: 39138,
            inputs: 53268,
        },
        (38, 28) => Sizes {
            style: 158352,
            table: 40629,
            inputs: 55282,
        },
        (38, 29) => Sizes {
            style: 164166,
            table: 42120,
            inputs: 57296,
        },
        (38, 30) => Sizes {
            style: 169980,
            table: 43611,
            inputs: 59310,
        },
        (38, 31) => Sizes {
            style: 175794,
            table: 45102,
            inputs: 61324,
        },
        (38, 32) => Sizes {
            style: 181608,
            table: 46593,
            inputs: 63338,
        },
        (38, 33) => Sizes {
            style: 187422,
            table: 48084,
            inputs: 65352,
        },
        (38, 34) => Sizes {
            style: 193236,
            table: 49575,
            inputs: 67366,
        },
        (38, 35) => Sizes {
            style: 199050,
            table: 51066,
            inputs: 69380,
        },
        (38, 36) => Sizes {
            style: 204864,
            table: 52557,
            inputs: 71394,
        },
        (38, 37) => Sizes {
            style: 210678,
            table: 54048,
            inputs: 73408,
        },
        (38, 38) => Sizes {
            style: 216492,
            table: 55539,
            inputs: 75422,
        },
        (38, 39) => Sizes {
            style: 222306,
            table: 57030,
            inputs: 77436,
        },
        (38, 40) => Sizes {
            style: 228120,
            table: 58521,
            inputs: 79450,
        },
        (38, 41) => Sizes {
            style: 233934,
            table: 60012,
            inputs: 81464,
        },
        (38, 42) => Sizes {
            style: 239748,
            table: 61503,
            inputs: 83478,
        },
        (38, 43) => Sizes {
            style: 245562,
            table: 62994,
            inputs: 85492,
        },
        (38, 44) => Sizes {
            style: 251376,
            table: 64485,
            inputs: 87506,
        },
        (38, 45) => Sizes {
            style: 257190,
            table: 65976,
            inputs: 89520,
        },
        (38, 46) => Sizes {
            style: 263004,
            table: 67467,
            inputs: 91534,
        },
        (38, 47) => Sizes {
            style: 268818,
            table: 68958,
            inputs: 93548,
        },
        (38, 48) => Sizes {
            style: 274632,
            table: 70449,
            inputs: 95562,
        },
        (38, 49) => Sizes {
            style: 280446,
            table: 71940,
            inputs: 97576,
        },
        (38, 50) => Sizes {
            style: 286260,
            table: 73431,
            inputs: 99590,
        },
        (39, 2) => Sizes {
            style: 11270,
            table: 2885,
            inputs: 3968,
        },
        (39, 3) => Sizes {
            style: 16993,
            table: 4354,
            inputs: 5974,
        },
        (39, 4) => Sizes {
            style: 22804,
            table: 5845,
            inputs: 8002,
        },
        (39, 5) => Sizes {
            style: 28615,
            table: 7336,
            inputs: 10030,
        },
        (39, 6) => Sizes {
            style: 34426,
            table: 8827,
            inputs: 12058,
        },
        (39, 7) => Sizes {
            style: 40237,
            table: 10318,
            inputs: 14086,
        },
        (39, 8) => Sizes {
            style: 46048,
            table: 11809,
            inputs: 16114,
        },
        (39, 9) => Sizes {
            style: 51859,
            table: 13300,
            inputs: 18142,
        },
        (39, 10) => Sizes {
            style: 57670,
            table: 14791,
            inputs: 20170,
        },
        (39, 11) => Sizes {
            style: 63481,
            table: 16282,
            inputs: 22198,
        },
        (39, 12) => Sizes {
            style: 69292,
            table: 17773,
            inputs: 24226,
        },
        (39, 13) => Sizes {
            style: 75103,
            table: 19264,
            inputs: 26254,
        },
        (39, 14) => Sizes {
            style: 80914,
            table: 20755,
            inputs: 28282,
        },
        (39, 15) => Sizes {
            style: 86725,
            table: 22246,
            inputs: 30310,
        },
        (39, 16) => Sizes {
            style: 92536,
            table: 23737,
            inputs: 32338,
        },
        (39, 17) => Sizes {
            style: 98347,
            table: 25228,
            inputs: 34366,
        },
        (39, 18) => Sizes {
            style: 104158,
            table: 26719,
            inputs: 36394,
        },
        (39, 19) => Sizes {
            style: 109969,
            table: 28210,
            inputs: 38422,
        },
        (39, 20) => Sizes {
            style: 115780,
            table: 29701,
            inputs: 40450,
        },
        (39, 21) => Sizes {
            style: 121591,
            table: 31192,
            inputs: 42478,
        },
        (39, 22) => Sizes {
            style: 127402,
            table: 32683,
            inputs: 44506,
        },
        (39, 23) => Sizes {
            style: 133213,
            table: 34174,
            inputs: 46534,
        },
        (39, 24) => Sizes {
            style: 139024,
            table: 35665,
            inputs: 48562,
        },
        (39, 25) => Sizes {
            style: 144835,
            table: 37156,
            inputs: 50590,
        },
        (39, 26) => Sizes {
            style: 150702,
            table: 38661,
            inputs: 52632,
        },
        (39, 27) => Sizes {
            style: 156669,
            table: 40191,
            inputs: 54699,
        },
        (39, 28) => Sizes {
            style: 162636,
            table: 41721,
            inputs: 56766,
        },
        (39, 29) => Sizes {
            style: 168603,
            table: 43251,
            inputs: 58833,
        },
        (39, 30) => Sizes {
            style: 174570,
            table: 44781,
            inputs: 60900,
        },
        (39, 31) => Sizes {
            style: 180537,
            table: 46311,
            inputs: 62967,
        },
        (39, 32) => Sizes {
            style: 186504,
            table: 47841,
            inputs: 65034,
        },
        (39, 33) => Sizes {
            style: 192471,
            table: 49371,
            inputs: 67101,
        },
        (39, 34) => Sizes {
            style: 198438,
            table: 50901,
            inputs: 69168,
        },
        (39, 35) => Sizes {
            style: 204405,
            table: 52431,
            inputs: 71235,
        },
        (39, 36) => Sizes {
            style: 210372,
            table: 53961,
            inputs: 73302,
        },
        (39, 37) => Sizes {
            style: 216339,
            table: 55491,
            inputs: 75369,
        },
        (39, 38) => Sizes {
            style: 222306,
            table: 57021,
            inputs: 77436,
        },
        (39, 39) => Sizes {
            style: 228273,
            table: 58551,
            inputs: 79503,
        },
        (39, 40) => Sizes {
            style: 234240,
            table: 60081,
            inputs: 81570,
        },
        (39, 41) => Sizes {
            style: 240207,
            table: 61611,
            inputs: 83637,
        },
        (39, 42) => Sizes {
            style: 246174,
            table: 63141,
            inputs: 85704,
        },
        (39, 43) => Sizes {
            style: 252141,
            table: 64671,
            inputs: 87771,
        },
        (39, 44) => Sizes {
            style: 258108,
            table: 66201,
            inputs: 89838,
        },
        (39, 45) => Sizes {
            style: 264075,
            table: 67731,
            inputs: 91905,
        },
        (39, 46) => Sizes {
            style: 270042,
            table: 69261,
            inputs: 93972,
        },
        (39, 47) => Sizes {
            style: 276009,
            table: 70791,
            inputs: 96039,
        },
        (39, 48) => Sizes {
            style: 281976,
            table: 72321,
            inputs: 98106,
        },
        (39, 49) => Sizes {
            style: 287943,
            table: 73851,
            inputs: 100173,
        },
        (39, 50) => Sizes {
            style: 293910,
            table: 75381,
            inputs: 102240,
        },
        (40, 2) => Sizes {
            style: 11560,
            table: 2959,
            inputs: 4070,
        },
        (40, 3) => Sizes {
            style: 17440,
            table: 4468,
            inputs: 6130,
        },
        (40, 4) => Sizes {
            style: 23400,
            table: 5997,
            inputs: 8210,
        },
        (40, 5) => Sizes {
            style: 29360,
            table: 7526,
            inputs: 10290,
        },
        (40, 6) => Sizes {
            style: 35320,
            table: 9055,
            inputs: 12370,
        },
        (40, 7) => Sizes {
            style: 41280,
            table: 10584,
            inputs: 14450,
        },
        (40, 8) => Sizes {
            style: 47240,
            table: 12113,
            inputs: 16530,
        },
        (40, 9) => Sizes {
            style: 53200,
            table: 13642,
            inputs: 18610,
        },
        (40, 10) => Sizes {
            style: 59160,
            table: 15171,
            inputs: 20690,
        },
        (40, 11) => Sizes {
            style: 65120,
            table: 16700,
            inputs: 22770,
        },
        (40, 12) => Sizes {
            style: 71080,
            table: 18229,
            inputs: 24850,
        },
        (40, 13) => Sizes {
            style: 77040,
            table: 19758,
            inputs: 26930,
        },
        (40, 14) => Sizes {
            style: 83000,
            table: 21287,
            inputs: 29010,
        },
        (40, 15) => Sizes {
            style: 88960,
            table: 22816,
            inputs: 31090,
        },
        (40, 16) => Sizes {
            style: 94920,
            table: 24345,
            inputs: 33170,
        },
        (40, 17) => Sizes {
            style: 100880,
            table: 25874,
            inputs: 35250,
        },
        (40, 18) => Sizes {
            style: 106840,
            table: 27403,
            inputs: 37330,
        },
        (40, 19) => Sizes {
            style: 112800,
            table: 28932,
            inputs: 39410,
        },
        (40, 20) => Sizes {
            style: 118760,
            table: 30461,
            inputs: 41490,
        },
        (40, 21) => Sizes {
            style: 124720,
            table: 31990,
            inputs: 43570,
        },
        (40, 22) => Sizes {
            style: 130680,
            table: 33519,
            inputs: 45650,
        },
        (40, 23) => Sizes {
            style: 136640,
            table: 35048,
            inputs: 47730,
        },
        (40, 24) => Sizes {
            style: 142600,
            table: 36577,
            inputs: 49810,
        },
        (40, 25) => Sizes {
            style: 148560,
            table: 38106,
            inputs: 51890,
        },
        (40, 26) => Sizes {
            style: 154680,
            table: 39675,
            inputs: 54010,
        },
        (40, 27) => Sizes {
            style: 160800,
            table: 41244,
            inputs: 56130,
        },
        (40, 28) => Sizes {
            style: 166920,
            table: 42813,
            inputs: 58250,
        },
        (40, 29) => Sizes {
            style: 173040,
            table: 44382,
            inputs: 60370,
        },
        (40, 30) => Sizes {
            style: 179160,
            table: 45951,
            inputs: 62490,
        },
        (40, 31) => Sizes {
            style: 185280,
            table: 47520,
            inputs: 64610,
        },
        (40, 32) => Sizes {
            style: 191400,
            table: 49089,
            inputs: 66730,
        },
        (40, 33) => Sizes {
            style: 197520,
            table: 50658,
            inputs: 68850,
        },
        (40, 34) => Sizes {
            style: 203640,
            table: 52227,
            inputs: 70970,
        },
        (40, 35) => Sizes {
            style: 209760,
            table: 53796,
            inputs: 73090,
        },
        (40, 36) => Sizes {
            style: 215880,
            table: 55365,
            inputs: 75210,
        },
        (40, 37) => Sizes {
            style: 222000,
            table: 56934,
            inputs: 77330,
        },
        (40, 38) => Sizes {
            style: 228120,
            table: 58503,
            inputs: 79450,
        },
        (40, 39) => Sizes {
            style: 234240,
            table: 60072,
            inputs: 81570,
        },
        (40, 40) => Sizes {
            style: 240360,
            table: 61641,
            inputs: 83690,
        },
        (40, 41) => Sizes {
            style: 246480,
            table: 63210,
            inputs: 85810,
        },
        (40, 42) => Sizes {
            style: 252600,
            table: 64779,
            inputs: 87930,
        },
        (40, 43) => Sizes {
            style: 258720,
            table: 66348,
            inputs: 90050,
        },
        (40, 44) => Sizes {
            style: 264840,
            table: 67917,
            inputs: 92170,
        },
        (40, 45) => Sizes {
            style: 270960,
            table: 69486,
            inputs: 94290,
        },
        (40, 46) => Sizes {
            style: 277080,
            table: 71055,
            inputs: 96410,
        },
        (40, 47) => Sizes {
            style: 283200,
            table: 72624,
            inputs: 98530,
        },
        (40, 48) => Sizes {
            style: 289320,
            table: 74193,
            inputs: 100650,
        },
        (40, 49) => Sizes {
            style: 295440,
            table: 75762,
            inputs: 102770,
        },
        (40, 50) => Sizes {
            style: 301560,
            table: 77331,
            inputs: 104890,
        },
        (41, 2) => Sizes {
            style: 11850,
            table: 3033,
            inputs: 4172,
        },
        (41, 3) => Sizes {
            style: 17887,
            table: 4582,
            inputs: 6286,
        },
        (41, 4) => Sizes {
            style: 23996,
            table: 6149,
            inputs: 8418,
        },
        (41, 5) => Sizes {
            style: 30105,
            table: 7716,
            inputs: 10550,
        },
        (41, 6) => Sizes {
            style: 36214,
            table: 9283,
            inputs: 12682,
        },
        (41, 7) => Sizes {
            style: 42323,
            table: 10850,
            inputs: 14814,
        },
        (41, 8) => Sizes {
            style: 48432,
            table: 12417,
            inputs: 16946,
        },
        (41, 9) => Sizes {
            style: 54541,
            table: 13984,
            inputs: 19078,
        },
        (41, 10) => Sizes {
            style: 60650,
            table: 15551,
            inputs: 21210,
        },
        (41, 11) => Sizes {
            style: 66759,
            table: 17118,
            inputs: 23342,
        },
        (41, 12) => Sizes {
            style: 72868,
            table: 18685,
            inputs: 25474,
        },
        (41, 13) => Sizes {
            style: 78977,
            table: 20252,
            inputs: 27606,
        },
        (41, 14) => Sizes {
            style: 85086,
            table: 21819,
            inputs: 29738,
        },
        (41, 15) => Sizes {
            style: 91195,
            table: 23386,
            inputs: 31870,
        },
        (41, 16) => Sizes {
            style: 97304,
            table: 24953,
            inputs: 34002,
        },
        (41, 17) => Sizes {
            style: 103413,
            table: 26520,
            inputs: 36134,
        },
        (41, 18) => Sizes {
            style: 109522,
            table: 28087,
            inputs: 38266,
        },
        (41, 19) => Sizes {
            style: 115631,
            table: 29654,
            inputs: 40398,
        },
        (41, 20) => Sizes {
            style: 121740,
            table: 31221,
            inputs: 42530,
        },
        (41, 21) => Sizes {
            style: 127849,
            table: 32788,
            inputs: 44662,
        },
        (41, 22) => Sizes {
            style: 133958,
            table: 34355,
            inputs: 46794,
        },
        (41, 23) => Sizes {
            style: 140067,
            table: 35922,
            inputs: 48926,
        },
        (41, 24) => Sizes {
            style: 146176,
            table: 37489,
            inputs: 51058,
        },
        (41, 25) => Sizes {
            style: 152385,
            table: 39081,
            inputs: 53215,
        },
        (41, 26) => Sizes {
            style: 158658,
            table: 40689,
            inputs: 55388,
        },
        (41, 27) => Sizes {
            style: 164931,
            table: 42297,
            inputs: 57561,
        },
        (41, 28) => Sizes {
            style: 171204,
            table: 43905,
            inputs: 59734,
        },
        (41, 29) => Sizes {
            style: 177477,
            table: 45513,
            inputs: 61907,
        },
        (41, 30) => Sizes {
            style: 183750,
            table: 47121,
            inputs: 64080,
        },
        (41, 31) => Sizes {
            style: 190023,
            table: 48729,
            inputs: 66253,
        },
        (41, 32) => Sizes {
            style: 196296,
            table: 50337,
            inputs: 68426,
        },
        (41, 33) => Sizes {
            style: 202569,
            table: 51945,
            inputs: 70599,
        },
        (41, 34) => Sizes {
            style: 208842,
            table: 53553,
            inputs: 72772,
        },
        (41, 35) => Sizes {
            style: 215115,
            table: 55161,
            inputs: 74945,
        },
        (41, 36) => Sizes {
            style: 221388,
            table: 56769,
            inputs: 77118,
        },
        (41, 37) => Sizes {
            style: 227661,
            table: 58377,
            inputs: 79291,
        },
        (41, 38) => Sizes {
            style: 233934,
            table: 59985,
            inputs: 81464,
        },
        (41, 39) => Sizes {
            style: 240207,
            table: 61593,
            inputs: 83637,
        },
        (41, 40) => Sizes {
            style: 246480,
            table: 63201,
            inputs: 85810,
        },
        (41, 41) => Sizes {
            style: 252753,
            table: 64809,
            inputs: 87983,
        },
        (41, 42) => Sizes {
            style: 259026,
            table: 66417,
            inputs: 90156,
        },
        (41, 43) => Sizes {
            style: 265299,
            table: 68025,
            inputs: 92329,
        },
        (41, 44) => Sizes {
            style: 271572,
            table: 69633,
            inputs: 94502,
        },
        (41, 45) => Sizes {
            style: 277845,
            table: 71241,
            inputs: 96675,
        },
        (41, 46) => Sizes {
            style: 284118,
            table: 72849,
            inputs: 98848,
        },
        (41, 47) => Sizes {
            style: 290391,
            table: 74457,
            inputs: 101021,
        },
        (41, 48) => Sizes {
            style: 296664,
            table: 76065,
            inputs: 103194,
        },
        (41, 49) => Sizes {
            style: 302937,
            table: 77673,
            inputs: 105367,
        },
        (41, 50) => Sizes {
            style: 309210,
            table: 79281,
            inputs: 107540,
        },
        (42, 2) => Sizes {
            style: 12140,
            table: 3107,
            inputs: 4274,
        },
        (42, 3) => Sizes {
            style: 18334,
            table: 4696,
            inputs: 6442,
        },
        (42, 4) => Sizes {
            style: 24592,
            table: 6301,
            inputs: 8626,
        },
        (42, 5) => Sizes {
            style: 30850,
            table: 7906,
            inputs: 10810,
        },
        (42, 6) => Sizes {
            style: 37108,
            table: 9511,
            inputs: 12994,
        },
        (42, 7) => Sizes {
            style: 43366,
            table: 11116,
            inputs: 15178,
        },
        (42, 8) => Sizes {
            style: 49624,
            table: 12721,
            inputs: 17362,
        },
        (42, 9) => Sizes {
            style: 55882,
            table: 14326,
            inputs: 19546,
        },
        (42, 10) => Sizes {
            style: 62140,
            table: 15931,
            inputs: 21730,
        },
        (42, 11) => Sizes {
            style: 68398,
            table: 17536,
            inputs: 23914,
        },
        (42, 12) => Sizes {
            style: 74656,
            table: 19141,
            inputs: 26098,
        },
        (42, 13) => Sizes {
            style: 80914,
            table: 20746,
            inputs: 28282,
        },
        (42, 14) => Sizes {
            style: 87172,
            table: 22351,
            inputs: 30466,
        },
        (42, 15) => Sizes {
            style: 93430,
            table: 23956,
            inputs: 32650,
        },
        (42, 16) => Sizes {
            style: 99688,
            table: 25561,
            inputs: 34834,
        },
        (42, 17) => Sizes {
            style: 105946,
            table: 27166,
            inputs: 37018,
        },
        (42, 18) => Sizes {
            style: 112204,
            table: 28771,
            inputs: 39202,
        },
        (42, 19) => Sizes {
            style: 118462,
            table: 30376,
            inputs: 41386,
        },
        (42, 20) => Sizes {
            style: 124720,
            table: 31981,
            inputs: 43570,
        },
        (42, 21) => Sizes {
            style: 130978,
            table: 33586,
            inputs: 45754,
        },
        (42, 22) => Sizes {
            style: 137236,
            table: 35191,
            inputs: 47938,
        },
        (42, 23) => Sizes {
            style: 143494,
            table: 36796,
            inputs: 50122,
        },
        (42, 24) => Sizes {
            style: 149784,
            table: 38409,
            inputs: 52314,
        },
        (42, 25) => Sizes {
            style: 156210,
            table: 40056,
            inputs: 54540,
        },
        (42, 26) => Sizes {
            style: 162636,
            table: 41703,
            inputs: 56766,
        },
        (42, 27) => Sizes {
            style: 169062,
            table: 43350,
            inputs: 58992,
        },
        (42, 28) => Sizes {
            style: 175488,
            table: 44997,
            inputs: 61218,
        },
        (42, 29) => Sizes {
            style: 181914,
            table: 46644,
            inputs: 63444,
        },
        (42, 30) => Sizes {
            style: 188340,
            table: 48291,
            inputs: 65670,
        },
        (42, 31) => Sizes {
            style: 194766,
            table: 49938,
            inputs: 67896,
        },
        (42, 32) => Sizes {
            style: 201192,
            table: 51585,
            inputs: 70122,
        },
        (42, 33) => Sizes {
            style: 207618,
            table: 53232,
            inputs: 72348,
        },
        (42, 34) => Sizes {
            style: 214044,
            table: 54879,
            inputs: 74574,
        },
        (42, 35) => Sizes {
            style: 220470,
            table: 56526,
            inputs: 76800,
        },
        (42, 36) => Sizes {
            style: 226896,
            table: 58173,
            inputs: 79026,
        },
        (42, 37) => Sizes {
            style: 233322,
            table: 59820,
            inputs: 81252,
        },
        (42, 38) => Sizes {
            style: 239748,
            table: 61467,
            inputs: 83478,
        },
        (42, 39) => Sizes {
            style: 246174,
            table: 63114,
            inputs: 85704,
        },
        (42, 40) => Sizes {
            style: 252600,
            table: 64761,
            inputs: 87930,
        },
        (42, 41) => Sizes {
            style: 259026,
            table: 66408,
            inputs: 90156,
        },
        (42, 42) => Sizes {
            style: 265452,
            table: 68055,
            inputs: 92382,
        },
        (42, 43) => Sizes {
            style: 271878,
            table: 69702,
            inputs: 94608,
        },
        (42, 44) => Sizes {
            style: 278304,
            table: 71349,
            inputs: 96834,
        },
        (42, 45) => Sizes {
            style: 284730,
            table: 72996,
            inputs: 99060,
        },
        (42, 46) => Sizes {
            style: 291156,
            table: 74643,
            inputs: 101286,
        },
        (42, 47) => Sizes {
            style: 297582,
            table: 76290,
            inputs: 103512,
        },
        (42, 48) => Sizes {
            style: 304008,
            table: 77937,
            inputs: 105738,
        },
        (42, 49) => Sizes {
            style: 310434,
            table: 79584,
            inputs: 107964,
        },
        (42, 50) => Sizes {
            style: 316860,
            table: 81231,
            inputs: 110190,
        },
        (43, 2) => Sizes {
            style: 12430,
            table: 3181,
            inputs: 4376,
        },
        (43, 3) => Sizes {
            style: 18781,
            table: 4810,
            inputs: 6598,
        },
        (43, 4) => Sizes {
            style: 25188,
            table: 6453,
            inputs: 8834,
        },
        (43, 5) => Sizes {
            style: 31595,
            table: 8096,
            inputs: 11070,
        },
        (43, 6) => Sizes {
            style: 38002,
            table: 9739,
            inputs: 13306,
        },
        (43, 7) => Sizes {
            style: 44409,
            table: 11382,
            inputs: 15542,
        },
        (43, 8) => Sizes {
            style: 50816,
            table: 13025,
            inputs: 17778,
        },
        (43, 9) => Sizes {
            style: 57223,
            table: 14668,
            inputs: 20014,
        },
        (43, 10) => Sizes {
            style: 63630,
            table: 16311,
            inputs: 22250,
        },
        (43, 11) => Sizes {
            style: 70037,
            table: 17954,
            inputs: 24486,
        },
        (43, 12) => Sizes {
            style: 76444,
            table: 19597,
            inputs: 26722,
        },
        (43, 13) => Sizes {
            style: 82851,
            table: 21240,
            inputs: 28958,
        },
        (43, 14) => Sizes {
            style: 89258,
            table: 22883,
            inputs: 31194,
        },
        (43, 15) => Sizes {
            style: 95665,
            table: 24526,
            inputs: 33430,
        },
        (43, 16) => Sizes {
            style: 102072,
            table: 26169,
            inputs: 35666,
        },
        (43, 17) => Sizes {
            style: 108479,
            table: 27812,
            inputs: 37902,
        },
        (43, 18) => Sizes {
            style: 114886,
            table: 29455,
            inputs: 40138,
        },
        (43, 19) => Sizes {
            style: 121293,
            table: 31098,
            inputs: 42374,
        },
        (43, 20) => Sizes {
            style: 127700,
            table: 32741,
            inputs: 44610,
        },
        (43, 21) => Sizes {
            style: 134107,
            table: 34384,
            inputs: 46846,
        },
        (43, 22) => Sizes {
            style: 140514,
            table: 36027,
            inputs: 49082,
        },
        (43, 23) => Sizes {
            style: 146921,
            table: 37670,
            inputs: 51318,
        },
        (43, 24) => Sizes {
            style: 153456,
            table: 39345,
            inputs: 53586,
        },
        (43, 25) => Sizes {
            style: 160035,
            table: 41031,
            inputs: 55865,
        },
        (43, 26) => Sizes {
            style: 166614,
            table: 42717,
            inputs: 58144,
        },
        (43, 27) => Sizes {
            style: 173193,
            table: 44403,
            inputs: 60423,
        },
        (43, 28) => Sizes {
            style: 179772,
            table: 46089,
            inputs: 62702,
        },
        (43, 29) => Sizes {
            style: 186351,
            table: 47775,
            inputs: 64981,
        },
        (43, 30) => Sizes {
            style: 192930,
            table: 49461,
            inputs: 67260,
        },
        (43, 31) => Sizes {
            style: 199509,
            table: 51147,
            inputs: 69539,
        },
        (43, 32) => Sizes {
            style: 206088,
            table: 52833,
            inputs: 71818,
        },
        (43, 33) => Sizes {
            style: 212667,
            table: 54519,
            inputs: 74097,
        },
        (43, 34) => Sizes {
            style: 219246,
            table: 56205,
            inputs: 76376,
        },
        (43, 35) => Sizes {
            style: 225825,
            table: 57891,
            inputs: 78655,
        },
        (43, 36) => Sizes {
            style: 232404,
            table: 59577,
            inputs: 80934,
        },
        (43, 37) => Sizes {
            style: 238983,
            table: 61263,
            inputs: 83213,
        },
        (43, 38) => Sizes {
            style: 245562,
            table: 62949,
            inputs: 85492,
        },
        (43, 39) => Sizes {
            style: 252141,
            table: 64635,
            inputs: 87771,
        },
        (43, 40) => Sizes {
            style: 258720,
            table: 66321,
            inputs: 90050,
        },
        (43, 41) => Sizes {
            style: 265299,
            table: 68007,
            inputs: 92329,
        },
        (43, 42) => Sizes {
            style: 271878,
            table: 69693,
            inputs: 94608,
        },
        (43, 43) => Sizes {
            style: 278457,
            table: 71379,
            inputs: 96887,
        },
        (43, 44) => Sizes {
            style: 285036,
            table: 73065,
            inputs: 99166,
        },
        (43, 45) => Sizes {
            style: 291615,
            table: 74751,
            inputs: 101445,
        },
        (43, 46) => Sizes {
            style: 298194,
            table: 76437,
            inputs: 103724,
        },
        (43, 47) => Sizes {
            style: 304773,
            table: 78123,
            inputs: 106003,
        },
        (43, 48) => Sizes {
            style: 311352,
            table: 79809,
            inputs: 108282,
        },
        (43, 49) => Sizes {
            style: 317931,
            table: 81495,
            inputs: 110561,
        },
        (43, 50) => Sizes {
            style: 324510,
            table: 83181,
            inputs: 112840,
        },
        (44, 2) => Sizes {
            style: 12720,
            table: 3255,
            inputs: 4478,
        },
        (44, 3) => Sizes {
            style: 19228,
            table: 4924,
            inputs: 6754,
        },
        (44, 4) => Sizes {
            style: 25784,
            table: 6605,
            inputs: 9042,
        },
        (44, 5) => Sizes {
            style: 32340,
            table: 8286,
            inputs: 11330,
        },
        (44, 6) => Sizes {
            style: 38896,
            table: 9967,
            inputs: 13618,
        },
        (44, 7) => Sizes {
            style: 45452,
            table: 11648,
            inputs: 15906,
        },
        (44, 8) => Sizes {
            style: 52008,
            table: 13329,
            inputs: 18194,
        },
        (44, 9) => Sizes {
            style: 58564,
            table: 15010,
            inputs: 20482,
        },
        (44, 10) => Sizes {
            style: 65120,
            table: 16691,
            inputs: 22770,
        },
        (44, 11) => Sizes {
            style: 71676,
            table: 18372,
            inputs: 25058,
        },
        (44, 12) => Sizes {
            style: 78232,
            table: 20053,
            inputs: 27346,
        },
        (44, 13) => Sizes {
            style: 84788,
            table: 21734,
            inputs: 29634,
        },
        (44, 14) => Sizes {
            style: 91344,
            table: 23415,
            inputs: 31922,
        },
        (44, 15) => Sizes {
            style: 97900,
            table: 25096,
            inputs: 34210,
        },
        (44, 16) => Sizes {
            style: 104456,
            table: 26777,
            inputs: 36498,
        },
        (44, 17) => Sizes {
            style: 111012,
            table: 28458,
            inputs: 38786,
        },
        (44, 18) => Sizes {
            style: 117568,
            table: 30139,
            inputs: 41074,
        },
        (44, 19) => Sizes {
            style: 124124,
            table: 31820,
            inputs: 43362,
        },
        (44, 20) => Sizes {
            style: 130680,
            table: 33501,
            inputs: 45650,
        },
        (44, 21) => Sizes {
            style: 137236,
            table: 35182,
            inputs: 47938,
        },
        (44, 22) => Sizes {
            style: 143792,
            table: 36863,
            inputs: 50226,
        },
        (44, 23) => Sizes {
            style: 150396,
            table: 38556,
            inputs: 52526,
        },
        (44, 24) => Sizes {
            style: 157128,
            table: 40281,
            inputs: 54858,
        },
        (44, 25) => Sizes {
            style: 163860,
            table: 42006,
            inputs: 57190,
        },
        (44, 26) => Sizes {
            style: 170592,
            table: 43731,
            inputs: 59522,
        },
        (44, 27) => Sizes {
            style: 177324,
            table: 45456,
            inputs: 61854,
        },
        (44, 28) => Sizes {
            style: 184056,
            table: 47181,
            inputs: 64186,
        },
        (44, 29) => Sizes {
            style: 190788,
            table: 48906,
            inputs: 66518,
        },
        (44, 30) => Sizes {
            style: 197520,
            table: 50631,
            inputs: 68850,
        },
        (44, 31) => Sizes {
            style: 204252,
            table: 52356,
            inputs: 71182,
        },
        (44, 32) => Sizes {
            style: 210984,
            table: 54081,
            inputs: 73514,
        },
        (44, 33) => Sizes {
            style: 217716,
            table: 55806,
            inputs: 75846,
        },
        (44, 34) => Sizes {
            style: 224448,
            table: 57531,
            inputs: 78178,
        },
        (44, 35) => Sizes {
            style: 231180,
            table: 59256,
            inputs: 80510,
        },
        (44, 36) => Sizes {
            style: 237912,
            table: 60981,
            inputs: 82842,
        },
        (44, 37) => Sizes {
            style: 244644,
            table: 62706,
            inputs: 85174,
        },
        (44, 38) => Sizes {
            style: 251376,
            table: 64431,
            inputs: 87506,
        },
        (44, 39) => Sizes {
            style: 258108,
            table: 66156,
            inputs: 89838,
        },
        (44, 40) => Sizes {
            style: 264840,
            table: 67881,
            inputs: 92170,
        },
        (44, 41) => Sizes {
            style: 271572,
            table: 69606,
            inputs: 94502,
        },
        (44, 42) => Sizes {
            style: 278304,
            table: 71331,
            inputs: 96834,
        },
        (44, 43) => Sizes {
            style: 285036,
            table: 73056,
            inputs: 99166,
        },
        (44, 44) => Sizes {
            style: 291768,
            table: 74781,
            inputs: 101498,
        },
        (44, 45) => Sizes {
            style: 298500,
            table: 76506,
            inputs: 103830,
        },
        (44, 46) => Sizes {
            style: 305232,
            table: 78231,
            inputs: 106162,
        },
        (44, 47) => Sizes {
            style: 311964,
            table: 79956,
            inputs: 108494,
        },
        (44, 48) => Sizes {
            style: 318696,
            table: 81681,
            inputs: 110826,
        },
        (44, 49) => Sizes {
            style: 325428,
            table: 83406,
            inputs: 113158,
        },
        (44, 50) => Sizes {
            style: 332160,
            table: 85131,
            inputs: 115490,
        },
        (45, 2) => Sizes {
            style: 13010,
            table: 3329,
            inputs: 4580,
        },
        (45, 3) => Sizes {
            style: 19675,
            table: 5038,
            inputs: 6910,
        },
        (45, 4) => Sizes {
            style: 26380,
            table: 6757,
            inputs: 9250,
        },
        (45, 5) => Sizes {
            style: 33085,
            table: 8476,
            inputs: 11590,
        },
        (45, 6) => Sizes {
            style: 39790,
            table: 10195,
            inputs: 13930,
        },
        (45, 7) => Sizes {
            style: 46495,
            table: 11914,
            inputs: 16270,
        },
        (45, 8) => Sizes {
            style: 53200,
            table: 13633,
            inputs: 18610,
        },
        (45, 9) => Sizes {
            style: 59905,
            table: 15352,
            inputs: 20950,
        },
        (45, 10) => Sizes {
            style: 66610,
            table: 17071,
            inputs: 23290,
        },
        (45, 11) => Sizes {
            style: 73315,
            table: 18790,
            inputs: 25630,
        },
        (45, 12) => Sizes {
            style: 80020,
            table: 20509,
            inputs: 27970,
        },
        (45, 13) => Sizes {
            style: 86725,
            table: 22228,
            inputs: 30310,
        },
        (45, 14) => Sizes {
            style: 93430,
            table: 23947,
            inputs: 32650,
        },
        (45, 15) => Sizes {
            style: 100135,
            table: 25666,
            inputs: 34990,
        },
        (45, 16) => Sizes {
            style: 106840,
            table: 27385,
            inputs: 37330,
        },
        (45, 17) => Sizes {
            style: 113545,
            table: 29104,
            inputs: 39670,
        },
        (45, 18) => Sizes {
            style: 120250,
            table: 30823,
            inputs: 42010,
        },
        (45, 19) => Sizes {
            style: 126955,
            table: 32542,
            inputs: 44350,
        },
        (45, 20) => Sizes {
            style: 133660,
            table: 34261,
            inputs: 46690,
        },
        (45, 21) => Sizes {
            style: 140365,
            table: 35980,
            inputs: 49030,
        },
        (45, 22) => Sizes {
            style: 147070,
            table: 37699,
            inputs: 51370,
        },
        (45, 23) => Sizes {
            style: 153915,
            table: 39453,
            inputs: 53745,
        },
        (45, 24) => Sizes {
            style: 160800,
            table: 41217,
            inputs: 56130,
        },
        (45, 25) => Sizes {
            style: 167685,
            table: 42981,
            inputs: 58515,
        },
        (45, 26) => Sizes {
            style: 174570,
            table: 44745,
            inputs: 60900,
        },
        (45, 27) => Sizes {
            style: 181455,
            table: 46509,
            inputs: 63285,
        },
        (45, 28) => Sizes {
            style: 188340,
            table: 48273,
            inputs: 65670,
        },
        (45, 29) => Sizes {
            style: 195225,
            table: 50037,
            inputs: 68055,
        },
        (45, 30) => Sizes {
            style: 202110,
            table: 51801,
            inputs: 70440,
        },
        (45, 31) => Sizes {
            style: 208995,
            table: 53565,
            inputs: 72825,
        },
        (45, 32) => Sizes {
            style: 215880,
            table: 55329,
            inputs: 75210,
        },
        (45, 33) => Sizes {
            style: 222765,
            table: 57093,
            inputs: 77595,
        },
        (45, 34) => Sizes {
            style: 229650,
            table: 58857,
            inputs: 79980,
        },
        (45, 35) => Sizes {
            style: 236535,
            table: 60621,
            inputs: 82365,
        },
        (45, 36) => Sizes {
            style: 243420,
            table: 62385,
            inputs: 84750,
        },
        (45, 37) => Sizes {
            style: 250305,
            table: 64149,
            inputs: 87135,
        },
        (45, 38) => Sizes {
            style: 257190,
            table: 65913,
            inputs: 89520,
        },
        (45, 39) => Sizes {
            style: 264075,
            table: 67677,
            inputs: 91905,
        },
        (45, 40) => Sizes {
            style: 270960,
            table: 69441,
            inputs: 94290,
        },
        (45, 41) => Sizes {
            style: 277845,
            table: 71205,
            inputs: 96675,
        },
        (45, 42) => Sizes {
            style: 284730,
            table: 72969,
            inputs: 99060,
        },
        (45, 43) => Sizes {
            style: 291615,
            table: 74733,
            inputs: 101445,
        },
        (45, 44) => Sizes {
            style: 298500,
            table: 76497,
            inputs: 103830,
        },
        (45, 45) => Sizes {
            style: 305385,
            table: 78261,
            inputs: 106215,
        },
        (45, 46) => Sizes {
            style: 312270,
            table: 80025,
            inputs: 108600,
        },
        (45, 47) => Sizes {
            style: 319155,
            table: 81789,
            inputs: 110985,
        },
        (45, 48) => Sizes {
            style: 326040,
            table: 83553,
            inputs: 113370,
        },
        (45, 49) => Sizes {
            style: 332925,
            table: 85317,
            inputs: 115755,
        },
        (45, 50) => Sizes {
            style: 339810,
            table: 87081,
            inputs: 118140,
        },
        (46, 2) => Sizes {
            style: 13300,
            table: 3403,
            inputs: 4682,
        },
        (46, 3) => Sizes {
            style: 20122,
            table: 5152,
            inputs: 7066,
        },
        (46, 4) => Sizes {
            style: 26976,
            table: 6909,
            inputs: 9458,
        },
        (46, 5) => Sizes {
            style: 33830,
            table: 8666,
            inputs: 11850,
        },
        (46, 6) => Sizes {
            style: 40684,
            table: 10423,
            inputs: 14242,
        },
        (46, 7) => Sizes {
            style: 47538,
            table: 12180,
            inputs: 16634,
        },
        (46, 8) => Sizes {
            style: 54392,
            table: 13937,
            inputs: 19026,
        },
        (46, 9) => Sizes {
            style: 61246,
            table: 15694,
            inputs: 21418,
        },
        (46, 10) => Sizes {
            style: 68100,
            table: 17451,
            inputs: 23810,
        },
        (46, 11) => Sizes {
            style: 74954,
            table: 19208,
            inputs: 26202,
        },
        (46, 12) => Sizes {
            style: 81808,
            table: 20965,
            inputs: 28594,
        },
        (46, 13) => Sizes {
            style: 88662,
            table: 22722,
            inputs: 30986,
        },
        (46, 14) => Sizes {
            style: 95516,
            table: 24479,
            inputs: 33378,
        },
        (46, 15) => Sizes {
            style: 102370,
            table: 26236,
            inputs: 35770,
        },
        (46, 16) => Sizes {
            style: 109224,
            table: 27993,
            inputs: 38162,
        },
        (46, 17) => Sizes {
            style: 116078,
            table: 29750,
            inputs: 40554,
        },
        (46, 18) => Sizes {
            style: 122932,
            table: 31507,
            inputs: 42946,
        },
        (46, 19) => Sizes {
            style: 129786,
            table: 33264,
            inputs: 45338,
        },
        (46, 20) => Sizes {
            style: 136640,
            table: 35021,
            inputs: 47730,
        },
        (46, 21) => Sizes {
            style: 143494,
            table: 36778,
            inputs: 50122,
        },
        (46, 22) => Sizes {
            style: 150396,
            table: 38547,
            inputs: 52526,
        },
        (46, 23) => Sizes {
            style: 157434,
            table: 40350,
            inputs: 54964,
        },
        (46, 24) => Sizes {
            style: 164472,
            table: 42153,
            inputs: 57402,
        },
        (46, 25) => Sizes {
            style: 171510,
            table: 43956,
            inputs: 59840,
        },
        (46, 26) => Sizes {
            style: 178548,
            table: 45759,
            inputs: 62278,
        },
        (46, 27) => Sizes {
            style: 185586,
            table: 47562,
            inputs: 64716,
        },
        (46, 28) => Sizes {
            style: 192624,
            table: 49365,
            inputs: 67154,
        },
        (46, 29) => Sizes {
            style: 199662,
            table: 51168,
            inputs: 69592,
        },
        (46, 30) => Sizes {
            style: 206700,
            table: 52971,
            inputs: 72030,
        },
        (46, 31) => Sizes {
            style: 213738,
            table: 54774,
            inputs: 74468,
        },
        (46, 32) => Sizes {
            style: 220776,
            table: 56577,
            inputs: 76906,
        },
        (46, 33) => Sizes {
            style: 227814,
            table: 58380,
            inputs: 79344,
        },
        (46, 34) => Sizes {
            style: 234852,
            table: 60183,
            inputs: 81782,
        },
        (46, 35) => Sizes {
            style: 241890,
            table: 61986,
            inputs: 84220,
        },
        (46, 36) => Sizes {
            style: 248928,
            table: 63789,
            inputs: 86658,
        },
        (46, 37) => Sizes {
            style: 255966,
            table: 65592,
            inputs: 89096,
        },
        (46, 38) => Sizes {
            style: 263004,
            table: 67395,
            inputs: 91534,
        },
        (46, 39) => Sizes {
            style: 270042,
            table: 69198,
            inputs: 93972,
        },
        (46, 40) => Sizes {
            style: 277080,
            table: 71001,
            inputs: 96410,
        },
        (46, 41) => Sizes {
            style: 284118,
            table: 72804,
            inputs: 98848,
        },
        (46, 42) => Sizes {
            style: 291156,
            table: 74607,
            inputs: 101286,
        },
        (46, 43) => Sizes {
            style: 298194,
            table: 76410,
            inputs: 103724,
        },
        (46, 44) => Sizes {
            style: 305232,
            table: 78213,
            inputs: 106162,
        },
        (46, 45) => Sizes {
            style: 312270,
            table: 80016,
            inputs: 108600,
        },
        (46, 46) => Sizes {
            style: 319308,
            table: 81819,
            inputs: 111038,
        },
        (46, 47) => Sizes {
            style: 326346,
            table: 83622,
            inputs: 113476,
        },
        (46, 48) => Sizes {
            style: 333384,
            table: 85425,
            inputs: 115914,
        },
        (46, 49) => Sizes {
            style: 340422,
            table: 87228,
            inputs: 118352,
        },
        (46, 50) => Sizes {
            style: 347460,
            table: 89031,
            inputs: 120790,
        },
        (47, 2) => Sizes {
            style: 13590,
            table: 3477,
            inputs: 4784,
        },
        (47, 3) => Sizes {
            style: 20569,
            table: 5266,
            inputs: 7222,
        },
        (47, 4) => Sizes {
            style: 27572,
            table: 7061,
            inputs: 9666,
        },
        (47, 5) => Sizes {
            style: 34575,
            table: 8856,
            inputs: 12110,
        },
        (47, 6) => Sizes {
            style: 41578,
            table: 10651,
            inputs: 14554,
        },
        (47, 7) => Sizes {
            style: 48581,
            table: 12446,
            inputs: 16998,
        },
        (47, 8) => Sizes {
            style: 55584,
            table: 14241,
            inputs: 19442,
        },
        (47, 9) => Sizes {
            style: 62587,
            table: 16036,
            inputs: 21886,
        },
        (47, 10) => Sizes {
            style: 69590,
            table: 17831,
            inputs: 24330,
        },
        (47, 11) => Sizes {
            style: 76593,
            table: 19626,
            inputs: 26774,
        },
        (47, 12) => Sizes {
            style: 83596,
            table: 21421,
            inputs: 29218,
        },
        (47, 13) => Sizes {
            style: 90599,
            table: 23216,
            inputs: 31662,
        },
        (47, 14) => Sizes {
            style: 97602,
            table: 25011,
            inputs: 34106,
        },
        (47, 15) => Sizes {
            style: 104605,
            table: 26806,
            inputs: 36550,
        },
        (47, 16) => Sizes {
            style: 111608,
            table: 28601,
            inputs: 38994,
        },
        (47, 17) => Sizes {
            style: 118611,
            table: 30396,
            inputs: 41438,
        },
        (47, 18) => Sizes {
            style: 125614,
            table: 32191,
            inputs: 43882,
        },
        (47, 19) => Sizes {
            style: 132617,
            table: 33986,
            inputs: 46326,
        },
        (47, 20) => Sizes {
            style: 139620,
            table: 35781,
            inputs: 48770,
        },
        (47, 21) => Sizes {
            style: 146623,
            table: 37576,
            inputs: 51214,
        },
        (47, 22) => Sizes {
            style: 153762,
            table: 39405,
            inputs: 53692,
        },
        (47, 23) => Sizes {
            style: 160953,
            table: 41247,
            inputs: 56183,
        },
        (47, 24) => Sizes {
            style: 168144,
            table: 43089,
            inputs: 58674,
        },
        (47, 25) => Sizes {
            style: 175335,
            table: 44931,
            inputs: 61165,
        },
        (47, 26) => Sizes {
            style: 182526,
            table: 46773,
            inputs: 63656,
        },
        (47, 27) => Sizes {
            style: 189717,
            table: 48615,
            inputs: 66147,
        },
        (47, 28) => Sizes {
            style: 196908,
            table: 50457,
            inputs: 68638,
        },
        (47, 29) => Sizes {
            style: 204099,
            table: 52299,
            inputs: 71129,
        },
        (47, 30) => Sizes {
            style: 211290,
            table: 54141,
            inputs: 73620,
        },
        (47, 31) => Sizes {
            style: 218481,
            table: 55983,
            inputs: 76111,
        },
        (47, 32) => Sizes {
            style: 225672,
            table: 57825,
            inputs: 78602,
        },
        (47, 33) => Sizes {
            style: 232863,
            table: 59667,
            inputs: 81093,
        },
        (47, 34) => Sizes {
            style: 240054,
            table: 61509,
            inputs: 83584,
        },
        (47, 35) => Sizes {
            style: 247245,
            table: 63351,
            inputs: 86075,
        },
        (47, 36) => Sizes {
            style: 254436,
            table: 65193,
            inputs: 88566,
        },
        (47, 37) => Sizes {
            style: 261627,
            table: 67035,
            inputs: 91057,
        },
        (47, 38) => Sizes {
            style: 268818,
            table: 68877,
            inputs: 93548,
        },
        (47, 39) => Sizes {
            style: 276009,
            table: 70719,
            inputs: 96039,
        },
        (47, 40) => Sizes {
            style: 283200,
            table: 72561,
            inputs: 98530,
        },
        (47, 41) => Sizes {
            style: 290391,
            table: 74403,
            inputs: 101021,
        },
        (47, 42) => Sizes {
            style: 297582,
            table: 76245,
            inputs: 103512,
        },
        (47, 43) => Sizes {
            style: 304773,
            table: 78087,
            inputs: 106003,
        },
        (47, 44) => Sizes {
            style: 311964,
            table: 79929,
            inputs: 108494,
        },
        (47, 45) => Sizes {
            style: 319155,
            table: 81771,
            inputs: 110985,
        },
        (47, 46) => Sizes {
            style: 326346,
            table: 83613,
            inputs: 113476,
        },
        (47, 47) => Sizes {
            style: 333537,
            table: 85455,
            inputs: 115967,
        },
        (47, 48) => Sizes {
            style: 340728,
            table: 87297,
            inputs: 118458,
        },
        (47, 49) => Sizes {
            style: 347919,
            table: 89139,
            inputs: 120949,
        },
        (47, 50) => Sizes {
            style: 355110,
            table: 90981,
            inputs: 123440,
        },
        (48, 2) => Sizes {
            style: 13880,
            table: 3551,
            inputs: 4886,
        },
        (48, 3) => Sizes {
            style: 21016,
            table: 5380,
            inputs: 7378,
        },
        (48, 4) => Sizes {
            style: 28168,
            table: 7213,
            inputs: 9874,
        },
        (48, 5) => Sizes {
            style: 35320,
            table: 9046,
            inputs: 12370,
        },
        (48, 6) => Sizes {
            style: 42472,
            table: 10879,
            inputs: 14866,
        },
        (48, 7) => Sizes {
            style: 49624,
            table: 12712,
            inputs: 17362,
        },
        (48, 8) => Sizes {
            style: 56776,
            table: 14545,
            inputs: 19858,
        },
        (48, 9) => Sizes {
            style: 63928,
            table: 16378,
            inputs: 22354,
        },
        (48, 10) => Sizes {
            style: 71080,
            table: 18211,
            inputs: 24850,
        },
        (48, 11) => Sizes {
            style: 78232,
            table: 20044,
            inputs: 27346,
        },
        (48, 12) => Sizes {
            style: 85384,
            table: 21877,
            inputs: 29842,
        },
        (48, 13) => Sizes {
            style: 92536,
            table: 23710,
            inputs: 32338,
        },
        (48, 14) => Sizes {
            style: 99688,
            table: 25543,
            inputs: 34834,
        },
        (48, 15) => Sizes {
            style: 106840,
            table: 27376,
            inputs: 37330,
        },
        (48, 16) => Sizes {
            style: 113992,
            table: 29209,
            inputs: 39826,
        },
        (48, 17) => Sizes {
            style: 121144,
            table: 31042,
            inputs: 42322,
        },
        (48, 18) => Sizes {
            style: 128296,
            table: 32875,
            inputs: 44818,
        },
        (48, 19) => Sizes {
            style: 135448,
            table: 34708,
            inputs: 47314,
        },
        (48, 20) => Sizes {
            style: 142600,
            table: 36541,
            inputs: 49810,
        },
        (48, 21) => Sizes {
            style: 149784,
            table: 38382,
            inputs: 52314,
        },
        (48, 22) => Sizes {
            style: 157128,
            table: 40263,
            inputs: 54858,
        },
        (48, 23) => Sizes {
            style: 164472,
            table: 42144,
            inputs: 57402,
        },
        (48, 24) => Sizes {
            style: 171816,
            table: 44025,
            inputs: 59946,
        },
        (48, 25) => Sizes {
            style: 179160,
            table: 45906,
            inputs: 62490,
        },
        (48, 26) => Sizes {
            style: 186504,
            table: 47787,
            inputs: 65034,
        },
        (48, 27) => Sizes {
            style: 193848,
            table: 49668,
            inputs: 67578,
        },
        (48, 28) => Sizes {
            style: 201192,
            table: 51549,
            inputs: 70122,
        },
        (48, 29) => Sizes {
            style: 208536,
            table: 53430,
            inputs: 72666,
        },
        (48, 30) => Sizes {
            style: 215880,
            table: 55311,
            inputs: 75210,
        },
        (48, 31) => Sizes {
            style: 223224,
            table: 57192,
            inputs: 77754,
        },
        (48, 32) => Sizes {
            style: 230568,
            table: 59073,
            inputs: 80298,
        },
        (48, 33) => Sizes {
            style: 237912,
            table: 60954,
            inputs: 82842,
        },
        (48, 34) => Sizes {
            style: 245256,
            table: 62835,
            inputs: 85386,
        },
        (48, 35) => Sizes {
            style: 252600,
            table: 64716,
            inputs: 87930,
        },
        (48, 36) => Sizes {
            style: 259944,
            table: 66597,
            inputs: 90474,
        },
        (48, 37) => Sizes {
            style: 267288,
            table: 68478,
            inputs: 93018,
        },
        (48, 38) => Sizes {
            style: 274632,
            table: 70359,
            inputs: 95562,
        },
        (48, 39) => Sizes {
            style: 281976,
            table: 72240,
            inputs: 98106,
        },
        (48, 40) => Sizes {
            style: 289320,
            table: 74121,
            inputs: 100650,
        },
        (48, 41) => Sizes {
            style: 296664,
            table: 76002,
            inputs: 103194,
        },
        (48, 42) => Sizes {
            style: 304008,
            table: 77883,
            inputs: 105738,
        },
        (48, 43) => Sizes {
            style: 311352,
            table: 79764,
            inputs: 108282,
        },
        (48, 44) => Sizes {
            style: 318696,
            table: 81645,
            inputs: 110826,
        },
        (48, 45) => Sizes {
            style: 326040,
            table: 83526,
            inputs: 113370,
        },
        (48, 46) => Sizes {
            style: 333384,
            table: 85407,
            inputs: 115914,
        },
        (48, 47) => Sizes {
            style: 340728,
            table: 87288,
            inputs: 118458,
        },
        (48, 48) => Sizes {
            style: 348072,
            table: 89169,
            inputs: 121002,
        },
        (48, 49) => Sizes {
            style: 355416,
            table: 91050,
            inputs: 123546,
        },
        (48, 50) => Sizes {
            style: 362760,
            table: 92931,
            inputs: 126090,
        },
        (49, 2) => Sizes {
            style: 14170,
            table: 3625,
            inputs: 4988,
        },
        (49, 3) => Sizes {
            style: 21463,
            table: 5494,
            inputs: 7534,
        },
        (49, 4) => Sizes {
            style: 28764,
            table: 7365,
            inputs: 10082,
        },
        (49, 5) => Sizes {
            style: 36065,
            table: 9236,
            inputs: 12630,
        },
        (49, 6) => Sizes {
            style: 43366,
            table: 11107,
            inputs: 15178,
        },
        (49, 7) => Sizes {
            style: 50667,
            table: 12978,
            inputs: 17726,
        },
        (49, 8) => Sizes {
            style: 57968,
            table: 14849,
            inputs: 20274,
        },
        (49, 9) => Sizes {
            style: 65269,
            table: 16720,
            inputs: 22822,
        },
        (49, 10) => Sizes {
            style: 72570,
            table: 18591,
            inputs: 25370,
        },
        (49, 11) => Sizes {
            style: 79871,
            table: 20462,
            inputs: 27918,
        },
        (49, 12) => Sizes {
            style: 87172,
            table: 22333,
            inputs: 30466,
        },
        (49, 13) => Sizes {
            style: 94473,
            table: 24204,
            inputs: 33014,
        },
        (49, 14) => Sizes {
            style: 101774,
            table: 26075,
            inputs: 35562,
        },
        (49, 15) => Sizes {
            style: 109075,
            table: 27946,
            inputs: 38110,
        },
        (49, 16) => Sizes {
            style: 116376,
            table: 29817,
            inputs: 40658,
        },
        (49, 17) => Sizes {
            style: 123677,
            table: 31688,
            inputs: 43206,
        },
        (49, 18) => Sizes {
            style: 130978,
            table: 33559,
            inputs: 45754,
        },
        (49, 19) => Sizes {
            style: 138279,
            table: 35430,
            inputs: 48302,
        },
        (49, 20) => Sizes {
            style: 145580,
            table: 37301,
            inputs: 50850,
        },
        (49, 21) => Sizes {
            style: 152997,
            table: 39201,
            inputs: 53427,
        },
        (49, 22) => Sizes {
            style: 160494,
            table: 41121,
            inputs: 56024,
        },
        (49, 23) => Sizes {
            style: 167991,
            table: 43041,
            inputs: 58621,
        },
        (49, 24) => Sizes {
            style: 175488,
            table: 44961,
            inputs: 61218,
        },
        (49, 25) => Sizes {
            style: 182985,
            table: 46881,
            inputs: 63815,
        },
        (49, 26) => Sizes {
            style: 190482,
            table: 48801,
            inputs: 66412,
        },
        (49, 27) => Sizes {
            style: 197979,
            table: 50721,
            inputs: 69009,
        },
        (49, 28) => Sizes {
            style: 205476,
            table: 52641,
            inputs: 71606,
        },
        (49, 29) => Sizes {
            style: 212973,
            table: 54561,
            inputs: 74203,
        },
        (49, 30) => Sizes {
            style: 220470,
            table: 56481,
            inputs: 76800,
        },
        (49, 31) => Sizes {
            style: 227967,
            table: 58401,
            inputs: 79397,
        },
        (49, 32) => Sizes {
            style: 235464,
            table: 60321,
            inputs: 81994,
        },
        (49, 33) => Sizes {
            style: 242961,
            table: 62241,
            inputs: 84591,
        },
        (49, 34) => Sizes {
            style: 250458,
            table: 64161,
            inputs: 87188,
        },
        (49, 35) => Sizes {
            style: 257955,
            table: 66081,
            inputs: 89785,
        },
        (49, 36) => Sizes {
            style: 265452,
            table: 68001,
            inputs: 92382,
        },
        (49, 37) => Sizes {
            style: 272949,
            table: 69921,
            inputs: 94979,
        },
        (49, 38) => Sizes {
            style: 280446,
            table: 71841,
            inputs: 97576,
        },
        (49, 39) => Sizes {
            style: 287943,
            table: 73761,
            inputs: 100173,
        },
        (49, 40) => Sizes {
            style: 295440,
            table: 75681,
            inputs: 102770,
        },
        (49, 41) => Sizes {
            style: 302937,
            table: 77601,
            inputs: 105367,
        },
        (49, 42) => Sizes {
            style: 310434,
            table: 79521,
            inputs: 107964,
        },
        (49, 43) => Sizes {
            style: 317931,
            table: 81441,
            inputs: 110561,
        },
        (49, 44) => Sizes {
            style: 325428,
            table: 83361,
            inputs: 113158,
        },
        (49, 45) => Sizes {
            style: 332925,
            table: 85281,
            inputs: 115755,
        },
        (49, 46) => Sizes {
            style: 340422,
            table: 87201,
            inputs: 118352,
        },
        (49, 47) => Sizes {
            style: 347919,
            table: 89121,
            inputs: 120949,
        },
        (49, 48) => Sizes {
            style: 355416,
            table: 91041,
            inputs: 123546,
        },
        (49, 49) => Sizes {
            style: 362913,
            table: 92961,
            inputs: 126143,
        },
        (49, 50) => Sizes {
            style: 370410,
            table: 94881,
            inputs: 128740,
        },
        (50, 2) => Sizes {
            style: 14460,
            table: 3699,
            inputs: 5090,
        },
        (50, 3) => Sizes {
            style: 21910,
            table: 5608,
            inputs: 7690,
        },
        (50, 4) => Sizes {
            style: 29360,
            table: 7517,
            inputs: 10290,
        },
        (50, 5) => Sizes {
            style: 36810,
            table: 9426,
            inputs: 12890,
        },
        (50, 6) => Sizes {
            style: 44260,
            table: 11335,
            inputs: 15490,
        },
        (50, 7) => Sizes {
            style: 51710,
            table: 13244,
            inputs: 18090,
        },
        (50, 8) => Sizes {
            style: 59160,
            table: 15153,
            inputs: 20690,
        },
        (50, 9) => Sizes {
            style: 66610,
            table: 17062,
            inputs: 23290,
        },
        (50, 10) => Sizes {
            style: 74060,
            table: 18971,
            inputs: 25890,
        },
        (50, 11) => Sizes {
            style: 81510,
            table: 20880,
            inputs: 28490,
        },
        (50, 12) => Sizes {
            style: 88960,
            table: 22789,
            inputs: 31090,
        },
        (50, 13) => Sizes {
            style: 96410,
            table: 24698,
            inputs: 33690,
        },
        (50, 14) => Sizes {
            style: 103860,
            table: 26607,
            inputs: 36290,
        },
        (50, 15) => Sizes {
            style: 111310,
            table: 28516,
            inputs: 38890,
        },
        (50, 16) => Sizes {
            style: 118760,
            table: 30425,
            inputs: 41490,
        },
        (50, 17) => Sizes {
            style: 126210,
            table: 32334,
            inputs: 44090,
        },
        (50, 18) => Sizes {
            style: 133660,
            table: 34243,
            inputs: 46690,
        },
        (50, 19) => Sizes {
            style: 141110,
            table: 36152,
            inputs: 49290,
        },
        (50, 20) => Sizes {
            style: 148560,
            table: 38061,
            inputs: 51890,
        },
        (50, 21) => Sizes {
            style: 156210,
            table: 40020,
            inputs: 54540,
        },
        (50, 22) => Sizes {
            style: 163860,
            table: 41979,
            inputs: 57190,
        },
        (50, 23) => Sizes {
            style: 171510,
            table: 43938,
            inputs: 59840,
        },
        (50, 24) => Sizes {
            style: 179160,
            table: 45897,
            inputs: 62490,
        },
        (50, 25) => Sizes {
            style: 186810,
            table: 47856,
            inputs: 65140,
        },
        (50, 26) => Sizes {
            style: 194460,
            table: 49815,
            inputs: 67790,
        },
        (50, 27) => Sizes {
            style: 202110,
            table: 51774,
            inputs: 70440,
        },
        (50, 28) => Sizes {
            style: 209760,
            table: 53733,
            inputs: 73090,
        },
        (50, 29) => Sizes {
            style: 217410,
            table: 55692,
            inputs: 75740,
        },
        (50, 30) => Sizes {
            style: 225060,
            table: 57651,
            inputs: 78390,
        },
        (50, 31) => Sizes {
            style: 232710,
            table: 59610,
            inputs: 81040,
        },
        (50, 32) => Sizes {
            style: 240360,
            table: 61569,
            inputs: 83690,
        },
        (50, 33) => Sizes {
            style: 248010,
            table: 63528,
            inputs: 86340,
        },
        (50, 34) => Sizes {
            style: 255660,
            table: 65487,
            inputs: 88990,
        },
        (50, 35) => Sizes {
            style: 263310,
            table: 67446,
            inputs: 91640,
        },
        (50, 36) => Sizes {
            style: 270960,
            table: 69405,
            inputs: 94290,
        },
        (50, 37) => Sizes {
            style: 278610,
            table: 71364,
            inputs: 96940,
        },
        (50, 38) => Sizes {
            style: 286260,
            table: 73323,
            inputs: 99590,
        },
        (50, 39) => Sizes {
            style: 293910,
            table: 75282,
            inputs: 102240,
        },
        (50, 40) => Sizes {
            style: 301560,
            table: 77241,
            inputs: 104890,
        },
        (50, 41) => Sizes {
            style: 309210,
            table: 79200,
            inputs: 107540,
        },
        (50, 42) => Sizes {
            style: 316860,
            table: 81159,
            inputs: 110190,
        },
        (50, 43) => Sizes {
            style: 324510,
            table: 83118,
            inputs: 112840,
        },
        (50, 44) => Sizes {
            style: 332160,
            table: 85077,
            inputs: 115490,
        },
        (50, 45) => Sizes {
            style: 339810,
            table: 87036,
            inputs: 118140,
        },
        (50, 46) => Sizes {
            style: 347460,
            table: 88995,
            inputs: 120790,
        },
        (50, 47) => Sizes {
            style: 355110,
            table: 90954,
            inputs: 123440,
        },
        (50, 48) => Sizes {
            style: 362760,
            table: 92913,
            inputs: 126090,
        },
        (50, 49) => Sizes {
            style: 370410,
            table: 94872,
            inputs: 128740,
        },
        (50, 50) => Sizes {
            style: 378060,
            table: 96831,
            inputs: 131390,
        },
        _ => Sizes {
            style: 0,
            table: 0,
            inputs: 0,
        },
    }
}
