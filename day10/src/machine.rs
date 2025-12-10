#[derive(Debug, PartialEq, Clone, Eq, Hash, PartialOrd, Ord)]
pub struct Machine {
    light_diagram: Vec<bool>,
    wiring_schematics: Vec<Vec<i64>>,
    joltage_requirements: Vec<i64>,
    lights: Vec<bool>,
}

impl Machine {
    pub fn wiring_schematics(&self) -> &[Vec<i64>] {
        &self.wiring_schematics
    }

    pub fn joltage_requirements(&self) -> &[i64] {
        &self.joltage_requirements
    }

    pub fn new(
        light_diagram: &Vec<bool>,
        wiring_schematics: &Vec<Vec<i64>>,
        joltage_requirements: &Vec<i64>,
    ) -> Machine {
        Machine {
            light_diagram: light_diagram.clone(),
            wiring_schematics: wiring_schematics.clone(),
            joltage_requirements: joltage_requirements.clone(),
            lights: vec![false; light_diagram.len()],
        }
    }

    pub fn push_button(&mut self, button_index: i64) -> bool {
        for light_index in &self.wiring_schematics[button_index as usize] {
            self.lights[*light_index as usize] = !self.lights[*light_index as usize];
        }
        self.lights == self.light_diagram
    }

    pub fn reset_lights(&mut self) {
        self.lights.fill(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_button() {
        // Arrange
        let mut machine = Machine::new(&vec![false, true], &vec![vec![1]], &vec![]);

        // Act
        let actual: bool = machine.push_button(0);

        // Assert
        let expected = true;
        assert_eq!(actual, expected);
    }
}
