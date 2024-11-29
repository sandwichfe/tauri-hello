<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { ElMessage } from 'element-plus' 


const greetMsg = ref("");
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke("greet", { name: name.value });
  const result: string = await invoke('my_custom_command',{ command: "git --version" });

  console.log(result);
  greetMsg.value = result;
  ElMessage.success("ok")
}


async function enableGitProxy() {
  try {
    // 分开执行两个 git config 命令
    await invoke('my_custom_command', { command: "git config --global http.proxy http://127.0.0.1:7897" });
    await invoke('my_custom_command', { command: "git config --global https.proxy http://127.0.0.1:7897" });
    
    console.log("Proxy configured successfully");
    ElMessage.success("代理已设置");
  } catch (error) {
    console.error("Error configuring proxy:", error);
    ElMessage.error("Failed to configure proxy");
  }
}


async function closeGitProxy() {
  try {
    // 分开执行两个 git config 命令来取消代理
    await invoke('my_custom_command', { command: "git config --global --unset http.proxy" });
    await invoke('my_custom_command', { command: "git config --global --unset https.proxy" });
    
    // 代理取消成功后，提示成功信息
    ElMessage.success("代理已取消");
  } catch (error) {
    // 如果执行失败，捕获错误并显示失败信息
    console.error("Error removing proxy:", error);
    ElMessage.error("操作失败");
  }
}



async function getGitProxyInfo() {
  try {
    // 获取 HTTP 和 HTTPS 代理配置
    const result_http: string = await invoke('my_custom_command', { command: "git config --global --get http.proxy" });
    const result_https: string = await invoke('my_custom_command', { command: "git config --global --get https.proxy" });
    
    // 打印结果到控制台
    console.log(result_http);
    console.log(result_https);


    if (!result_http.trim() && !result_https.trim()) {
      ElMessage.success("未设置代理")
    } else {
      ElMessage.success({
     message: result_http + "<br>" + result_https,
     dangerouslyUseHTMLString: true
     });
  }


   

    
  } catch (error) {
    // 如果发生错误，显示错误消息
    ElMessage.error("An error occurred while retrieving the proxy configuration.");
    console.error(error);
  }
}



</script>

<template>
  <main class="container">
  
    <div class="row">
 
    </div>
    <p>Click on the Tauri, Vite, and Vue logos to learn more.</p>

    <form class="row" @submit.prevent="greet">
      <input id="greet-input" v-model="name" placeholder="Enter a name..." />
      <button type="submit">Greet</button>
    </form>
    <p>{{ greetMsg }}</p>

      <button @click="enableGitProxy">git开启本地代理</button>
      <button @click="closeGitProxy">git关闭本地代理</button>
      <button @click="getGitProxyInfo">git查看代理</button>
  </main>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}

</style>
<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}

</style>