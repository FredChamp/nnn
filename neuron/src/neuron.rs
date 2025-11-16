//! Neuron implementation with realistic biological properties

use std::collections::VecDeque;

use crate::constants::{
    ACTION_POTENTIAL_PEAK, MAX_SPIKE_HISTORY, REFRACTORY_PERIOD_MS, RESTING_POTENTIAL, THRESHOLD,
};
use crate::neurotransmitter::Neurotransmitter;
use crate::synapse::Synapse;

/// Represents a single neuron with anatomical and physiological properties
#[derive(Debug)]
pub struct Neuron {
    id: usize,
    
    // Anatomical components
    dendrites: Vec<f32>,
    soma_potential: f32,
    axon_signal: Option<f32>,
    synapses: Vec<Synapse>,
    
    // Physiological state
    is_refractory: bool,
    refractory_timer: u32,
    spike_history: VecDeque<u32>,
}

impl Neuron {
    /// Creates a new neuron with the given ID
    pub fn new(id: usize) -> Self {
        Self {
            id,
            dendrites: Vec::new(),
            soma_potential: RESTING_POTENTIAL,
            axon_signal: None,
            synapses: Vec::new(),
            is_refractory: false,
            refractory_timer: 0,
            spike_history: VecDeque::with_capacity(MAX_SPIKE_HISTORY),
        }
    }

    /// Returns the neuron's ID
    pub fn id(&self) -> usize {
        self.id
    }

    /// Returns the current membrane potential
    pub fn membrane_potential(&self) -> f32 {
        self.soma_potential
    }

    /// Returns whether the neuron is in refractory period
    pub fn is_refractory(&self) -> bool {
        self.is_refractory
    }

    /// Connects this neuron to another via a synapse
    ///
    /// # Arguments
    /// * `target_id` - ID of the target neuron
    /// * `weight` - Synaptic weight (connection strength)
    /// * `neurotransmitter` - Type of neurotransmitter
    pub fn connect_to(&mut self, target_id: usize, weight: f32, neurotransmitter: Neurotransmitter) {
        self.synapses.push(Synapse::new(target_id, weight, neurotransmitter));
    }

    /// Receives an input signal on the dendrites
    pub fn receive_input(&mut self, signal: f32) {
        self.dendrites.push(signal);
    }

    /// Integrates all dendritic inputs into the soma (spatial summation)
    pub fn integrate_inputs(&mut self) {
        if !self.dendrites.is_empty() {
            let sum: f32 = self.dendrites.iter().sum();
            let average = sum / self.dendrites.len() as f32;
            self.soma_potential += average;
            self.dendrites.clear();
        }
    }

    /// Attempts to generate an action potential (all-or-none law)
    ///
    /// # Arguments
    /// * `time_ms` - Current simulation time in milliseconds
    ///
    /// # Returns
    /// `true` if an action potential was generated, `false` otherwise
    pub fn generate_action_potential(&mut self, time_ms: u32) -> bool {
        // Handle refractory period
        if self.is_refractory {
            self.refractory_timer = self.refractory_timer.saturating_sub(1);
            if self.refractory_timer == 0 {
                self.is_refractory = false;
                self.soma_potential = RESTING_POTENTIAL;
            }
            return false;
        }

        // All-or-none law: fire if threshold is reached
        if self.soma_potential >= THRESHOLD {
            // Depolarization
            self.soma_potential = ACTION_POTENTIAL_PEAK;
            self.axon_signal = Some(ACTION_POTENTIAL_PEAK);
            
            // Enter refractory period
            self.is_refractory = true;
            self.refractory_timer = REFRACTORY_PERIOD_MS;
            
            // Record spike
            self.spike_history.push_back(time_ms);
            if self.spike_history.len() > MAX_SPIKE_HISTORY {
                self.spike_history.pop_front();
            }
            
            true
        } else {
            // Passive decay towards resting potential
            let decay_rate = 0.1;
            self.soma_potential += (RESTING_POTENTIAL - self.soma_potential) * decay_rate;
            self.axon_signal = None;
            false
        }
    }

    /// Transmits the axon signal through all synapses
    ///
    /// # Returns
    /// A vector of (target_id, signal, neurotransmitter) tuples
    pub fn transmit(&self) -> Vec<(usize, f32, Neurotransmitter)> {
        if let Some(signal) = self.axon_signal {
            self.synapses
                .iter()
                .map(|synapse| {
                    let modulated_signal = synapse.modulate_signal(signal);
                    (synapse.target_id(), modulated_signal, synapse.neurotransmitter())
                })
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Calculates the firing rate over a time window
    ///
    /// # Arguments
    /// * `window_ms` - Time window in milliseconds
    ///
    /// # Returns
    /// Firing rate in Hz (spikes per second)
    pub fn firing_rate(&self, window_ms: u32) -> f32 {
        if self.spike_history.is_empty() {
            return 0.0;
        }
        
        let last_time = *self.spike_history.back().unwrap();
        let threshold_time = last_time.saturating_sub(window_ms);
        
        let recent_spikes = self.spike_history
            .iter()
            .filter(|&&t| t >= threshold_time)
            .count();
        
        (recent_spikes as f32 / window_ms as f32) * 1000.0
    }

    /// Returns the number of output synapses
    pub fn synapse_count(&self) -> usize {
        self.synapses.len()
    }

    /// Returns the spike history
    pub fn spike_history(&self) -> &VecDeque<u32> {
        &self.spike_history
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neuron_creation() {
        let neuron = Neuron::new(0);
        assert_eq!(neuron.id(), 0);
        assert_eq!(neuron.membrane_potential(), RESTING_POTENTIAL);
        assert!(!neuron.is_refractory());
    }

    #[test]
    fn test_receive_and_integrate() {
        let mut neuron = Neuron::new(0);
        neuron.receive_input(10.0);
        neuron.receive_input(20.0);
        neuron.integrate_inputs();
        
        // Should integrate the average: 15.0
        assert_eq!(neuron.membrane_potential(), RESTING_POTENTIAL + 15.0);
    }

    #[test]
    fn test_action_potential_generation() {
        let mut neuron = Neuron::new(0);
        
        // Bring to threshold
        neuron.receive_input(20.0);
        neuron.integrate_inputs();
        
        let fired = neuron.generate_action_potential(0);
        assert!(fired);
        assert_eq!(neuron.membrane_potential(), ACTION_POTENTIAL_PEAK);
        assert!(neuron.is_refractory());
    }

    #[test]
    fn test_synapse_connection() {
        let mut neuron = Neuron::new(0);
        neuron.connect_to(1, 0.8, Neurotransmitter::Glutamate);
        assert_eq!(neuron.synapse_count(), 1);
    }
}
