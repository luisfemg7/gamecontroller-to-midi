extern crate sdl2;

use sdl2::controller::Axis;
use sdl2::controller::Button;
use sdl2::event::Event;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let mut event_pump = sdl_context.event_pump()?;

    let controller_subsystem = sdl_context.game_controller()?;

    let _game_controller = controller_subsystem.open(0).map_err(|e| format!("Failed to open controller: {}", e))?;
    
    /*
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::ControllerButtonDown {which: _, timestamp: _, button: Button::A} => println!("A was pressed down"),
                Event::ControllerButtonUp {which: _, timestamp: _, button: Button::A} => println!("A was pressed up"),
                _ => {},
            }
        }
    }
    */

    'running: loop {
        for event in event_pump.poll_iter()
        {
            match event {
                Event::Quit { .. } => break 'running,
                Event::ControllerButtonDown { button, .. } => {
                    match button {
                        Button::A => println!("A button pressed down!"),
                        Button::B => println!("B button pressed down!"),
                        Button::X => println!("X button pressed down!"),
                        Button::Y => println!("Y button pressed down!"),
                        Button::Back => println!("Back button pressed down!"),
                        Button::Guide => println!("Guide button pressed down!"),
                        Button::Start => println!("Start button pressed down!"),
                        Button::LeftStick => println!("Left stick button pressed down!"),
                        Button::RightStick => println!("Right stick button pressed down!"),
                        Button::LeftShoulder => println!("Left shoulder button pressed down!"),
                        Button::RightShoulder => println!("Right shoulder button pressed down!"),
                        Button::DPadUp => println!("DPadUp button pressed down!"),
                        Button::DPadDown => println!("DPadDown button pressed down!"),
                        Button::DPadLeft => println!("DPadLeft button pressed down!"),
                        Button::DPadRight => println!("DPadRight button pressed down!"),
                        Button::Misc1 => println!("Misc1 button pressed down!"),
                        Button::Paddle1 => println!("Paddle1 button pressed down!"),
                        Button::Paddle2 => println!("Paddle2 button pressed down!"),
                        Button::Paddle3 => println!("Paddle3 button pressed down!"),
                        Button::Paddle4 => println!("Paddle4 button pressed down!"),
                        Button::Touchpad => println!("Touchpad button pressed down!"),
                    }
                },
                Event::ControllerButtonUp { button, .. } => {
                    match button {
                        Button::A => println!("A button pressed up!"),
                        Button::B => println!("B button pressed up!"),
                        Button::X => println!("X button pressed up!"),
                        Button::Y => println!("Y button pressed up!"),
                        Button::Back => println!("Back button pressed up!"),
                        Button::Guide => println!("Guide button pressed up!"),
                        Button::Start => println!("Start button pressed up!"),
                        Button::LeftStick => println!("Left stick button pressed up!"),
                        Button::RightStick => println!("Right stick button pressed up!"),
                        Button::LeftShoulder => println!("Left shoulder button pressed up!"),
                        Button::RightShoulder => println!("Right shoulder button pressed up!"),
                        Button::DPadUp => println!("DPadUp button pressed up!"),
                        Button::DPadDown => println!("DPadDown button pressed up!"),
                        Button::DPadLeft => println!("DPadLeft button pressed up!"),
                        Button::DPadRight => println!("DPadRight button pressed up!"),
                        Button::Misc1 => println!("Misc1 button pressed up!"),
                        Button::Paddle1 => println!("Paddle1 button pressed up!"),
                        Button::Paddle2 => println!("Paddle2 button pressed up!"),
                        Button::Paddle3 => println!("Paddle3 button pressed up!"),
                        Button::Paddle4 => println!("Paddle4 button pressed up!"),
                        Button::Touchpad => println!("Touchpad button pressed up!"),
                    }
                },
                Event::ControllerAxisMotion { axis, value, .. } => {
                    match axis {
                        Axis::RightX => {
                            if value > 16384 {
                                println!("Right joystick moved right");
                            } else if value < -16384 {
                                println!("Right joystick moved left");
                            }
                        },
                        Axis::RightY => {
                            if value > 16384 {
                                println!("Right joystick moved down");
                            } else if value < -16384 {
                                println!("Right joystick moved up");
                            }
                        },
                        Axis::LeftX => {
                            if value > 16384 {
                                println!("Left joystick moved right");
                            } else if value < -16384 {
                                println!("Left joystick moved left");
                            }
                        },
                        Axis::LeftY => {
                            if value > 16384 {
                                println!("Left joystick moved down");
                            } else if value < -16384 {
                                println!("Left joystick moved up");
                            }
                        },
                        Axis::TriggerRight => {
                            println!("Right trigger value: {}", value);
                        },
                        Axis::TriggerLeft => {
                            println!("Left trigger value: {}", value);
                        },
                    }
                },
                _ => {}
            }
        }
    }

    Ok(())
}
