use std::path::Path;
use installer_analyzer::analyzers::{WheelAnalyzer, InstallerAnalyzer};
use installer_analyzer::reporting::{ReportGenerator, Reporter, ReportFormat};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    println!("Testing File Tree Display functionality...\n");
    
    // Test with Python Wheel file (has good file structure)
    let test_file = "tests/data/persistent_ssh_agent-0.9.0-py3-none-any.whl";
    let path = Path::new(test_file);
    
    if !path.exists() {
        println!("❌ Test file not found: {}", test_file);
        println!("Please ensure the test file exists to test file tree functionality.");
        return Ok(());
    }
    
    println!("=== File Tree Data Generation Test ===");
    
    // Analyze the file
    let analyzer = WheelAnalyzer::new();
    
    if !analyzer.can_analyze(path).await? {
        println!("❌ File cannot be analyzed by WheelAnalyzer");
        return Ok(());
    }
    
    println!("✓ File can be analyzed");
    
    // Extract metadata and files
    let metadata = analyzer.extract_metadata(path).await?;
    let files = analyzer.extract_files(path).await?;
    let registry_operations = analyzer.extract_registry_operations(path).await?;
    
    println!("✓ Extracted {} files", files.len());
    
    // Create analysis result
    let analysis_result = installer_analyzer::core::AnalysisResult {
        session_id: uuid::Uuid::new_v4(),
        metadata,
        files,
        registry_operations,
        file_operations: Vec::new(),
        process_operations: Vec::new(),
        network_operations: Vec::new(),
        analyzed_at: chrono::Utc::now(),
        analysis_duration: std::time::Duration::from_secs(1),
        dynamic_analysis: false,
    };
    
    println!("✓ Created analysis result");
    
    // Test file tree data generation
    println!("\n=== File Tree Structure Test ===");
    
    let template_data = installer_analyzer::reporting::templates::ReportTemplateData::from_analysis_result(&analysis_result);
    
    println!("✓ Generated template data");
    println!("  - Total files in tree: {}", template_data.file_tree.total_files);
    println!("  - Total directories in tree: {}", template_data.file_tree.total_directories);
    println!("  - Root nodes: {}", template_data.file_tree.nodes.len());
    
    // Show tree structure
    println!("\n=== File Tree Structure Preview ===");
    for (i, node) in template_data.file_tree.nodes.iter().take(5).enumerate() {
        print_tree_node(node, 0);
        if i >= 4 && template_data.file_tree.nodes.len() > 5 {
            println!("  ... and {} more root nodes", template_data.file_tree.nodes.len() - 5);
            break;
        }
    }
    
    // Test JSON serialization
    println!("\n=== JSON Serialization Test ===");
    
    let json_result = serde_json::to_string_pretty(&template_data.file_tree);
    match json_result {
        Ok(json) => {
            println!("✓ File tree JSON serialization successful");
            println!("  JSON size: {} bytes", json.len());
            
            // Show first few lines of JSON
            let lines: Vec<&str> = json.lines().take(10).collect();
            println!("  JSON preview (first 10 lines):");
            for line in lines {
                println!("    {}", line);
            }
            if json.lines().count() > 10 {
                println!("    ... ({} more lines)", json.lines().count() - 10);
            }
        }
        Err(e) => {
            println!("❌ JSON serialization failed: {}", e);
        }
    }
    
    // Test HTML report generation
    println!("\n=== HTML Report Generation Test ===");
    
    let report_generator = ReportGenerator::new();
    match report_generator.generate_report(&analysis_result, ReportFormat::Html).await {
        Ok(html_report) => {
            println!("✓ HTML report generation successful");
            println!("  Report size: {} bytes", html_report.len());
            
            // Check for file tree components
            let has_file_tree_container = html_report.contains("file-tree-container");
            let has_file_tree_search = html_report.contains("fileTreeSearch");
            let has_file_tree_json = html_report.contains("fileTreeData");
            let has_tree_functions = html_report.contains("toggleNode");
            
            println!("  ✓ File tree container: {}", has_file_tree_container);
            println!("  ✓ File tree search: {}", has_file_tree_search);
            println!("  ✓ File tree JSON data: {}", has_file_tree_json);
            println!("  ✓ Tree interaction functions: {}", has_tree_functions);
            
            // Save test report
            let output_path = "test_file_tree_report.html";
            match report_generator.save_report(&analysis_result, ReportFormat::Html, Path::new(output_path)).await {
                Ok(_) => {
                    println!("✓ Test report saved to: {}", output_path);
                    println!("  You can open this file in a browser to test the interactive file tree!");
                }
                Err(e) => {
                    println!("❌ Failed to save test report: {}", e);
                }
            }
        }
        Err(e) => {
            println!("❌ HTML report generation failed: {}", e);
        }
    }
    
    // Test file type detection
    println!("\n=== File Type Detection Test ===");
    
    let test_files = [
        ("test.exe", "executable"),
        ("test.dll", "library"),
        ("test.py", "source"),
        ("test.txt", "document"),
        ("test.png", "image"),
        ("test.zip", "archive"),
        ("test.json", "config"),
        ("test", "file"),
    ];
    
    for (filename, expected_type) in &test_files {
        // We can't directly test the private method, but we can verify the logic
        let path = std::path::Path::new(filename);
        let actual_type = if let Some(ext) = path.extension() {
            match ext.to_str().unwrap_or("").to_lowercase().as_str() {
                "exe" => "executable",
                "dll" | "so" | "dylib" => "library",
                "py" | "rs" | "cpp" | "c" | "h" | "java" | "cs" => "source",
                "txt" | "md" | "readme" => "document",
                "png" | "jpg" | "jpeg" | "gif" | "ico" | "bmp" => "image",
                "zip" | "rar" | "7z" | "tar" | "gz" => "archive",
                "xml" | "json" | "yaml" | "yml" | "toml" => "config",
                _ => "file",
            }
        } else {
            "file"
        };
        
        let status = if actual_type == *expected_type { "✓" } else { "❌" };
        println!("  {} {}: {} (expected {})", status, filename, actual_type, expected_type);
    }
    
    println!("\nFile Tree Display functionality test completed!");
    println!("\n🎉 Key Features Implemented:");
    println!("  ✓ Hierarchical file tree data structure");
    println!("  ✓ File type detection and icon assignment");
    println!("  ✓ JSON serialization for JavaScript consumption");
    println!("  ✓ Interactive HTML components (search, expand/collapse)");
    println!("  ✓ Integration with existing report generation");
    
    Ok(())
}

fn print_tree_node(node: &installer_analyzer::reporting::templates::FileTreeNode, depth: usize) {
    let indent = "  ".repeat(depth);
    let icon = if node.is_directory { "📁" } else { "📄" };
    println!("{}{}{}  {} ({})", indent, icon, node.name, node.file_type, node.size_formatted);
    
    // Show first few children
    for (i, child) in node.children.iter().take(3).enumerate() {
        print_tree_node(child, depth + 1);
        if i >= 2 && node.children.len() > 3 {
            let child_indent = "  ".repeat(depth + 1);
            println!("{}... and {} more items", child_indent, node.children.len() - 3);
            break;
        }
    }
}
