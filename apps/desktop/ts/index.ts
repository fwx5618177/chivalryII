import path from 'path';
import type { ClientConfig, DesktopClient } from './types';

// 直接使用 require 加载原生模块
const binding = require(path.join(__dirname, '../index.node'));

// 调试：打印出绑定对象的所有属性
console.log('Available methods:', Object.keys(binding));

// 如果方法名不匹配，可能需要调整这里的调用
export const createClient = (): DesktopClient => binding.createClient();  // 尝试使用 camelCase
export const useVulkan = (): boolean => binding.useVulkan();
export const getBuildMode = (): string => binding.getBuildMode();

export type { ClientConfig, DesktopClient };

// Example usage
async function main() {
    try {
        // 创建客户端实例
        const client = createClient();

        // 配置选项
        const config: ClientConfig = {
            width: 1280,
            height: 720,
            title: "Bevy Desktop Client",
            vsync: true
        };

        // 初始化
        await client.initialize(config);

        // 运行
        await client.run();

        // 清理资源
        await client.cleanup();
    } catch (error) {
        console.error('Error:', error);
        process.exit(1);
    }
}

// Only run if this file is being run directly
if (require.main === module) {
    main();
}
