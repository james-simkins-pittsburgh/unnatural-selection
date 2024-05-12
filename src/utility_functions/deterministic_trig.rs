/* This module exists to produce deterministic trig without using floating point
since any floating point math could break determinism.*/

use bevy::prelude::*;

/* These arrays hold precaulcated results for trig functions. This guaranteeds deterinism
across different implementations of floating point arithmetic */
#[derive(Resource)]
pub struct TrigArrays {
    pub sine_array: [i32; 360],
    pub cosine_array: [i32; 360],
    pub tangent_array: [i32; 360],
    pub arc_sine_array_by_thousandths: [i16; 2001],
    pub arc_cosine_array_by_thousandths: [i16; 2001],
    pub arc_tan_by_ones: [i16; 201],
    pub arc_tan_by_tenths: [i16; 201],
    pub arc_tan_by_hundreths: [i16; 201],
}
// These are the functions for getting trig results.
pub fn sine_times_1000(angle: i16, sine_array: [i32; 360]) -> i32 {
    return sine_array[normalize_angle(angle) as usize];
}

pub fn cosine_times_1000(angle: i16, cosine_array: [i32; 360]) -> i32 {
    return cosine_array[normalize_angle(angle) as usize];
}

pub fn tangent_times_1000(angle: i16, tangent_array: [i32; 360]) -> i32 {
    return tangent_array[normalize_angle(angle) as usize];
}

pub fn arc_sine_of_thousandths(proportion_out_of_1000: i32, arcsine_array: [i16; 2001]) -> i16 {
    // The first two conditions are error conditions.
    if proportion_out_of_1000 < -1000 {
        warn!("Arcsine below allowed domain!");
        return -90;
    } else if proportion_out_of_1000 > 1000 {
        warn!("Arcsine above allowed domain!");
        return 90;
    } else {
        // This is the code that is meant to run.
        return arcsine_array[(proportion_out_of_1000 as usize) + 1000];
    }
}

pub fn arc_cosine_of_thousandths(proportion_out_of_1000: i32, arccosine_array: [i16; 2001]) -> i16 {
    // The first two conditions are error conditions.
    if proportion_out_of_1000 > 1000 {
        warn!("Arccosine above allowed domain!");
        return 180;
    } else if proportion_out_of_1000 < -1000 {
        warn!("Arccosine below allowed domain!");
        return 0;
    } else {
        // This is the code that is meant to run.
        return arccosine_array[(proportion_out_of_1000 as usize) + 1000];
    }
}

pub fn arc_tangent_of_hundreths(
    proportion_out_of_100: i32,
    arc_tan_by_ones: [i16; 201],
    arc_tan_by_tenths: [i16; 201],
    arc_tan_by_hundreths: [i16; 201]
) -> i16 {

    if proportion_out_of_100 < -8935 {

        return -90 }
        
        else if proportion_out_of_100 > 8935 {

        return 90

        } else {

            


        }
    }


}

// This takes any angle and normalizes it to 0 to 359.: []
fn normalize_angle(argument_angle: i16) -> i16 {
    let mut normal_angle = argument_angle;

    if normal_angle > 359 {
        normal_angle = normal_angle % 360;
    } else if normal_angle < 0 {
        if normal_angle < -359 {
            normal_angle = normal_angle % -360;
        }
        normal_angle = normal_angle + 360;
    }

    return normal_angle;
}

/* This sets up the basic trig arrays to allow the functions to work.
These arrays have to be passed to the functions because they are a resource
in the Bevy app, but the functions are not systems. */
pub fn initialize_deterministic_trig(mut trig_arrays: ResMut<TrigArrays>) {
    trig_arrays.sine_array = [
        0, 17, 35, 52, 70, 87, 105, 122, 139, 156, 174, 191, 208, 225, 242, 259, 276, 292, 309, 326,
        342, 358, 375, 391, 407, 423, 438, 454, 469, 485, 500, 515, 530, 545, 559, 574, 588, 602,
        616, 629, 643, 656, 669, 682, 695, 707, 719, 731, 743, 755, 766, 777, 788, 799, 809, 819,
        829, 839, 848, 857, 866, 875, 883, 891, 899, 906, 914, 921, 927, 934, 940, 946, 951, 956,
        961, 966, 970, 974, 978, 982, 985, 988, 990, 993, 995, 996, 998, 999, 999, 1000, 1000, 1000,
        999, 999, 998, 996, 995, 993, 990, 988, 985, 982, 978, 974, 970, 966, 961, 956, 951, 946,
        940, 934, 927, 921, 914, 906, 899, 891, 883, 875, 866, 857, 848, 839, 829, 819, 809, 799,
        788, 777, 766, 755, 743, 731, 719, 707, 695, 682, 669, 656, 643, 629, 616, 602, 588, 574,
        559, 545, 530, 515, 500, 485, 469, 454, 438, 423, 407, 391, 375, 358, 342, 326, 309, 292,
        276, 259, 242, 225, 208, 191, 174, 156, 139, 122, 105, 87, 70, 52, 35, 17, 0, -17, -35, -52,
        -70, -87, -105, -122, -139, -156, -174, -191, -208, -225, -242, -259, -276, -292, -309, -326,
        -342, -358, -375, -391, -407, -423, -438, -454, -469, -485, -500, -515, -530, -545, -559, -574,
        -588, -602, -616, -629, -643, -656, -669, -682, -695, -707, -719, -731, -743, -755, -766, -777,
        -788, -799, -809, -819, -829, -839, -848, -857, -866, -875, -883, -891, -899, -906, -914, -921,
        -927, -934, -940, -946, -951, -956, -961, -966, -970, -974, -978, -982, -985, -988, -990, -993,
        -995, -996, -998, -999, -999, -1000, -1000, -1000, -999, -999, -998, -996, -995, -993,
        -990, -988, -985, -982, -978, -974, -970, -966, -961, -956, -951, -946, -940, -934, -927, -921,
        -914, -906, -899, -891, -883, -875, -866, -857, -848, -839, -829, -819, -809, -799, -788, -777,
        -766, -755, -743, -731, -719, -707, -695, -682, -669, -656, -643, -629, -616, -602, -588, -574,
        -559, -545, -530, -515, -500, -485, -469, -454, -438, -423, -407, -391, -375, -358, -342, -326,
        -309, -292, -276, -259, -242, -225, -208, -191, -174, -156, -139, -122, -105, -87, -70, -52,
        -35, -17,
    ];

    trig_arrays.cosine_array = [
        1000, 1000, 999, 999, 998, 996, 995, 993, 990, 988, 985, 982, 978, 974, 970, 966, 961, 956,
        951, 946, 940, 934, 927, 921, 914, 906, 899, 891, 883, 875, 866, 857, 848, 839, 829, 819,
        809, 799, 788, 777, 766, 755, 743, 731, 719, 707, 695, 682, 669, 656, 643, 629, 616, 602,
        588, 574, 559, 545, 530, 515, 500, 485, 469, 454, 438, 423, 407, 391, 375, 358, 342, 326,
        309, 292, 276, 259, 242, 225, 208, 191, 174, 156, 139, 122, 105, 87, 70, 52, 35, 17, 0, -17,
        -35, -52, -70, -87, -105, -122, -139, -156, -174, -191, -208, -225, -242, -259, -276, -292,
        -309, -326, -342, -358, -375, -391, -407, -423, -438, -454, -469, -485, -500, -515, -530, -545,
        -559, -574, -588, -602, -616, -629, -643, -656, -669, -682, -695, -707, -719, -731, -743, -755,
        -766, -777, -788, -799, -809, -819, -829, -839, -848, -857, -866, -875, -883, -891, -899, -906,
        -914, -921, -927, -934, -940, -946, -951, -956, -961, -966, -970, -974, -978, -982, -985, -988,
        -990, -993, -995, -996, -998, -999, -999, -1000, -1000, -1000, -999, -999, -998, -996,
        -995, -993, -990, -988, -985, -982, -978, -974, -970, -966, -961, -956, -951, -946, -940, -934,
        -927, -921, -914, -906, -899, -891, -883, -875, -866, -857, -848, -839, -829, -819, -809, -799,
        -788, -777, -766, -755, -743, -731, -719, -707, -695, -682, -669, -656, -643, -629, -616, -602,
        -588, -574, -559, -545, -530, -515, -500, -485, -469, -454, -438, -423, -407, -391, -375, -358,
        -342, -326, -309, -292, -276, -259, -242, -225, -208, -191, -174, -156, -139, -122, -105, -87,
        -70, -52, -35, -17, 0, 17, 35, 52, 70, 87, 105, 122, 139, 156, 174, 191, 208, 225, 242, 259,
        276, 292, 309, 326, 342, 358, 375, 391, 407, 423, 438, 454, 469, 485, 500, 515, 530, 545,
        559, 574, 588, 602, 616, 629, 643, 656, 669, 682, 695, 707, 719, 731, 743, 755, 766, 777,
        788, 799, 809, 819, 829, 839, 848, 857, 866, 875, 883, 891, 899, 906, 914, 921, 927, 934,
        940, 946, 951, 956, 961, 966, 970, 974, 978, 982, 985, 988, 990, 993, 995, 996, 998, 999,
        999, 1000,
    ];

    trig_arrays.tangent_array = [
        0, 17, 35, 52, 70, 87, 105, 123, 141, 158, 176, 194, 213, 231, 249, 268, 287, 306, 325, 344,
        364, 384, 404, 424, 445, 466, 488, 510, 532, 554, 577, 601, 625, 649, 675, 700, 727, 754,
        781, 810, 839, 869, 900, 933, 966, 1000, 1036, 1072, 1111, 1150, 1192, 1235, 1280, 1327,
        1376, 1428, 1483, 1540, 1600, 1664, 1732, 1804, 1881, 1963, 2050, 2145, 2246, 2356, 2475, 2605,
        2747, 2904, 3078, 3271, 3487, 3732, 4011, 4331, 4705, 5145, 5671, 6314, 7115, 8144, 9514, 11430,
        14301, 19081, 28637, 57292, -1563091456, -57288, -28636, -19081, -14301, -11430, -9514, -8144,
        -7115, -6314, -5671, -5145, -4705, -4331, -4011, -3732, -3487, -3271, -3078, -2904, -2747, -2605,
        -2475, -2356, -2246, -2145, -2050, -1963, -1881, -1804, -1732, -1664, -1600, -1540, -1483, -1428,
        -1376, -1327, -1280, -1235, -1192, -1150, -1111, -1072, -1036, -1000, -966, -933, -900, -869,
        -839, -810, -781, -754, -727, -700, -675, -649, -625, -601, -577, -554, -532, -510, -488, -466,
        -445, -424, -404, -384, -364, -344, -325, -306, -287, -268, -249, -231, -213, -194, -176, -158,
        -141, -123, -105, -87, -70, -52, -35, -17, 0, 17, 35, 52, 70, 87, 105, 123, 141, 158, 176, 194,
        213, 231, 249, 268, 287, 306, 325, 344, 364, 384, 404, 424, 445, 466, 488, 510, 532, 554,
        577, 601, 625, 649, 675, 700, 727, 754, 781, 810, 839, 869, 900, 933, 966, 1000, 1036, 1072,
        1111, 1150, 1192, 1235, 1280, 1327, 1376, 1428, 1483, 1540, 1600, 1664, 1732, 1804, 1881, 1963,
        2050, 2145, 2246, 2356, 2475, 2605, 2747, 2904, 3078, 3271, 3487, 3732, 4011, 4332, 4705, 5145,
        5671, 6314, 7115, 8144, 9515, 11430, 14301, 19082, 28638, 57297, -521030464, -57284,
        -28634, -19080, -14300, -11430, -9514, -8144, -7115, -6314, -5671, -5144, -4705, -4331,
        -4011, -3732, -3487, -3271, -3078, -2904, -2747, -2605, -2475, -2356, -2246, -2144, -2050, -1963,
        -1881, -1804, -1732, -1664, -1600, -1540, -1483, -1428, -1376, -1327, -1280, -1235, -1192, -1150,
        -1111, -1072, -1036, -1000, -966, -933, -900, -869, -839, -810, -781, -754, -727, -700,
        -675, -649, -625, -601, -577, -554, -532, -510, -488, -466, -445, -424, -404, -384, -364, -344,
        -325, -306, -287, -268, -249, -231, -213, -194, -176, -158, -141, -123, -105, -87, -70, -52,
        -35, -17,
    ];

    trig_arrays.arc_sine_array_by_thousandths = [
        -90, -87, -86, -86, -85, -84, -84, -83, -83, -82, -82, -81, -81, -81, -80, -80, -80, -79,
        -79, -79, -79, -78, -78, -78, -77, -77, -77, -77, -76, -76, -76, -76, -75, -75, -75, -75,
        -75, -74, -74, -74, -74, -74, -73, -73, -73, -73, -73, -72, -72, -72, -72, -72, -71, -71,
        -71, -71, -71, -71, -70, -70, -70, -70, -70, -70, -69, -69, -69, -69, -69, -69, -68, -68,
        -68, -68, -68, -68, -68, -67, -67, -67, -67, -67, -67, -66, -66, -66, -66, -66, -66, -66,
        -66, -65, -65, -65, -65, -65, -65, -65, -64, -64, -64, -64, -64, -64, -64, -64, -63, -63,
        -63, -63, -63, -63, -63, -62, -62, -62, -62, -62, -62, -62, -62, -62, -61, -61, -61, -61,
        -61, -61, -61, -61, -60, -60, -60, -60, -60, -60, -60, -60, -60, -59, -59, -59, -59, -59,
        -59, -59, -59, -59, -58, -58, -58, -58, -58, -58, -58, -58, -58, -57, -57, -57, -57, -57,
        -57, -57, -57, -57, -57, -56, -56, -56, -56, -56, -56, -56, -56, -56, -55, -55, -55, -55,
        -55, -55, -55, -55, -55, -55, -54, -54, -54, -54, -54, -54, -54, -54, -54, -54, -54, -53,
        -53, -53, -53, -53, -53, -53, -53, -53, -53, -52, -52, -52, -52, -52, -52, -52, -52, -52,
        -52, -52, -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, -50, -50, -50, -50, -50,
        -50, -50, -50, -50, -50, -50, -49, -49, -49, -49, -49, -49, -49, -49, -49, -49, -49, -49,
        -48, -48, -48, -48, -48, -48, -48, -48, -48, -48, -48, -47, -47, -47, -47, -47, -47, -47,
        -47, -47, -47, -47, -47, -46, -46, -46, -46, -46, -46, -46, -46, -46, -46, -46, -46, -45,
        -45, -45, -45, -45, -45, -45, -45, -45, -45, -45, -45, -45, -44, -44, -44, -44, -44, -44,
        -44, -44, -44, -44, -44, -44, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43,
        -43, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -41, -41, -41, -41,
        -41, -41, -41, -41, -41, -41, -41, -41, -41, -40, -40, -40, -40, -40, -40, -40, -40, -40,
        -40, -40, -40, -40, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39,
        -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -37, -37, -37, -37,
        -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -36, -36, -36, -36, -36, -36, -36, -36,
        -36, -36, -36, -36, -36, -36, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35,
        -35, -35, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -33,
        -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -32, -32, -32, -32, -32,
        -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -31, -31, -31, -31, -31, -31, -31, -31,
        -31, -31, -31, -31, -31, -31, -31, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30,
        -30, -30, -30, -30, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29,
        -29, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -27,
        -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -26, -26, -26, -26,
        -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -25, -25, -25, -25, -25, -25,
        -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -24, -24, -24, -24, -24, -24, -24, -24,
        -24, -24, -24, -24, -24, -24, -24, -24, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23,
        -23, -23, -23, -23, -23, -23, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22,
        -22, -22, -22, -22, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21,
        -21, -21, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20,
        -20, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -18,
        -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -17, -17,
        -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -16, -16, -16, -16,
        -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -15, -15, -15, -15, -15,
        -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -14, -14, -14, -14, -14, -14,
        -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -13, -13, -13, -13, -13, -13, -13,
        -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -12, -12, -12, -12, -12, -12, -12, -12,
        -12, -12, -12, -12, -12, -12, -12, -12, -12, -11, -11, -11, -11, -11, -11, -11, -11, -11,
        -11, -11, -11, -11, -11, -11, -11, -11, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10,
        -10, -10, -10, -10, -10, -10, -10, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9,
        -9, -9, -9, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -7, -7, -7, -7,
        -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6,
        -6, -6, -6, -6, -6, -6, -6, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5,
        -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -3, -3, -3, -3, -3, -3, -3,
        -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2,
        -2, -2, -2, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
        3, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5,
        5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
        7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 10, 10, 10, 10, 10, 10,
        10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,
        11, 11, 11, 11, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 13, 13, 13,
        13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
        14, 14, 14, 14, 14, 14, 14, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
        16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 17, 17, 17, 17, 17, 17, 17,
        17, 17, 17, 17, 17, 17, 17, 17, 17, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18,
        18, 18, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 20, 20, 20, 20, 20, 20,
        20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21,
        21, 21, 21, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 23, 23, 23, 23, 23,
        23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24,
        24, 24, 24, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 26, 26, 26, 26, 26,
        26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27,
        27, 27, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 29, 29, 29, 29, 29, 29,
        29, 29, 29, 29, 29, 29, 29, 29, 29, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
        31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 32, 32, 32, 32, 32, 32, 32, 32, 32,
        32, 32, 32, 32, 32, 32, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 34, 34, 34, 34,
        34, 34, 34, 34, 34, 34, 34, 34, 34, 34, 34, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35,
        35, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 37, 37, 37, 37, 37, 37, 37, 37, 37,
        37, 37, 37, 37, 37, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 39, 39, 39, 39, 39,
        39, 39, 39, 39, 39, 39, 39, 39, 39, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 41, 41,
        41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42,
        43, 43, 43, 43, 43, 43, 43, 43, 43, 43, 43, 43, 43, 44, 44, 44, 44, 44, 44, 44, 44, 44, 44, 44,
        44, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 46, 46, 46, 46, 46, 46, 46, 46, 46, 46,
        46, 46, 47, 47, 47, 47, 47, 47, 47, 47, 47, 47, 47, 47, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48,
        48, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50,
        51, 51, 51, 51, 51, 51, 51, 51, 51, 51, 51, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 53, 53,
        53, 53, 53, 53, 53, 53, 53, 53, 54, 54, 54, 54, 54, 54, 54, 54, 54, 54, 54, 55, 55, 55, 55, 55,
        55, 55, 55, 55, 55, 56, 56, 56, 56, 56, 56, 56, 56, 56, 57, 57, 57, 57, 57, 57, 57, 57, 57, 57,
        58, 58, 58, 58, 58, 58, 58, 58, 58, 59, 59, 59, 59, 59, 59, 59, 59, 59, 60, 60, 60, 60, 60, 60,
        60, 60, 60, 61, 61, 61, 61, 61, 61, 61, 61, 62, 62, 62, 62, 62, 62, 62, 62, 62, 63, 63, 63, 63,
        63, 63, 63, 64, 64, 64, 64, 64, 64, 64, 64, 65, 65, 65, 65, 65, 65, 65, 66, 66, 66, 66, 66, 66,
        66, 66, 67, 67, 67, 67, 67, 67, 68, 68, 68, 68, 68, 68, 68, 69, 69, 69, 69, 69, 69, 70, 70, 70,
        70, 70, 70, 71, 71, 71, 71, 71, 71, 72, 72, 72, 72, 72, 73, 73, 73, 73, 73, 74, 74, 74, 74, 74,
        75, 75, 75, 75, 75, 76, 76, 76, 76, 77, 77, 77, 77, 78, 78, 78, 79, 79, 79, 79, 80, 80, 80, 81,
        81, 81, 82, 82, 83, 83, 84, 84, 85, 86, 86, 87, 90,
    ];

    trig_arrays.arc_cosine_array_by_thousandths = [
        180, 177, 176, 176, 175, 174, 174, 173, 173, 172, 172, 171, 171, 171, 170, 170, 170, 169,
        169, 169, 169, 168, 168, 168, 167, 167, 167, 167, 166, 166, 166, 166, 165, 165, 165, 165,
        165, 164, 164, 164, 164, 164, 163, 163, 163, 163, 163, 162, 162, 162, 162, 162, 161, 161,
        161, 161, 161, 161, 160, 160, 160, 160, 160, 160, 159, 159, 159, 159, 159, 159, 158, 158,
        158, 158, 158, 158, 158, 157, 157, 157, 157, 157, 157, 156, 156, 156, 156, 156, 156, 156,
        156, 155, 155, 155, 155, 155, 155, 155, 154, 154, 154, 154, 154, 154, 154, 154, 153, 153,
        153, 153, 153, 153, 153, 152, 152, 152, 152, 152, 152, 152, 152, 152, 151, 151, 151, 151,
        151, 151, 151, 151, 150, 150, 150, 150, 150, 150, 150, 150, 150, 149, 149, 149, 149, 149,
        149, 149, 149, 149, 148, 148, 148, 148, 148, 148, 148, 148, 148, 147, 147, 147, 147, 147,
        147, 147, 147, 147, 147, 146, 146, 146, 146, 146, 146, 146, 146, 146, 145, 145, 145, 145,
        145, 145, 145, 145, 145, 145, 144, 144, 144, 144, 144, 144, 144, 144, 144, 144, 144, 143,
        143, 143, 143, 143, 143, 143, 143, 143, 143, 142, 142, 142, 142, 142, 142, 142, 142, 142,
        142, 142, 141, 141, 141, 141, 141, 141, 141, 141, 141, 141, 141, 140, 140, 140, 140, 140,
        140, 140, 140, 140, 140, 140, 139, 139, 139, 139, 139, 139, 139, 139, 139, 139, 139, 139,
        138, 138, 138, 138, 138, 138, 138, 138, 138, 138, 138, 137, 137, 137, 137, 137, 137, 137,
        137, 137, 137, 137, 137, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 135,
        135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 134, 134, 134, 134, 134, 134,
        134, 134, 134, 134, 134, 134, 133, 133, 133, 133, 133, 133, 133, 133, 133, 133, 133, 133,
        133, 132, 132, 132, 132, 132, 132, 132, 132, 132, 132, 132, 132, 132, 131, 131, 131, 131,
        131, 131, 131, 131, 131, 131, 131, 131, 131, 130, 130, 130, 130, 130, 130, 130, 130, 130,
        130, 130, 130, 130, 129, 129, 129, 129, 129, 129, 129, 129, 129, 129, 129, 129, 129, 129,
        128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 127, 127, 127, 127,
        127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 126, 126, 126, 126, 126, 126, 126, 126,
        126, 126, 126, 126, 126, 126, 125, 125, 125, 125, 125, 125, 125, 125, 125, 125, 125, 125,
        125, 125, 124, 124, 124, 124, 124, 124, 124, 124, 124, 124, 124, 124, 124, 124, 124, 123,
        123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 122, 122, 122, 122, 122,
        122, 122, 122, 122, 122, 122, 122, 122, 122, 122, 121, 121, 121, 121, 121, 121, 121, 121,
        121, 121, 121, 121, 121, 121, 121, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120,
        120, 120, 120, 120, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119,
        119, 118, 118, 118, 118, 118, 118, 118, 118, 118, 118, 118, 118, 118, 118, 118, 118, 117,
        117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 116, 116, 116, 116,
        116, 116, 116, 116, 116, 116, 116, 116, 116, 116, 116, 116, 115, 115, 115, 115, 115, 115,
        115, 115, 115, 115, 115, 115, 115, 115, 115, 115, 114, 114, 114, 114, 114, 114, 114, 114,
        114, 114, 114, 114, 114, 114, 114, 114, 113, 113, 113, 113, 113, 113, 113, 113, 113, 113,
        113, 113, 113, 113, 113, 113, 112, 112, 112, 112, 112, 112, 112, 112, 112, 112, 112, 112,
        112, 112, 112, 112, 111, 111, 111, 111, 111, 111, 111, 111, 111, 111, 111, 111, 111, 111,
        111, 111, 110, 110, 110, 110, 110, 110, 110, 110, 110, 110, 110, 110, 110, 110, 110, 110,
        110, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 108,
        108, 108, 108, 108, 108, 108, 108, 108, 108, 108, 108, 108, 108, 108, 108, 108, 107, 107,
        107, 107, 107, 107, 107, 107, 107, 107, 107, 107, 107, 107, 107, 107, 106, 106, 106, 106,
        106, 106, 106, 106, 106, 106, 106, 106, 106, 106, 106, 106, 106, 105, 105, 105, 105, 105,
        105, 105, 105, 105, 105, 105, 105, 105, 105, 105, 105, 105, 104, 104, 104, 104, 104, 104,
        104, 104, 104, 104, 104, 104, 104, 104, 104, 104, 104, 103, 103, 103, 103, 103, 103, 103,
        103, 103, 103, 103, 103, 103, 103, 103, 103, 103, 102, 102, 102, 102, 102, 102, 102, 102,
        102, 102, 102, 102, 102, 102, 102, 102, 102, 101, 101, 101, 101, 101, 101, 101, 101, 101,
        101, 101, 101, 101, 101, 101, 101, 101, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
        100, 100, 100, 100, 100, 100, 100, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99,
        99, 99, 99, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 97, 97, 97, 97,
        97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 96, 96, 96, 96, 96, 96, 96, 96, 96, 96, 96,
        96, 96, 96, 96, 96, 96, 96, 95, 95, 95, 95, 95, 95, 95, 95, 95, 95, 95, 95, 95, 95, 95, 95, 95,
        94, 94, 94, 94, 94, 94, 94, 94, 94, 94, 94, 94, 94, 94, 94, 94, 94, 93, 93, 93, 93, 93, 93, 93,
        93, 93, 93, 93, 93, 93, 93, 93, 93, 93, 93, 92, 92, 92, 92, 92, 92, 92, 92, 92, 92, 92, 92, 92,
        92, 92, 92, 92, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 90, 90,
        90, 90, 90, 90, 90, 90, 90, 90, 90, 90, 90, 90, 90, 90, 90, 89, 89, 89, 89, 89, 89, 89, 89, 89,
        89, 89, 89, 89, 89, 89, 89, 89, 89, 88, 88, 88, 88, 88, 88, 88, 88, 88, 88, 88, 88, 88, 88, 88,
        88, 88, 87, 87, 87, 87, 87, 87, 87, 87, 87, 87, 87, 87, 87, 87, 87, 87, 87, 87, 86, 86, 86, 86,
        86, 86, 86, 86, 86, 86, 86, 86, 86, 86, 86, 86, 86, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85,
        85, 85, 85, 85, 85, 85, 84, 84, 84, 84, 84, 84, 84, 84, 84, 84, 84, 84, 84, 84, 84, 84, 84, 84,
        83, 83, 83, 83, 83, 83, 83, 83, 83, 83, 83, 83, 83, 83, 83, 83, 83, 82, 82, 82, 82, 82, 82, 82,
        82, 82, 82, 82, 82, 82, 82, 82, 82, 82, 81, 81, 81, 81, 81, 81, 81, 81, 81, 81, 81, 81, 81, 81,
        81, 81, 81, 81, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 79, 79, 79,
        79, 79, 79, 79, 79, 79, 79, 79, 79, 79, 79, 79, 79, 79, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78,
        78, 78, 78, 78, 78, 78, 78, 77, 77, 77, 77, 77, 77, 77, 77, 77, 77, 77, 77, 77, 77, 77, 77, 77,
        76, 76, 76, 76, 76, 76, 76, 76, 76, 76, 76, 76, 76, 76, 76, 76, 76, 75, 75, 75, 75, 75, 75, 75,
        75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 74, 74, 74, 74, 74, 74, 74, 74, 74, 74, 74, 74, 74, 74,
        74, 74, 74, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 72, 72, 72, 72, 72,
        72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 71, 71, 71, 71, 71, 71, 71, 71, 71, 71, 71, 71,
        71, 71, 71, 71, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 69, 69, 69,
        69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 68, 68, 68, 68, 68, 68, 68, 68, 68, 68, 68,
        68, 68, 68, 68, 68, 67, 67, 67, 67, 67, 67, 67, 67, 67, 67, 67, 67, 67, 67, 67, 67, 66, 66, 66,
        66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65,
        65, 65, 65, 65, 65, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 63, 63, 63,
        63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 62, 62, 62, 62, 62, 62, 62, 62, 62, 62, 62, 62,
        62, 62, 62, 62, 61, 61, 61, 61, 61, 61, 61, 61, 61, 61, 61, 61, 61, 61, 61, 60, 60, 60, 60, 60,
        60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 59, 59, 59, 59, 59, 59, 59, 59, 59, 59, 59, 59, 59, 59,
        59, 58, 58, 58, 58, 58, 58, 58, 58, 58, 58, 58, 58, 58, 58, 58, 57, 57, 57, 57, 57, 57, 57, 57,
        57, 57, 57, 57, 57, 57, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 55, 55, 55,
        55, 55, 55, 55, 55, 55, 55, 55, 55, 55, 55, 54, 54, 54, 54, 54, 54, 54, 54, 54, 54, 54, 54, 54,
        54, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 52, 52, 52, 52, 52, 52, 52, 52, 52,
        52, 52, 52, 52, 52, 51, 51, 51, 51, 51, 51, 51, 51, 51, 51, 51, 51, 51, 51, 50, 50, 50, 50, 50,
        50, 50, 50, 50, 50, 50, 50, 50, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 48, 48, 48,
        48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 47, 47, 47, 47, 47, 47, 47, 47, 47, 47, 47, 47, 47, 46,
        46, 46, 46, 46, 46, 46, 46, 46, 46, 46, 46, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45,
        44, 44, 44, 44, 44, 44, 44, 44, 44, 44, 44, 44, 43, 43, 43, 43, 43, 43, 43, 43, 43, 43, 43, 43,
        42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 40,
        40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 38, 38, 38,
        38, 38, 38, 38, 38, 38, 38, 38, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 36, 36, 36, 36, 36, 36,
        36, 36, 36, 36, 36, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 34, 34, 34, 34, 34, 34, 34, 34, 34,
        33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 32, 32, 32, 32, 32, 32, 32, 32, 32, 31, 31, 31, 31, 31,
        31, 31, 31, 31, 30, 30, 30, 30, 30, 30, 30, 30, 30, 29, 29, 29, 29, 29, 29, 29, 29, 28, 28, 28,
        28, 28, 28, 28, 28, 28, 27, 27, 27, 27, 27, 27, 27, 26, 26, 26, 26, 26, 26, 26, 26, 25, 25, 25,
        25, 25, 25, 25, 24, 24, 24, 24, 24, 24, 24, 24, 23, 23, 23, 23, 23, 23, 22, 22, 22, 22, 22, 22,
        22, 21, 21, 21, 21, 21, 21, 20, 20, 20, 20, 20, 20, 19, 19, 19, 19, 19, 19, 18, 18, 18, 18, 18,
        17, 17, 17, 17, 17, 16, 16, 16, 16, 16, 15, 15, 15, 15, 15, 14, 14, 14, 14, 13, 13, 13, 13, 12,
        12, 12, 11, 11, 11, 11, 10, 10, 10, 9, 9, 9, 8, 8, 7, 7, 6, 6, 5, 4, 4, 3, 0,
    ];

    trig_arrays.arc_tan_by_ones = [
        -89, -89, -89, -89, -89, -89, -89, -89, -89, -89, -89, -89, -89, -89, -89, -89, -89, -89,
        -89, -89, -89, -89, -89, -89, -89, -89, -89, -89, -89, -89, -89, -89, -89, -89, -89, -89,
        -89, -89, -89, -89, -89, -89, -89, -89, -89, -89, -89, -89, -89, -89, -89, -89, -89, -89,
        -89, -89, -89, -89, -89, -89, -89, -89, -88, -88, -88, -88, -88, -88, -88, -88, -88, -88,
        -88, -88, -88, -88, -88, -88, -87, -87, -87, -87, -87, -87, -86, -86, -86, -86, -85, -85,
        -84, -84, -83, -82, -81, -79, -76, -72, -63, -45, 0, 45, 63, 72, 76, 79, 81, 82, 83, 84,
        84, 85, 85, 86, 86, 86, 86, 87, 87, 87, 87, 87, 87, 88, 88, 88, 88, 88, 88, 88, 88, 88, 88, 88,
        88, 88, 88, 88, 88, 89, 89, 89, 89, 89, 89, 89, 89, 89, 89, 89, 89, 89, 89, 89, 89, 89, 89, 89,
        89, 89, 89, 89, 89, 89, 89, 89, 89, 89, 89, 89, 89, 89, 89, 89, 89, 89, 89, 89, 89, 89, 89, 89,
        89, 89, 89, 89, 89, 89, 89, 89, 89, 89, 89, 89, 89, 89, 89, 89, 89, 89, 89,
    ];

    trig_arrays.arc_tan_by_tenths = [
        -84, -84, -84, -84, -84, -84, -84, -84, -84, -84, -84, -84, -84, -83, -83, -83, -83, -83,
        -83, -83, -83, -83, -83, -83, -83, -82, -82, -82, -82, -82, -82, -82, -82, -82, -81, -81,
        -81, -81, -81, -81, -81, -80, -80, -80, -80, -80, -80, -79, -79, -79, -79, -78, -78, -78,
        -78, -77, -77, -77, -77, -76, -76, -76, -75, -75, -74, -74, -74, -73, -73, -72, -72, -71,
        -70, -70, -69, -68, -67, -67, -66, -65, -63, -62, -61, -60, -58, -56, -54, -52, -50, -48,
        -45, -42, -39, -35, -31, -27, -22, -17, -11, -6, 0, 6, 11, 17, 22, 27, 31, 35, 39, 42, 45, 48,
        50, 52, 54, 56, 58, 60, 61, 62, 63, 65, 66, 67, 67, 68, 69, 70, 70, 71, 72, 72, 73, 73, 74, 74,
        74, 75, 75, 76, 76, 76, 77, 77, 77, 77, 78, 78, 78, 78, 79, 79, 79, 79, 80, 80, 80, 80, 80, 80,
        81, 81, 81, 81, 81, 81, 81, 82, 82, 82, 82, 82, 82, 82, 82, 82, 83, 83, 83, 83, 83, 83, 83, 83,
        83, 83, 83, 83, 84, 84, 84, 84, 84, 84, 84, 84, 84, 84, 84, 84, 84,
    ];

    trig_arrays.arc_tan_by_hundreths = [
        -45, -45, -44, -44, -44, -44, -43, -43, -43, -42, -42, -42, -41, -41, -41, -40, -40, -40,
        -39, -39, -39, -38, -38, -38, -37, -37, -37, -36, -36, -35, -35, -35, -34, -34, -33, -33,
        -33, -32, -32, -31, -31, -31, -30, -30, -29, -29, -28, -28, -27, -27, -27, -26, -26, -25,
        -25, -24, -24, -23, -23, -22, -22, -21, -21, -20, -20, -19, -19, -18, -18, -17, -17, -16,
        -16, -15, -15, -14, -13, -13, -12, -12, -11, -11, -10, -10, -9, -9, -8, -7, -7, -6, -6, -5,
        -5, -4, -3, -3, -2, -2, -1, -1, 0, 1, 1, 2, 2, 3, 3, 4, 5, 5, 6, 6, 7, 7, 8, 9, 9, 10, 10, 11,
        11, 12, 12, 13, 13, 14, 15, 15, 16, 16, 17, 17, 18, 18, 19, 19, 20, 20, 21, 21, 22, 22, 23, 23,
        24, 24, 25, 25, 26, 26, 27, 27, 27, 28, 28, 29, 29, 30, 30, 31, 31, 31, 32, 32, 33, 33, 33, 34,
        34, 35, 35, 35, 36, 36, 37, 37, 37, 38, 38, 38, 39, 39, 39, 40, 40, 40, 41, 41, 41, 42, 42, 42,
        43, 43, 43, 44, 44, 44, 44, 45, 45,
    ];
}
