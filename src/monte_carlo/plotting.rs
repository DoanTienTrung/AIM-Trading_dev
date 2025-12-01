use anyhow::{Ok, Result};
use plotters::prelude::*;
use plotters_bitmap::bitmap_pixel::RGBPixel;
use plotters_bitmap::BitMapBackend;

const CHART_WIDTH: u32 = 800;
const CHART_HEIGHT: u32 = 600;

pub fn plot_price_paths(paths: &[Vec<f64>],  _model_type: &str) -> Result<(Vec<u8>, u32, u32)> {
    let mut buf = vec![0; (CHART_WIDTH * CHART_HEIGHT * 3) as usize];
    let backend = BitMapBackend::<RGBPixel>::with_buffer_and_format(
        &mut buf, (CHART_WIDTH, CHART_HEIGHT))?;
    {
        let root = backend.into_drawing_area();
        root.fill(&RGBColor(30, 30, 46))?;

        if paths.is_empty() || paths[0].is_empty() {
            root.draw(&EmptyElement::at((0,0)))?;
            return Ok((vec![0; (CHART_WIDTH * CHART_HEIGHT * 3) as usize], CHART_WIDTH, CHART_HEIGHT));
        }

        // Find best, median, and worst paths based on terminal prices
        let terminal_prices: Vec<(usize, f64)> = paths.iter()
            .enumerate()
            .map(|(idx, path)| (idx, *path.last().unwrap()))
            .collect();
        
        let mut sorted_indices: Vec<usize> = (0..terminal_prices.len()).collect();
        sorted_indices.sort_by(|&a, &b| {
            terminal_prices[a].1.partial_cmp(&terminal_prices[b].1).unwrap()
        });
        
        let worst_idx = sorted_indices[0];
        let best_idx = sorted_indices[sorted_indices.len() - 1];
        let median_idx = sorted_indices[sorted_indices.len() / 2];

        let mut min_price = paths[0][0];
        let mut max_price = paths[0][0];
        for path in paths.iter() {
            for &price in path.iter() {
                if price < min_price {
                    min_price = price;
                }
                if price > max_price {
                    max_price = price;
                }
            }
        }
        
        //add padding
        min_price *= 0.95;
        max_price *= 1.05;

        let max_steps = paths[0].len() - 1;

        let mut chart = ChartBuilder::on(&root)
            .caption(
                "Simulated Price Paths",
                ("Inter", 30, &RGBColor(208, 208, 208)),
            )
            .margin(10)
            .x_label_area_size(40)
            .y_label_area_size(60)
            .build_cartesian_2d(0..max_steps, min_price..max_price)?;

        chart
            .configure_mesh()
            .axis_style(&RGBColor(208, 208, 208))
            .label_style(("Inter", 15, &RGBColor(208, 208, 208)))
            .draw()?;

        // Collect indices to skip (the special paths)
        let special_indices: std::collections::HashSet<usize> = [best_idx, median_idx, worst_idx].iter().cloned().collect();
        
        // Draw all paths except the special ones with muted color
        // Limit to 50 paths for performance, but always include special ones
        let max_paths_to_draw = 50.min(paths.len());
        for idx in 0..max_paths_to_draw {
            if !special_indices.contains(&idx) {
                chart.draw_series(LineSeries::new(
                    paths[idx].iter().enumerate().map(|(i, &p)| (i, p)),
                    &YELLOW.mix(0.3),
                ))?;
            }
        }
        
        // Also draw special paths if they're beyond the first 50
        for &special_idx in &[best_idx, median_idx, worst_idx] {
            if special_idx >= max_paths_to_draw && special_idx < paths.len() {
                // This path will be drawn below with its special color
            }
        }

        // Draw worst case path (RED)
        if worst_idx < paths.len() {
            chart.draw_series(LineSeries::new(
                paths[worst_idx].iter().enumerate().map(|(i, &p)| (i, p)),
                &RED.mix(0.9),
            ))?
                .label("Worst Case")
                .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
        }

        // Draw median case path (BLUE)
        if median_idx < paths.len() {
            chart.draw_series(LineSeries::new(
                paths[median_idx].iter().enumerate().map(|(i, &p)| (i, p)),
                &BLUE.mix(0.9),
            ))?
                .label("Median Case")
                .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));
        }

        // Draw best case path (GREEN)
        if best_idx < paths.len() {
            chart.draw_series(LineSeries::new(
                paths[best_idx].iter().enumerate().map(|(i, &p)| (i, p)),
                &GREEN.mix(0.9),
            ))?
                .label("Best Case")
                .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));
        }

        // Configure legend (always show for best/median/worst paths)
        // Position legend at top-left corner
        chart.configure_series_labels()
            .background_style(&RGBColor(30, 30, 46).mix(0.8))
            .border_style(&RGBColor(208, 208, 208))
            .position(SeriesLabelPosition::UpperLeft)
            .draw()?;
    }

    Ok((buf, CHART_WIDTH, CHART_HEIGHT))
}

pub fn plot_histogram(data: &[f64], num_bins: usize) -> Result<(Vec<u8>, u32, u32)> {
    let mut buf = vec![0; (CHART_WIDTH * CHART_HEIGHT * 3) as usize];
    let backend = BitMapBackend::<RGBPixel>::with_buffer_and_format(
        &mut buf,
        (CHART_WIDTH, CHART_HEIGHT),
    )?;

    {
        let root = backend.into_drawing_area();
        root.fill(&RGBColor(30, 30, 46))?;

        if data.is_empty() {
            root.draw(&EmptyElement::at((0, 0)))?;
            return Ok((vec![0; (CHART_WIDTH * CHART_HEIGHT * 3) as usize], CHART_WIDTH, CHART_HEIGHT));
        }

        let min_val = *data
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        let max_val = *data
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();

        let bin_width = (max_val - min_val) / num_bins as f64;
        let mut bins = vec![0; num_bins];
        for &val in data {
            let bin = ((val - min_val) / bin_width).floor() as usize;
            let bin_idx = (bin).min(num_bins - 1); 
            bins[bin_idx] += 1;
        }
        
        let max_count = *bins.iter().max().unwrap_or(&1) as u32;
        
        let x_spec = (min_val..max_val).step(bin_width);
        
        let mut chart = ChartBuilder::on(&root)
            .caption(
                "Terminal Price Distribution",
                ("Inter", 30, &RGBColor(208, 208, 208)),
            )
            .margin(10)
            .x_label_area_size(40)
            .y_label_area_size(60)
            .build_cartesian_2d(
                x_spec, 
                0..max_count, 
            )?;
        
        chart.draw_series(
            bins.iter().enumerate().map(|(i, &count)| {
                let x_start = min_val + i as f64 * bin_width;
                let x_end = x_start + bin_width;
                let mut rect = Rectangle::new(
                    [(x_start, 0), (x_end, count)],
                    GREEN.mix(0.5).filled(),
                );
                rect.set_margin(0, 0, 1, 1);
                rect
            })
        )?;
        
        chart
            .configure_mesh()
            .axis_style(&RGBColor(208, 208, 208))
            .label_style(("Inter", 15, &RGBColor(208, 208, 208)))
            .draw()?;
    }

    Ok((buf, CHART_WIDTH, CHART_HEIGHT))
}

// #[cfg(test)]
// mod tests {
//     use crate::{SimParams, core_sim::run_simulation};

//     #[test]
//     fn test_gbm_reproducibility() {
//         let params = SimParams {
//             initial_price: 100.0,
//             horizon: 30,
//             num_paths: 10,
//             mu: 0.0002,
//             sigma: 0.015,
//             seed: 12345,
//             use_antithetic: false,
//             dt: 1,
//             model_type: "GBM".to_string().into(),
//         };
        
//         let result1 = run_simulation(params.clone(), vec![]).unwrap();
//         let result2 = run_simulation(params, vec![]).unwrap();
        
//         assert_eq!(result1.0.mean, result2.0.mean);
//     }
// }