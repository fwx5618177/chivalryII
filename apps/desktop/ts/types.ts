export interface ClientConfig {
    width: number;
    height: number;
    title: string;
    vsync: boolean;
}

// 这里需要从生成的 napi 绑定中导入实际的类型
export interface DesktopClient {
    initialize(config?: ClientConfig): Promise<void>;
    run(): Promise<void>;
    cleanup(): Promise<void>;
}

// 声明从 native 模块导出的函数类型
export declare function createClient(): DesktopClient;
export declare function useVulkan(): boolean;
export declare function getBuildMode(): string; 