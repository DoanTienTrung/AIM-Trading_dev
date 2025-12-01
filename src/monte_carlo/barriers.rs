

/// Event when a barrier (stop-loss or target) is hit
#[derive(Debug, Clone)]
pub struct BarrierEvent {
    pub hit: bool,
    pub time_step: Option<usize>,
    pub price_at_hit: Option<f64>,
}

impl BarrierEvent {
    /// Create a new barrier event that was not hit
    pub fn not_hit() -> Self {
        Self {
            hit: false,
            time_step: None,
            price_at_hit: None,
        }
    }

    /// Create a new barrier event that was hit
    pub fn hit_at(time_step: usize, price: f64) -> Self {
        Self {
            hit: true,
            time_step: Some(time_step),
            price_at_hit: Some(price),
        }
    }
}

/// Check if stop-loss barrier is hit in a price path
pub fn check_stop_loss(path: &[f64], stop_loss: f64) -> BarrierEvent {
    for (step, &price) in path.iter().enumerate() {
        if price <= stop_loss {
            return BarrierEvent::hit_at(step, price);
        }
    }
    BarrierEvent::not_hit()
}

/// Check if target barrier is hit in a price path
pub fn check_target(path: &[f64], target: f64) -> BarrierEvent {
    for (step, &price) in path.iter().enumerate() {
        if price >= target {
            return BarrierEvent::hit_at(step, price);
        }
    }
    BarrierEvent::not_hit()
}

/// Check both stop-loss and target barriers
/// Returns (stop_loss_event, target_event)
/// If both are hit, returns the one that occurred first
pub fn check_barriers(
    path: &[f64],
    stop_loss: Option<f64>,
    target: Option<f64>,
) -> (Option<BarrierEvent>, Option<BarrierEvent>) {
    let mut sl_event = None;
    let mut target_event = None;

    // Check stop loss
    if let Some(sl) = stop_loss {
        let event = check_stop_loss(path, sl);
        if event.hit {
            sl_event = Some(event);
        }
    }

    // Check target
    if let Some(tgt) = target {
        let event = check_target(path, tgt);
        if event.hit {
            target_event = Some(event);
        }
    }

    (sl_event, target_event)
}

/// Calculate hit probabilities and average times from multiple paths
pub fn calculate_hit_statistics(
    paths: &[Vec<f64>],
    stop_loss: Option<f64>,
    target: Option<f64>,
) -> (f64, f64, Option<f64>, Option<f64>) {
    if paths.is_empty() {
        return (0.0, 0.0, None, None);
    }

    let mut sl_hits = 0;
    let mut target_hits = 0;
    let mut sl_times = Vec::new();
    let mut target_times = Vec::new();

    for path in paths {
        let (sl_event, target_event) = check_barriers(path, stop_loss, target);

        if let Some(event) = sl_event {
            if event.hit {
                sl_hits += 1;
                if let Some(time) = event.time_step {
                    sl_times.push(time as f64);
                }
            }
        }

        if let Some(event) = target_event {
            if event.hit {
                target_hits += 1;
                if let Some(time) = event.time_step {
                    target_times.push(time as f64);
                }
            }
        }
    }

    let total_paths = paths.len() as f64;
    let prob_hit_stoploss = sl_hits as f64 / total_paths;
    let prob_hit_target = target_hits as f64 / total_paths;

    let avg_time_to_stoploss = if !sl_times.is_empty() {
        Some(sl_times.iter().sum::<f64>() / sl_times.len() as f64)
    } else {
        None
    };

    let avg_time_to_target = if !target_times.is_empty() {
        Some(target_times.iter().sum::<f64>() / target_times.len() as f64)
    } else {
        None
    };

    (prob_hit_stoploss, prob_hit_target, avg_time_to_stoploss, avg_time_to_target)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stop_loss_hit() {
        let path = vec![100.0, 95.0, 90.0, 85.0, 80.0]; // Declining path
        let event = check_stop_loss(&path, 92.0);
        
        assert!(event.hit);
        assert_eq!(event.time_step, Some(1)); // Hit at step 1 (price 95.0)
        assert_eq!(event.price_at_hit, Some(95.0));
    }

    #[test]
    fn test_stop_loss_not_hit() {
        let path = vec![100.0, 105.0, 110.0, 115.0, 120.0]; // Rising path
        let event = check_stop_loss(&path, 95.0);
        
        assert!(!event.hit);
        assert_eq!(event.time_step, None);
        assert_eq!(event.price_at_hit, None);
    }

    #[test]
    fn test_target_hit() {
        let path = vec![100.0, 105.0, 110.0, 115.0, 120.0]; // Rising path
        let event = check_target(&path, 112.0);
        
        assert!(event.hit);
        assert_eq!(event.time_step, Some(3)); // Hit at step 3 (price 115.0)
        assert_eq!(event.price_at_hit, Some(115.0));
    }

    #[test]
    fn test_both_barriers() {
        let path = vec![100.0, 95.0, 105.0, 110.0, 115.0]; // V-shaped path
        let (sl_event, target_event) = check_barriers(&path, Some(97.0), Some(108.0));
        
        // Stop loss hit first at step 1
        assert!(sl_event.is_some());
        assert!(sl_event.as_ref().unwrap().hit);
        assert_eq!(sl_event.as_ref().unwrap().time_step, Some(1));
        
        // Target also hit at step 3
        assert!(target_event.is_some());
        assert!(target_event.as_ref().unwrap().hit);
        assert_eq!(target_event.as_ref().unwrap().time_step, Some(3));
    }

    #[test]
    fn test_hit_statistics() {
        let paths = vec![
            vec![100.0, 95.0, 90.0, 85.0], // Hits SL at step 1
            vec![100.0, 105.0, 110.0, 115.0], // Hits target at step 3
            vec![100.0, 102.0, 104.0, 106.0], // Hits neither
        ];

        let (prob_sl, prob_target, avg_sl_time, avg_target_time) = 
            calculate_hit_statistics(&paths, Some(97.0), Some(112.0));

        assert_eq!(prob_sl, 1.0 / 3.0); // 1 out of 3 paths hit SL
        assert_eq!(prob_target, 1.0 / 3.0); // 1 out of 3 paths hit target
        assert_eq!(avg_sl_time, Some(1.0)); // Average time to SL is 1
        assert_eq!(avg_target_time, Some(3.0)); // Average time to target is 3
    }
}