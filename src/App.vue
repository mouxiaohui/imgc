<script setup lang="ts"></script>

<template>
  <div class="upload">
    <label>
      <input class="hidden" type="file" />
      <div>上传图片</div>
      <img src="./assets/image/upload.svg" />
    </label>
  </div>
</template>

<style lang="scss">
.upload {
  @apply flex w-28 border border-white justify-center;

  img {
    @apply w-10 pl-3;
  }
}
</style>
