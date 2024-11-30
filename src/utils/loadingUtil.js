import { ElLoading } from 'element-plus'

export function openLoading(){
    ElLoading.service({
        lock: false,
        text: 'Loading',
        background: 'rgb(31 32 33 / 21%)',
        })
}

export function closeLoading(){
    ElLoading.service().close();
}