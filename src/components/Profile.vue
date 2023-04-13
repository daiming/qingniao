<!-- 界面左侧导航栏，用于放置用户头像、系统设置等 -->
<template>
    <div class="p-max-box">
        <img class="icon-img" src="/avatars/avatar01.svg" />
        
    </div>
</template>

<style scoped>
    .p-max-box {
        width: 60px;
        height: 100%;
        background-color: bisque;
        padding-top: 20px;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: start;
    }


.p-max-box .icon-img {
    height: 28px;
    width: 28px;

}
</style>