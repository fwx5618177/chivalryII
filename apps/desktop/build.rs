use std::env;

fn main() {
    // 获取目标平台
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    match target_os.as_str() {
        "macos" => {
            println!("cargo:rustc-cfg=macos");
            // 设置 MoltenVK 的 ICD 文件路径
            println!("cargo:rustc-env=VK_ICD_FILENAMES=/usr/local/share/vulkan/icd.d/MoltenVK_icd.json");
        },
        "linux" => {
            println!("cargo:rustc-cfg=linux");
            println!("cargo:rustc-env=VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/intel_icd.x86_64.json:/usr/share/vulkan/icd.d/radeon_icd.x86_64.json");
        },
        "windows" => {
            println!("cargo:rustc-cfg=windows");
        },
        _ => panic!("Unsupported OS"),
    }

    // 如果当前为 debug 模式，则设置 Vulkan validation layers 的路径
    if env::var("PROFILE").unwrap() == "debug" {
        println!("cargo:rustc-env=VK_LAYER_PATH=/usr/local/share/vulkan/explicit_layer.d");
    }
}
