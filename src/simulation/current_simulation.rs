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
    all_current_info.current_information_vec.retain(
        |&current| current.expiration_time > admin_info.tick_counter
    );

    // Makes new currents if there are less currents that the game setting.
    if all_current_info.current_information_vec.len() < (game_settings.number_of_currents as usize) {
        // If only one current short, 10% chance of adding new current.
        if
            (game_settings.number_of_currents as usize) -
                all_current_info.current_information_vec.len() == 1
        {
            if cheap_random.random_0_to_359.next_random() < 36 {
                // Makes a current with random direction and intensity 1 to 10.
                all_current_info.current_information_vec.push(
                    crate::simulation::CurrentInformation {
                        center_x: (180 - cheap_random.random_0_to_359.next_random()) *
                        (game_settings.map_width / 360),
                        center_y: (180 - cheap_random.random_0_to_359.next_random()) *
                        (game_settings.map_width / 360),
                        angle_in_radians_times_1000: cheap_random.random_0_to_359.next_random() *
                        17,
                        intensity: game_settings.current_intensity *
                        (cheap_random.random_0_to_359.next_random() / 36 + 1),
                        radius: 25000,
                        expiration_time: admin_info.tick_counter + 300,
                        background: false,
                    }
                )
            }

            // If more than one currents short, 100% chance of adding new current.
        } else {
            // Makes a current with random direction and intensity 1 to 10.
            all_current_info.current_information_vec.push(crate::simulation::CurrentInformation {
                center_x: (180 - cheap_random.random_0_to_359.next_random()) *
                (game_settings.map_width / 360),
                center_y: (180 - cheap_random.random_0_to_359.next_random()) *
                (game_settings.map_width / 360),
                angle_in_radians_times_1000: cheap_random.random_0_to_359.next_random() * 17,
                intensity: game_settings.current_intensity *
                (cheap_random.random_0_to_359.next_random() / 36 + 1),
                radius: 25000,
                expiration_time: admin_info.tick_counter + 300,
                background: false,
            })
        }
    }
}
