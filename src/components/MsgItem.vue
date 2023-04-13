<script setup lang="ts">
import { computed } from 'vue';

// 上级组件传入的参数定义
const props = defineProps({
    // 消息内容
    message: {
        type: String,
        required: true
    },
    // 是否为自己的消息
    isSelf: {
        type: Boolean,
        default: false
    },
    // 消息发送的时间
    msgTime: {
        type: String,
        required: true
    },
    // 用户昵称
    nickname: {
        type: String,
        required: true
    },
    // 用户头像
    avatar: {
        type: String,
        default: "avatar11"
    }
});

const textStyle = computed(() => {

});
</script>
<template>
    <el-row>
        <el-col v-if="isSelf" :span="24" class="msg-item" style="justify-content: end;" >
            <div class="msg-content" style="margin-left: 0;margin-right: 10px;align-items: end;">
                <div class="msg-info" style="align-items: end;">
                    <span class="msg-time" style="margin-left: 0; ">{{ msgTime }}</span>
                    <span class="msg-name" style="margin-left: 10px; margin-right: 0;">{{ nickname }}</span>
                </div>
                <div class="msg-text" style="background-color: palegreen;">
                    {{ message }}
                </div>
            </div>
            <img class="icon" src="/avatars/avatar11.svg" />
        </el-col>
        <el-col v-else :span="24" class="msg-item">
            <img class="icon" src="/avatars/avatar11.svg" />
            <div class="msg-content">
                <div class="msg-info">
                    <span class="msg-name">{{ nickname }}</span>
                    <span class="msg-time">{{ msgTime }}</span>
                </div>
                <div class="msg-text">
                    {{ message }}
                </div>
            </div>
        </el-col>
    </el-row>

</template>
<style scoped>
.msg-item {
    width: 100%;
    min-height: 30px;
    margin-bottom: 20px;
    display: flex;
    flex-direction: row;
    align-items: start;
    justify-content: start;
}

.msg-item .icon {
    width: 28px;
    height: 28px;
}

.msg-item .msg-info {
    display: flex;
    flex-direction: row;
    align-items: start;
    justify-content: start;
}

.msg-item .msg-name {
    font-size: 14px;
    font-weight: 600;
}


.msg-item .msg-time {
    font-size: 11px;
    margin-left: 10px;
}

.msg-item .msg-content {
    margin-left: 10px;
    display: flex;
    flex-direction: column;
    align-items: start;
    justify-content: start;
}

.msg-item .msg-text {
    background-color: #fff;
    padding: 10px;
    max-width: 360px;
    border-radius: 5px;
}
</style>