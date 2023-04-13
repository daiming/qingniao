<script setup lang="ts">
import { fa } from "element-plus/es/locale";
import IconItem from "./IconItem.vue";
import Profile from "./Profile.vue";
import ChatForm from "./ChatForm.vue";
import { computed, ref } from "vue";

// 用户列表
interface User {
    ipaddr: string,
    nickname: string,
    avatar?: string,
    msgCount?: number,
    isChecked?: boolean
}

const userList = ref<User[]>([
    {
        ipaddr: "192.168.12.1",
        nickname: "192.168.12.1",
        avatar: "avatar01",
        msgCount: 0
    },
    {
        ipaddr: "192.168.12.2",
        nickname: "192.168.12.2",
        avatar: "avatar12",
        msgCount: 2
    },
    {
        ipaddr: "192.168.12.3",
        nickname: "192.168.12.3",
        avatar: "avatar11",
        msgCount: 5
    },
    {
        ipaddr: "192.168.12.4",
        nickname: "192.168.12.4",
        avatar: "avatar04",
        msgCount: 0
    },
    {
        ipaddr: "192.168.12.5",
        nickname: "192.168.12.5",
        avatar: "avatar05",
        msgCount: 0
    },

]);

const currentUser = ref<User>({
    ipaddr: "",
    nickname: "",
    avatar: "",
    msgCount: 0
});

// 处理点击，设置点击效果
function handleItemClicked(ipaddr: string) {
    userList.value.map(function (user) {
        if(user.ipaddr === ipaddr){
            currentUser.value = user;
            user.isChecked  = true;
        }else{
            user.isChecked = false;
        }
    });
}

</script>
<template>
    <el-container class="max-box">
        <el-aside width="60px" class="side-line">
            <Profile />
        </el-aside>
        <el-aside width="200px" class="side-line">
            <IconItem v-for="item in userList" v-bind="item" @item-clieck="handleItemClicked" />
        </el-aside>
        <el-main style="margin: 0;padding: 0;">
            <ChatForm :nickname="currentUser.nickname" />
        </el-main>
    </el-container>
</template>
<style scoped>
.side-line {
    border-right: 1px solid #eee;
    display: flex;
    flex-direction: column;

}

.max-box {
    width: 100%;
    height: 100%;
}

.profile-box {
    width: 60px;
    height: 100%;
}

.item-box {
    height: 100%;
}
</style>