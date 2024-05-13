use bevy::prelude::*;

pub fn test_trig_arrays(
    trig_arrays: Res<crate::utility_functions::deterministic_trig::TrigArrays>
) {

    warn!("{}", crate::utility_functions::deterministic_trig::sine_times_1000(1082, trig_arrays.sine_array));
    warn!("{}", crate::utility_functions::deterministic_trig::sine_times_1000(361, trig_arrays.sine_array));
    warn!("{}", crate::utility_functions::deterministic_trig::sine_times_1000(130, trig_arrays.sine_array));
    warn!("{}", crate::utility_functions::deterministic_trig::sine_times_1000(81, trig_arrays.sine_array));
    warn!("{}", crate::utility_functions::deterministic_trig::sine_times_1000(0, trig_arrays.sine_array));
    warn!("{}", crate::utility_functions::deterministic_trig::sine_times_1000(-31, trig_arrays.sine_array));
    warn!("{}", crate::utility_functions::deterministic_trig::sine_times_1000(-18786, trig_arrays.sine_array));

    warn!("{}", crate::utility_functions::deterministic_trig::cosine_times_1000(13746, trig_arrays.cosine_array));
    warn!("{}", crate::utility_functions::deterministic_trig::cosine_times_1000(446, trig_arrays.cosine_array));
    warn!("{}", crate::utility_functions::deterministic_trig::cosine_times_1000(76, trig_arrays.cosine_array));
    warn!("{}", crate::utility_functions::deterministic_trig::cosine_times_1000(0, trig_arrays.cosine_array));
    warn!("{}", crate::utility_functions::deterministic_trig::cosine_times_1000(-52, trig_arrays.cosine_array));
    warn!("{}", crate::utility_functions::deterministic_trig::cosine_times_1000(-820, trig_arrays.cosine_array));
    warn!("{}", crate::utility_functions::deterministic_trig::cosine_times_1000(-14526, trig_arrays.cosine_array));

    warn!("{}", crate::utility_functions::deterministic_trig::tangent_times_1000(16788, trig_arrays.tangent_array));
    warn!("{}", crate::utility_functions::deterministic_trig::tangent_times_1000(198, trig_arrays.tangent_array));
    warn!("{}", crate::utility_functions::deterministic_trig::tangent_times_1000(17, trig_arrays.tangent_array));
    warn!("{}", crate::utility_functions::deterministic_trig::tangent_times_1000(0, trig_arrays.tangent_array));
    warn!("{}", crate::utility_functions::deterministic_trig::tangent_times_1000(-61, trig_arrays.tangent_array));
    warn!("{}", crate::utility_functions::deterministic_trig::tangent_times_1000(-148, trig_arrays.tangent_array));
    warn!("{}", crate::utility_functions::deterministic_trig::tangent_times_1000(-19877, trig_arrays.tangent_array));
    warn!("{}", crate::utility_functions::deterministic_trig::tangent_times_1000(90, trig_arrays.tangent_array));
    warn!("{}", crate::utility_functions::deterministic_trig::tangent_times_1000(270, trig_arrays.tangent_array));

    warn!("{}", crate::utility_functions::deterministic_trig::arc_sine_of_thousandths(10300, trig_arrays.arc_sine_array_by_thousandths));
    warn!("{}", crate::utility_functions::deterministic_trig::arc_sine_of_thousandths(1000, trig_arrays.arc_sine_array_by_thousandths));
    warn!("{}", crate::utility_functions::deterministic_trig::arc_sine_of_thousandths(770, trig_arrays.arc_sine_array_by_thousandths));
    warn!("{}", crate::utility_functions::deterministic_trig::arc_sine_of_thousandths(0, trig_arrays.arc_sine_array_by_thousandths));
    warn!("{}", crate::utility_functions::deterministic_trig::arc_sine_of_thousandths(-613, trig_arrays.arc_sine_array_by_thousandths));
    warn!("{}", crate::utility_functions::deterministic_trig::arc_sine_of_thousandths(-1000, trig_arrays.arc_sine_array_by_thousandths));
    warn!("{}", crate::utility_functions::deterministic_trig::arc_sine_of_thousandths(-9827, trig_arrays.arc_sine_array_by_thousandths));

    warn!("{}", crate::utility_functions::deterministic_trig::arc_cosine_of_thousandths(10300, trig_arrays.arc_cosine_array_by_thousandths));
    warn!("{}", crate::utility_functions::deterministic_trig::arc_cosine_of_thousandths(1000, trig_arrays.arc_cosine_array_by_thousandths));
    warn!("{}", crate::utility_functions::deterministic_trig::arc_cosine_of_thousandths(441, trig_arrays.arc_cosine_array_by_thousandths));
    warn!("{}", crate::utility_functions::deterministic_trig::arc_cosine_of_thousandths(0, trig_arrays.arc_cosine_array_by_thousandths));
    warn!("{}", crate::utility_functions::deterministic_trig::arc_cosine_of_thousandths(-800, trig_arrays.arc_cosine_array_by_thousandths));
    warn!("{}", crate::utility_functions::deterministic_trig::arc_cosine_of_thousandths(-1200, trig_arrays.arc_cosine_array_by_thousandths));
    
}