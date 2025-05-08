import { ElLoading } from 'element-plus';

declare function openLoading(): ElLoading;
declare function closeLoading(): void;
declare function preloadResources(): void;

export { openLoading, closeLoading,preloadResources };