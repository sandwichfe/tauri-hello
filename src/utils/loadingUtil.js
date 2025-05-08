import { ElLoading } from 'element-plus'

let loadingInstance = null;

// 预加载资源函数
export function preloadResources() {
    return new Promise((resolve) => {
        // 这里可以添加预加载图片、字体等资源的逻辑
        // 例如预加载常用图片
        const imagesToPreload = [
            // 添加需要预加载的图片路径
            // 'path/to/image1.png',
            // 'path/to/image2.png'
        ];
        
        let loadedCount = 0;
        const totalImages = imagesToPreload.length;
        
        // 如果没有图片需要预加载，直接完成
        if (totalImages === 0) {
            resolve();
            return;
        }
        
        // 预加载每张图片
        imagesToPreload.forEach(src => {
            const img = new Image();
            img.onload = img.onerror = () => {
                loadedCount++;
                if (loadedCount === totalImages) {
                    resolve();
                }
            };
            img.src = src;
        });
    });
}

export function openLoading(){
    loadingInstance = ElLoading.service({
        lock: true,
        text: '加载中...',
        background: 'rgba(31, 32, 33, 0.3)',
    });
}

export function closeLoading(){
    if (loadingInstance) {
        loadingInstance.close();
        loadingInstance = null;
    }
}