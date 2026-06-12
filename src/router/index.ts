import { createRouter, createWebHashHistory } from 'vue-router';
import MainView from '../views/MainView.vue';
import FileClassifierView from '../views/FileClassifierView.vue';
import LauncherWindow from '../views/LauncherWindow.vue';

const routes = [
  {
    path: '/',
    name: 'main',
    component: MainView
  },
  {
    path: '/file-classifier',
    name: 'file-classifier',
    component: FileClassifierView
  },
  {
    path: '/launcher',
    name: 'launcher',
    component: LauncherWindow
  }
];

const router = createRouter({
  history: createWebHashHistory(),
  routes
});

export default router;
