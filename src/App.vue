<template>
  <form class="form" @submit.prevent="greet">
    <div class="form-title">服务器设置</div>
    <div class="input-field">
      <label for="">服务器地址</label>
      <input id="input" v-model="serverAddr" placeholder="127.0.0.1" />
    </div>
    <div class="input-field">
      <label for="">服务器端口</label>
      <input id="input" v-model="serverPort" placeholder="7000" />
    </div>
    <div class="form-title">穿透配置</div>
    <div class="input-field">
      <label for="">名称</label>
      <input id="input" v-model="name" placeholder="web" />
    </div>
    <div class="input-field">
      <label for="">需要穿透的本地地址</label>
      <input id="input" v-model="localIp" placeholder="127.0.0.1" />
    </div>
    <div class="input-field">
      <label for="">需要穿透的本地端口</label>
      <input id="input" v-model="localPort" placeholder="8080" />
    </div>
    <div class="input-field">
      <label for="">外网访问端口</label>
      <input id="input" v-model="remotePort" placeholder="25635" />
    </div>
    <div class="button-group">
      <button class="full-width" @click="handleSave">保存配置</button>
      <button v-if="!isRunning" class="full-width primary" @click="handleRun">
        一键穿透
      </button>
      <button v-else class="full-width primary" @click="handleStop">
        暂停穿透
      </button>
    </div>
  </form>
</template>

<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const serverAddr = ref("127.0.0.1");
const serverPort = ref("7000");
const name = ref("web");
const localIp = ref("127.0.0.1");
const localPort = ref("22");
const remotePort = ref("6000");

const isRunning = ref(false);
invoke("get_config").then((res) => {
  serverAddr.value = res["server_addr"];
  serverPort.value = res["server_port"];
  name.value = res["name"];
  localIp.value = res["local_ip"];
  localPort.value = res["local_port"];
  remotePort.value = res["remote_port"];
});
const handleSave = () => {
  invoke("save_config", {
    serverAddr: serverAddr.value,
    serverPort: serverPort.value,
    name: name.value,
    localIp: localIp.value,
    localPort: localPort.value,
    remotePort: remotePort.value,
  }).then((res) => {
    console.log(res);
  });
};
const handleRun = () => {
  invoke("run_frpc").then((res) => {
    isRunning.value = true;
  });
};
const handleStop = () => {
  invoke("stop_frpc").then((res) => {
    isRunning.value = false;
  });
};
</script>

<style lang="scss" scoped></style>
