<script setup lang="ts">
import { fa } from "element-plus/es/locale";
import IconItem from "./IconItem.vue";
import Profile from "./Profile.vue";
import ChatForm from "./ChatForm.vue";
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

// 用户列表
interface User {
    ipaddr: string,
    nickname: string,
    avatar?: string,
    msg_count?: number,
    is_checked?: boolean
}

const userList = ref<User[]>([
]);

onMounted(() => {
    getUserList();
});

const currentUser = ref<User>({
    ipaddr: "",
    nickname: "",
    avatar: "",
    msg_count: 0
});

// 处理点击，设置点击效果
function handleItemClicked(ipaddr: string) {
    userList.value.map(function (user) {
        if(user.ipaddr === ipaddr){
            currentUser.value = user;
            user.is_checked  = true;
        }else{
            user.is_checked = false;
        }
    });
}

async function getUserList() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  userList.value = await invoke("get_user_list");
  console.log("接收到的参数：",userList.value);
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