<script setup lang="ts">
import MsgItem from "./MsgItem.vue";
import { Message } from "../interfaces/ObjectDefines";
// 上级组件传入的参数定义
const props = defineProps<{
    msgList: Message[]
}>();

</script>
<template>
    <el-row class="mf-max-box">
        <el-col :span="24" class="msg-list" >
            <MsgItem v-for="msg in msgList" v-bind="msg" />
        </el-col>
    </el-row>
</template>
<style scoped>
.mf-max-box {
    width: 100%;
    padding: 20px;
}
.msg-list{
    display: flex;
    flex-direction: column;
    justify-content: end;
}
</style>