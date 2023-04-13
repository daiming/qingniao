<script setup lang="ts">
import MsgForm from "./MsgForm.vue";
import InputForm from "./InputForm.vue";
import { computed, ref } from "vue";
import { Message } from "../interfaces/ObjectDefines";

const props = defineProps({
    nickname: {
        type: String,
        required: true
    }
});


const msgList = ref<Message[]>([
    {
        message: "在某些情况下，需要立即封存患者相关的所有未归档病案，封存文件为锁定状态，无法解锁，无法编辑修改。",
        nickname: "黑山老妖",
        avatar: "avatar01",
        msgTime: "2022-03-01 12:22:34",
        isSelf: false
    },
    {
        message: "病案封存通过系统赋予某种角色权限完成。",
        nickname: "黑山老妖",
        avatar: "avatar02",
        msgTime: "2022-03-01 12:22:34",
        isSelf: false
    },
    {
        message: "2.文件封存时能实时采集各业务系统中与该患者相关的当前有效数据，生成病案文档，并保存，产生独立的文档版本编号。",
        nickname: "钢铁侠",
        avatar: "avatar11",
        msgTime: "2022-03-01 12:22:34",
        isSelf: true
    },
    {
        message: "好。",
        nickname: "蜘蛛侠",
        avatar: "avatar13",
        msgTime: "2022-03-01 12:22:34",
        isSelf: false
    },


]);

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