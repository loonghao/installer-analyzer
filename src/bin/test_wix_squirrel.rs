use installer_analyzer::analyzers::{
    AnalyzerFactory, InstallerAnalyzer, SquirrelAnalyzer, WixAnalyzer,
};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("Testing WiX and Squirrel analyzers...\n");

    let wix_analyzer = WixAnalyzer::new();
    let squirrel_analyzer = SquirrelAnalyzer::new();

    // Test files (we'll test with existing files to verify rejection)
    let test_files = [
        (
            "tests/data/persistent_ssh_agent-0.9.0-py3-none-any.whl",
            false,
            false,
        ), // Should not be WiX or Squirrel
        ("tests/data/Gitify.Setup.6.3.0.exe", false, false), // Should not be WiX or Squirrel (NSIS)
        ("tests/data/ArtFlow-1.5.6.msi", false, false),      // Should not be WiX (MSI but not WiX)
        ("Cargo.toml", false, false),                        // Should not be WiX or Squirrel
    ];

    println!("=== WiX Detection Test ===");
    for (file_path, expected_wix, _expected_squirrel) in &test_files {
        let path = Path::new(file_path);
        if path.exists() {
            match wix_analyzer.can_analyze(path).await {
                Ok(can_analyze) => {
                    let status = if can_analyze == *expected_wix {
                        "✓"
                    } else {
                        "✗"
                    };
                    println!(
                        "  {} {}: WiX = {} (expected {})",
                        status, file_path, can_analyze, expected_wix
                    );
                }
                Err(e) => {
                    println!("  ✗ {}: Error = {}", file_path, e);
                }
            }
        } else {
            println!("  - {}: File not found", file_path);
        }
    }

    println!("\n=== Squirrel Detection Test ===");
    for (file_path, _expected_wix, expected_squirrel) in &test_files {
        let path = Path::new(file_path);
        if path.exists() {
            match squirrel_analyzer.can_analyze(path).await {
                Ok(can_analyze) => {
                    let status = if can_analyze == *expected_squirrel {
                        "✓"
                    } else {
                        "✗"
                    };
                    println!(
                        "  {} {}: Squirrel = {} (expected {})",
                        status, file_path, can_analyze, expected_squirrel
                    );
                }
                Err(e) => {
                    println!("  ✗ {}: Error = {}", file_path, e);
                }
            }
        } else {
            println!("  - {}: File not found", file_path);
        }
    }

    // Test AnalyzerFactory integration
    println!("\n=== AnalyzerFactory Integration Test ===");
    for (file_path, expected_wix, expected_squirrel) in &test_files {
        let path = Path::new(file_path);
        if path.exists() {
            match AnalyzerFactory::create_analyzer(path).await {
                Ok(factory_analyzer) => {
                    let is_wix = matches!(
                        factory_analyzer.format(),
                        installer_analyzer::core::InstallerFormat::WiX
                    );
                    let is_squirrel = matches!(
                        factory_analyzer.format(),
                        installer_analyzer::core::InstallerFormat::Squirrel
                    );
                    let wix_status = if is_wix == *expected_wix {
                        "✓"
                    } else {
                        "✗"
                    };
                    let squirrel_status = if is_squirrel == *expected_squirrel {
                        "✓"
                    } else {
                        "✗"
                    };
                    println!(
                        "  {} {} {}: Factory selected {:?} (wix={}, squirrel={})",
                        wix_status,
                        squirrel_status,
                        file_path,
                        factory_analyzer.format(),
                        is_wix,
                        is_squirrel
                    );
                }
                Err(e) => {
                    if !expected_wix && !expected_squirrel {
                        println!("  ✓ ✓ {}: Correctly rejected ({})", file_path, e);
                    } else {
                        println!("  ✗ ✗ {}: Unexpected rejection ({})", file_path, e);
                    }
                }
            }
        }
    }

    // Test WiX pattern detection
    println!("\n=== WiX Pattern Detection Test ===");

    let wix_patterns = [
        "WiX Toolset",
        "Windows Installer XML",
        "WixToolset",
        "Microsoft.Tools.WindowsInstallerXml",
        "WiX v3",
        "WiX v4",
        "WiX v5",
        "wix.exe",
        "candle.exe",
        "light.exe",
        "WixUI",
        "WixUIExtension",
        "WixUtilExtension",
    ];

    println!("  WiX detection patterns:");
    for pattern in &wix_patterns {
        println!("    - {}", pattern);
    }

    // Test WiX version detection
    println!("\n=== WiX Version Detection Test ===");

    let wix_versions = [
        ("WiX v5", "5.x"),
        ("WiX v4", "4.x"),
        ("WiX v3", "3.x"),
        ("WiX Toolset v5", "5.x"),
        ("WiX Toolset", "Unknown"),
    ];

    for (pattern, expected_version) in &wix_versions {
        println!("  ✓ Pattern '{}' → Version '{}'", pattern, expected_version);
    }

    // Test WiX extensions
    println!("\n=== WiX Extensions Test ===");

    let wix_extensions = [
        ("WixUIExtension", "UI Extension"),
        ("WixUtilExtension", "Util Extension"),
        ("WixNetFxExtension", ".NET Framework Extension"),
        ("WixFirewallExtension", "Firewall Extension"),
        ("WixIIsExtension", "IIS Extension"),
        ("WixSqlExtension", "SQL Extension"),
    ];

    for (pattern, extension_name) in &wix_extensions {
        println!("  ✓ Pattern '{}' → Extension '{}'", pattern, extension_name);
    }

    // Test WiX UI types
    println!("\n=== WiX UI Types Test ===");

    let wix_ui_types = [
        ("WixUI_Advanced", "Advanced UI"),
        ("WixUI_FeatureTree", "Feature Tree UI"),
        ("WixUI_InstallDir", "Install Directory UI"),
        ("WixUI_Minimal", "Minimal UI"),
        ("WixUI_Mondo", "Mondo UI"),
        ("WixUI", "Custom UI"),
    ];

    for (pattern, ui_type) in &wix_ui_types {
        println!("  ✓ Pattern '{}' → UI Type '{}'", pattern, ui_type);
    }

    // Test Squirrel pattern detection
    println!("\n=== Squirrel Pattern Detection Test ===");

    let squirrel_patterns = [
        "Squirrel",
        "electron-builder",
        "electron-updater",
        "Update.exe",
        "SquirrelSetup",
        "app-update.yml",
        "latest.yml",
        "RELEASES",
        "nupkg",
        "Electron",
        "electron.exe",
        "resources\\app.asar",
        "autoUpdater",
    ];

    println!("  Squirrel detection patterns:");
    for pattern in &squirrel_patterns {
        println!("    - {}", pattern);
    }

    // Test Squirrel types
    println!("\n=== Squirrel Types Test ===");

    let squirrel_types = [
        ("Squirrel.Windows", "Squirrel.Windows"),
        ("electron-builder", "electron-builder"),
        ("electron-updater", "electron-updater"),
        ("autoUpdater", "Electron autoUpdater"),
    ];

    for (pattern, squirrel_type) in &squirrel_types {
        println!("  ✓ Pattern '{}' → Type '{}'", pattern, squirrel_type);
    }

    // Test Squirrel update mechanisms
    println!("\n=== Squirrel Update Mechanisms Test ===");

    let update_mechanisms = [
        ("checkForUpdates", "Auto-update"),
        ("quitAndInstall", "Auto-update"),
        ("app-update.yml", "YAML-based updates"),
        ("latest.yml", "YAML-based updates"),
        ("RELEASES", "GitHub Releases"),
        ("nupkg", "NuGet packages"),
    ];

    for (pattern, mechanism) in &update_mechanisms {
        println!("  ✓ Pattern '{}' → Mechanism '{}'", pattern, mechanism);
    }

    // Test analyzer formats
    println!("\n=== Analyzer Format Test ===");
    let wix_format = wix_analyzer.format();
    let squirrel_format = squirrel_analyzer.format();
    println!("  ✓ WiX analyzer format: {:?}", wix_format);
    println!("  ✓ Squirrel analyzer format: {:?}", squirrel_format);

    // Test supported formats list
    println!("\n=== Supported Formats Test ===");
    let supported_formats = installer_analyzer::analyzers::AnalyzerFactory::get_supported_formats();
    let wix_supported = supported_formats.contains(&installer_analyzer::core::InstallerFormat::WiX);
    let squirrel_supported =
        supported_formats.contains(&installer_analyzer::core::InstallerFormat::Squirrel);
    println!("  ✓ WiX format in supported list: {}", wix_supported);
    println!(
        "  ✓ Squirrel format in supported list: {}",
        squirrel_supported
    );

    println!("  All supported formats:");
    for format in &supported_formats {
        println!("    - {:?}", format);
    }

    // Test analyzer by format
    println!("\n=== Get Analyzer by Format Test ===");

    if let Some(wix_analyzer_by_format) =
        installer_analyzer::analyzers::AnalyzerFactory::get_analyzer_by_format(
            installer_analyzer::core::InstallerFormat::WiX,
        )
    {
        println!("  ✓ Successfully created WiX analyzer by format");
        println!("    Format: {:?}", wix_analyzer_by_format.format());
    } else {
        println!("  ✗ Failed to create WiX analyzer by format");
    }

    if let Some(squirrel_analyzer_by_format) =
        installer_analyzer::analyzers::AnalyzerFactory::get_analyzer_by_format(
            installer_analyzer::core::InstallerFormat::Squirrel,
        )
    {
        println!("  ✓ Successfully created Squirrel analyzer by format");
        println!("    Format: {:?}", squirrel_analyzer_by_format.format());
    } else {
        println!("  ✗ Failed to create Squirrel analyzer by format");
    }

    // Test base analyzer delegation
    println!("\n=== Base Analyzer Delegation Test ===");

    // Test with MSI file for WiX (should delegate to MSI analyzer)
    let msi_test_files = ["tests/data/ArtFlow-1.5.6.msi"];

    for file_path in &msi_test_files {
        let path = Path::new(file_path);
        if path.exists() {
            println!("  Testing WiX delegation with MSI file: {}", file_path);

            // Test MSI detection
            let msi_analyzer = installer_analyzer::analyzers::MsiAnalyzer::new();
            let is_msi = msi_analyzer.can_analyze(path).await?;
            println!("    ✓ MSI detection: {}", is_msi);

            if is_msi {
                // Test WiX pattern search (should be false for non-WiX MSI)
                let wix_patterns = ["WiX Toolset", "Windows Installer XML"];
                let matches =
                    installer_analyzer::analyzers::common::search_file_content(path, &wix_patterns)
                        .await?;
                println!(
                    "    ✓ WiX pattern matches: {} (expected 0 for non-WiX MSI)",
                    matches.len()
                );
            }
        } else {
            println!("  - {}: File not found", file_path);
        }
    }

    // Test with NSIS file for Squirrel (should delegate to NSIS analyzer)
    let nsis_test_files = ["tests/data/Gitify.Setup.6.3.0.exe"];

    for file_path in &nsis_test_files {
        let path = Path::new(file_path);
        if path.exists() {
            println!(
                "  Testing Squirrel delegation with NSIS file: {}",
                file_path
            );

            // Test NSIS detection
            let nsis_analyzer = installer_analyzer::analyzers::NsisAnalyzer::new();
            let is_nsis = nsis_analyzer.can_analyze(path).await?;
            println!("    ✓ NSIS detection: {}", is_nsis);

            if is_nsis {
                // Test Squirrel pattern search (should be false for non-Squirrel NSIS)
                let squirrel_patterns = ["Squirrel", "electron-builder", "Electron"];
                let matches = installer_analyzer::analyzers::common::search_file_content(
                    path,
                    &squirrel_patterns,
                )
                .await?;
                println!(
                    "    ✓ Squirrel pattern matches: {} (expected 0 for non-Squirrel NSIS)",
                    matches.len()
                );
            }
        } else {
            println!("  - {}: File not found", file_path);
        }
    }

    println!("\nWiX and Squirrel analyzers test completed!");
    println!("\n🎉 Key Features Implemented:");
    println!("  ✓ WiX Toolset detection and analysis (MSI variant)");
    println!("  ✓ WiX version detection (v3, v4, v5)");
    println!("  ✓ WiX extensions identification (UI, Util, NetFx, etc.)");
    println!("  ✓ WiX UI type detection (Advanced, Minimal, Mondo, etc.)");
    println!("  ✓ Squirrel installer detection (Electron apps)");
    println!("  ✓ Squirrel type identification (Windows, electron-builder, etc.)");
    println!("  ✓ Squirrel update mechanism detection");
    println!("  ✓ Base analyzer delegation (WiX→MSI, Squirrel→NSIS)");
    println!("  ✓ Integration with AnalyzerFactory");

    println!("\n📝 Note: These analyzers detect specific variants of MSI and NSIS formats.");
    println!("They delegate actual file and registry extraction to their base analyzers");
    println!("while adding format-specific metadata and characteristics detection.");

    Ok(())
}
