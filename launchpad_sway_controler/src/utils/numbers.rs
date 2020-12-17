pub static NUMBERS: &[[[bool; 8]; 8]; 10] = &[
    //0
    [
        [false, false, false, false, false, false, false, false],
        [false, false, true, true, true, true, false, false],
        [false, false, true, false, false, true, false, false],
        [false, false, true, false, false, true, false, false],
        [false, false, true, false, false, true, false, false],
        [false, false, true, false, false, true, false, false],
        [false, false, true, true, true, true, false, false],
        [false, false, false, false, false, false, false, false],
    ],
    //1
    [
        [false, false, false, false, false, false, false, false],
        [false, false, false, false, false, true, false, false],
        [false, false, false, false, false, true, false, false],
        [false, false, false, false, false, true, false, false],
        [false, false, false, false, false, true, false, false],
        [false, false, false, false, false, true, false, false],
        [false, false, false, false, false, true, false, false],
        [false, false, false, false, false, false, false, false],
    ],
    //2
    [
        [false, false, false, false, false, false, false, false],
        [false, true, true, true, true, true, true, false],
        [false, false, false, false, false, false, true, false],
        [false, false, false, false, false, false, true, false],
        [false, true, true, true, true, true, true, false],
        [false, true, false, false, false, false, false, false],
        [false, true, true, true, true, true, true, false],
        [false, false, false, false, false, false, false, false],
    ],
    //3
    [
        [false, false, false, false, false, false, false, false],
        [false, true, true, true, true, true, true, false],
        [false, false, false, false, false, false, true, false],
        [false, false, false, false, false, false, true, false],
        [false, true, true, true, true, true, true, false],
        [false, false, false, false, false, false, true, false],
        [false, false, false, false, false, false, true, false],
        [false, true, true, true, true, true, true, false],
    ],
    //4
    [
        [false, false, false, false, false, false, false, false],
        [false, true, false, false, false, false, true, false],
        [false, true, false, false, false, false, true, false],
        [false, true, false, false, false, false, true, false],
        [false, true, true, true, true, true, true, false],
        [false, false, false, false, false, false, true, false],
        [false, false, false, false, false, false, true, false],
        [false, false, false, false, false, false, false, false],
    ],
    //5
    [
        [false, false, false, false, false, false, false, false],
        [false, true, true, true, true, true, true, false],
        [false, true, false, false, false, false, false, false],
        [false, true, false, false, false, false, false, false],
        [false, true, true, true, true, true, true, false],
        [false, false, false, false, false, false, true, false],
        [false, true, true, true, true, true, true, false],
        [false, false, false, false, false, false, false, false],
    ],
    //6
    [
        [false, false, false, false, false, false, false, false],
        [false, true, true, true, true, true, true, false],
        [false, true, false, false, false, false, false, false],
        [false, true, false, false, false, false, false, false],
        [false, true, true, true, true, true, true, false],
        [false, true, false, false, false, false, true, false],
        [false, true, true, true, true, true, true, false],
        [false, false, false, false, false, false, false, false],
    ],
    //7
    [
        [false, false, false, false, false, false, false, false],
        [false, true, true, true, true, true, true, false],
        [false, false, false, false, false, false, true, false],
        [false, false, false, false, false, false, true, false],
        [false, false, false, false, false, false, true, false],
        [false, false, false, false, false, false, true, false],
        [false, false, false, false, false, false, true, false],
        [false, false, false, false, false, false, false, false],
    ],
    //8
    [
        [false, false, false, false, false, false, false, false],
        [false, true, true, true, true, true, true, false],
        [false, true, false, false, false, false, true, false],
        [false, true, false, false, false, false, true, false],
        [false, true, true, true, true, true, true, false],
        [false, true, false, false, false, false, true, false],
        [false, true, false, false, false, false, true, false],
        [false, true, true, true, true, true, true, false],
    ],
    //9
    [
        [false, false, false, false, false, false, false, false],
        [false, true, true, true, true, true, true, false],
        [false, true, false, false, false, false, true, false],
        [false, true, false, false, false, false, true, false],
        [false, true, true, true, true, true, true, false],
        [false, false, false, false, false, false, true, false],
        [false, false, false, false, false, false, true, false],
        [false, false, false, false, false, false, false, false],
    ],
];

pub static INNER_KEYS: &[[i64; 8]; 8] = &[
    [0, 1, 2, 3, 4, 5, 6, 7],
    [16, 17, 18, 19, 20, 21, 22, 23],
    [32, 33, 34, 35, 36, 37, 38, 39],
    [48, 49, 50, 51, 52, 53, 54, 55],
    [64, 65, 66, 67, 68, 69, 70, 71],
    [80, 81, 82, 83, 84, 85, 86, 87],
    [96, 97, 98, 99, 100, 101, 102, 103],
    [12, 113, 114, 115, 116, 117, 118, 119],
];
