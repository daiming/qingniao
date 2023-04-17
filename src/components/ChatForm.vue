<script setup lang="ts">
import MsgForm from "./MsgForm.vue";
import InputForm from "./InputForm.vue";
import { onMounted, ref } from "vue";
import { Message } from "../interfaces/ObjectDefines";
import { invoke } from "@tauri-apps/api/tauri";

const props = defineProps({
    nickname: {
        type: String,
        required: true
    }
});


const msgList = ref<Message[]>([
]);

onMounted(() => {
    getMessageList();
});


async function getMessageList() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  msgList.value = await invoke("get_user_message",{mac:"0000000000"});
  if(msgList.value.length > 0) {
    msgList.value.map(msg => {
        if(msg.is_self) {
            msg.nickname = "自己";
        }else{
            msg.nickname = props.nickname;
        }
    }  );

  }
}

</script>

<template>
    
    <el-container class="cf-max-box">
        <!-- <el-header class="header">{{nickname}}</el-header> -->
        <el-main class="msg-form">
            <MsgForm :msg-list="msgList" />
        </el-main>
        <el-footer class="input-form">
            <InputForm />
        </el-footer>
    </el-container>
</template>
<style scoped>
.header {
    text-align: left;
    width: 100%;
    height: 30px;
    font-weight: 600;
}
.cf-max-box {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: start;
    width: 100%;
    height: 100%;
    padding: 0;
    margin: 0;
}

.msg-form {
    display: flex;
    flex-direction: column;
    align-items: start;
    justify-content: start;
    width: 100%;
    padding: 0;
    margin: 0;
}

.input-form {
    display: flex;
    flex-direction: column;
    align-items: start;
    justify-content: start;
    height: 120px;
    width: 100%;
    padding: 0;
    margin: 0;
}
</style>