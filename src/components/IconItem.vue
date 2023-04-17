<script setup lang="ts">
import { computed, ref } from "vue";
// 自定义事件
const emit = defineEmits<{
    // 用户点击事件，反馈给上级组件
    (e: 'itemClieck', ipaddr: string): void
}>()
// 上级组件传入的参数定义
const props = defineProps({
    ipaddr: {
        type: String,
        required: true
    },
    // 是否为选中状态
    is_checked: {
        type: Boolean,
        default: false
    },
    // 未读消息的数量
    msg_count: {
        type: Number,
        default: 0
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

console.log('传入的参数：', props);

// 设置被选中的样式
const itemStyle = computed(() => {
    return props.is_checked ? "item-style item-style-checked" : "item-style";
});
// 计算是否要显示用户消息徽章
const isShowBadge = computed(() => {
    return props.msg_count > 0 ? false : true;
});
// 设置用户头像地址
const avatarUrl = computed(() => {
    return "/avatars/" + props.avatar + ".svg";
});

// 处理点击事件
function handleChecked() {
    // 触发点击事件
    emit("itemClieck", props.ipaddr)
}

</script>
<template>
    <div :class="itemStyle" @click="handleChecked">
        <div class="icon-box">
            <el-badge :value="props.msg_count" :hidden="isShowBadge" class="badge-img">
                <img class="icon-img" :src="avatarUrl" />
            </el-badge>
        </div>
        <div class="text-box">
            <el-text class="text" truncated>{{ props.nickname }}</el-text>
        </div>
    </div>
</template>

<style scoped>
.item-style {
    height: 60px;
    line-height: 60px;
    width: 100%;
    border-bottom: 1px solid #eee;
    display: flex;
    flex-direction: row;
    justify-content: start;
    align-items: center;
    cursor: pointer;
}

.item-style-checked {
    background-color: #eee;
}

.item-style .icon-box {
    width: 60px;
    height: 100%;
    display: flex;
    flex-direction: row;
    justify-content: center;
    align-items: center;
}

.item-style .icon-box .item {
    height: 28px;
}

.item-style .icon-box .badge-img {
    display: flex;
    flex-direction: column;
    justify-content: center;
}

.item-style .icon-box .badge-img .icon-img {
    height: 28px;
    width: 28px;
}

.item-style .text-box {
    width: 140px;
    height: 100%;
    display: flex;
    flex-direction: row;
    justify-content: start;
    align-items: center;
}

.item-style .text-box .text {
    font-size: medium;
    width: 140px;
}
</style>