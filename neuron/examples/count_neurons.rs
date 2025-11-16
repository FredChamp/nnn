// Calcul du nombre de neurones pour une image 64x64

fn main() {
    let width = 64;
    let height = 64;
    
    // 1. CÔNES (Photorécepteurs)
    // Un cône par pixel
    let cones = width * height;
    println!("🔵 Cônes (photorécepteurs): {}", cones);
    
    // 2. CELLULES GANGLIONNAIRES
    // GanglionLayer::new(width, height, spacing=4, center_radius, surround_radius)
    // Espacement de 4 pixels
    let spacing_ganglion = 4;
    let positions_x = (0..width).step_by(spacing_ganglion).count();
    let positions_y = (0..height).step_by(spacing_ganglion).count();
    let ganglion_positions = positions_x * positions_y;
    // 2 types par position (ON-center et OFF-center)
    let ganglion_cells = ganglion_positions * 2;
    println!("🟢 Cellules ganglionnaires: {} (ON: {}, OFF: {})", 
             ganglion_cells, ganglion_positions, ganglion_positions);
    
    // 3. CORTEX V1
    // V1Cortex::new(width, height, spacing=8, rf_size=5)
    let spacing_v1 = 8;
    let rf_size = 5;
    
    // Positions valides (en évitant les bords)
    let start = rf_size;
    let end_x = width - rf_size;
    let end_y = height - rf_size;
    
    let v1_positions_x = (start..end_x).step_by(spacing_v1).count();
    let v1_positions_y = (start..end_y).step_by(spacing_v1).count();
    let v1_positions = v1_positions_x * v1_positions_y;
    
    // 4 orientations par position (0°, 45°, 90°, 135°)
    let orientations = 4;
    let v1_columns = v1_positions * orientations;
    
    // 2 neurones par colonne (1 simple + 1 complex)
    let neurons_per_column = 2;
    let v1_neurons = v1_columns * neurons_per_column;
    
    println!("🟡 Cortex V1:");
    println!("   - Positions: {}", v1_positions);
    println!("   - Colonnes (4 orientations): {}", v1_columns);
    println!("   - Neurones (simple + complex): {}", v1_neurons);
    
    // TOTAL
    let total = cones + ganglion_cells + v1_neurons;
    println!("\n📊 TOTAL: {} neurones", total);
    println!("   = {} cônes", cones);
    println!("   + {} ganglionnaires", ganglion_cells);
    println!("   + {} V1", v1_neurons);
    
    // Comparaison avec le cerveau humain
    println!("\n🧠 Pour comparaison:");
    println!("   Rétine humaine: ~126 millions de photorécepteurs");
    println!("   Cortex V1 humain: ~140 millions de neurones");
    println!("   Notre modèle: {} neurones ({:.2}% d'un œil humain)", 
             total, (total as f64 / 126_000_000.0) * 100.0);
}
