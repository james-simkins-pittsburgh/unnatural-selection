use crate::settings::GameSettings;
use crate::simulation::AllCurrentInformation;
use crate::simulation::CheapRandomGameworld;
use crate::simulation::AdministrativeInformation;

pub fn simulate_currents(
    all_current_info: &mut AllCurrentInformation,
    admin_info: &AdministrativeInformation,
    cheap_random: &mut CheapRandomGameworld,
    game_settings: &GameSettings
) {
    // Removes all expired currents.
    // TO DO: Update with more efficient code using retain() method.
    for index in 0..all_current_info.current_information_vec.len() {
        if
            all_current_info.current_information_vec.len() > 0 &&
            index < all_current_info.current_information_vec.len()
        {
            if
                all_current_info.current_information_vec[index].expiration_time <=
                    admin_info.tick_counter &&
                index < all_current_info.current_information_vec.len()
            {
                all_current_info.current_information_vec.swap_remove(index);
                if
                    all_current_info.current_information_vec.len() == 0 ||
                    index >= all_current_info.current_information_vec.len()
                {
                    break;
                }
            }
        }
    }

    // Makes new currents.
    if all_current_info.current_information_vec.len() < (game_settings.number_of_currents as usize) {
        if
            (game_settings.number_of_currents as usize) -
                all_current_info.current_information_vec.len() == 1
        {
            if cheap_random.random_0_to_359.next_random() < 36 {
                all_current_info.current_information_vec.push(
                    crate::simulation::CurrentInformation {
                        center_x: (180 - cheap_random.random_0_to_359.next_random()) *
                        (game_settings.map_length / 360),
                        center_y: (180 - cheap_random.random_0_to_359.next_random()) *
                        (game_settings.map_length / 360),
                        angle_in_radians_times_1000: cheap_random.random_0_to_359.next_random() *
                        17,
                        intensity: game_settings.current_intensity *
                        (cheap_random.random_0_to_359.next_random() / 36 + 1),
                        radius: 25000,
                        expiration_time: admin_info.tick_counter + 30,
                        background: false,
                    }
                )
            }
        } else {
            all_current_info.current_information_vec.push(crate::simulation::CurrentInformation {
                center_x: (180 - cheap_random.random_0_to_359.next_random()) *
                (game_settings.map_length / 360),
                center_y: (180 - cheap_random.random_0_to_359.next_random()) *
                (game_settings.map_length / 360),
                angle_in_radians_times_1000: cheap_random.random_0_to_359.next_random() * 17,
                intensity: game_settings.current_intensity *
                (cheap_random.random_0_to_359.next_random() / 180 + 1),
                radius: 250000,
                expiration_time: admin_info.tick_counter + 300,
                background: false,
            })
        }
    }
}
