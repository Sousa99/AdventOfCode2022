use std::collections::HashMap;

// =========================================== TYPE AND STRUCT DEFINITIONS ===========================================

trait Command {

    fn is_completed(&self) -> bool;
    fn make_iteration(&mut self, registers: &mut HashMap<DeviceRegisterKey, DeviceRegisterValue>);
    fn reset_command(&mut self);
}

struct CommandAddX {

    _name: String,
    steps_taken: DeviceTime,
    current_steps: DeviceTime,

    register: DeviceRegisterKey,
    register_change: DeviceRegisterValue,
}

struct CommandNoOp {
    
    _name: String,
    steps_taken: DeviceTime,
    current_steps: DeviceTime,
}

type DeviceTime = u32;
type DeviceRegisterKey = char;
type DeviceRegisterValue = i32;

pub struct HandheldDeviceSetup {

    time: DeviceTime,
    commands: Vec<Box<dyn Command>>,
    registers: HashMap<DeviceRegisterKey, DeviceRegisterValue>,
    current_registers: HashMap<DeviceRegisterKey, DeviceRegisterValue>
}

// =============================================== AUXILIARY FUNCTIONS ===============================================

fn develop_command(command_line: String) -> Box<dyn Command> {

    let mut arguments: Vec<String> = command_line.split_whitespace()
        .map(|split| split.to_owned())
        .collect();

    let first_argument: String = arguments.remove(0);

    match first_argument.as_str() {
        "addx" => Box::new(CommandAddX::new(arguments)),
        "noop" => Box::new(CommandNoOp::new(arguments)),
        invalid_command => panic!("🚨 Command '{}' not recognized!", invalid_command)
    }

}

// ================================================= IMPLEMENTATIONS =================================================

impl CommandAddX {

    fn new(mut arguments: Vec<String>) -> CommandAddX {

        let register_change: DeviceRegisterValue = arguments.remove(0)
            .parse().unwrap();

        CommandAddX {
            _name: "command_addx".to_owned(),
            steps_taken: 2,
            current_steps: 0,

            register: 'X',
            register_change,
        }
    }
}

impl Command for CommandAddX {

    fn is_completed(&self) -> bool { self.current_steps == self.steps_taken }

    fn make_iteration(&mut self, registers: &mut HashMap<DeviceRegisterKey, DeviceRegisterValue>) {
        
        self.current_steps = self.current_steps + 1;
        if self.current_steps == self.steps_taken {

            let register_x: &mut DeviceRegisterValue = registers.get_mut(&self.register).unwrap();
            *register_x = *register_x + self.register_change;
        }
    }

    fn reset_command(&mut self) { self.current_steps = 0 }
}

impl CommandNoOp {

    fn new(_: Vec<String>) -> CommandNoOp {

        CommandNoOp {
            _name: "command_noop".to_owned(),
            steps_taken: 1,
            current_steps: 0,
        }
    }
}

impl Command for CommandNoOp {

    fn is_completed(&self) -> bool { self.current_steps == self.steps_taken }

    fn make_iteration(&mut self, _: &mut HashMap<DeviceRegisterKey, DeviceRegisterValue>) {

        self.current_steps = self.current_steps + 1;
    }

    fn reset_command(&mut self) { self.current_steps = 0 }
}

impl HandheldDeviceSetup {

    pub fn new(command_lines: Vec<String>, registers: Vec<(DeviceRegisterKey, DeviceRegisterValue)>) -> HandheldDeviceSetup {

        HandheldDeviceSetup {
            time: 1,
            commands: command_lines.into_iter()
                .map(|line| develop_command(line))
                .collect(),
            registers: registers.clone().into_iter().collect(),
            current_registers: registers.into_iter().collect(),
        }
    }

    fn reset_machine(&mut self) {

        self.current_registers = self.registers.clone();
        self.commands.iter_mut().for_each(|command| command.reset_command());
        self.time = 1;
    }

    pub fn get_signal_strength(&mut self, initial: DeviceTime, step: DeviceTime, register: char) -> DeviceRegisterValue {

        self.reset_machine();

        let mut signal_strength: DeviceRegisterValue = 0;

        for command in self.commands.iter_mut() {
            while ! command.is_completed() {

                self.time = self.time + 1;
                command.make_iteration(&mut self.current_registers);

                if self.time >= initial && (self.time - initial) % step == 0 {
                    let register_value: DeviceRegisterValue = *self.current_registers.get(&register).unwrap();
                    signal_strength += self.time as DeviceRegisterValue * register_value;
                }

            }
        }

        return signal_strength;
    }

    pub fn display_screen(&mut self, line_size: usize, sprite_radius: usize, register: char) -> Vec<Vec<char>> {

        self.reset_machine();

        let mut lines: Vec<Vec<char>> = Vec::new();
        let mut current_line: Vec<char> = Vec::new();

        current_line.push('#');

        for command in self.commands.iter_mut() {
            while ! command.is_completed() {

                self.time = self.time + 1;
                command.make_iteration(&mut self.current_registers);

                let register_value: DeviceRegisterValue = *self.current_registers.get(&register).unwrap();

                let pixel_normalized: DeviceRegisterValue = (self.time as DeviceRegisterValue - 1) % line_size as DeviceRegisterValue;
                let sprite_min: DeviceRegisterValue = register_value - sprite_radius as DeviceRegisterValue;
                let sprite_max: DeviceRegisterValue = register_value + sprite_radius as DeviceRegisterValue;

                let in_sprite: bool = (pixel_normalized >= sprite_min) && (pixel_normalized <= sprite_max);

                if in_sprite { current_line.push('#') }
                else { current_line.push('.') }

                if (pixel_normalized + 1) % line_size as DeviceRegisterValue == 0 {
                    lines.push(current_line);
                    current_line = Vec::new();
                }

            }
        }

        return lines;
    }
}