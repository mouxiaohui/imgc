<script setup lang="ts">
import { ref } from "@vue/reactivity";
import { readBinaryFile, readTextFile } from "@tauri-apps/api/fs";
import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";

interface Palette {
  imgSrc: string | undefined;
  colors: string[];
}

let palettes = ref<Palette[]>([]);

// 处理上传图片
async function handleUpload() {
  let filesPath = await open({
    title: "选择图片",
    multiple: true,
    filters: [
      {
        name: "image",
        extensions: ["svg", "png", "jpg", "jpeg", "gif"]
      }
    ]
  });

  if (filesPath instanceof Array) {
    for (const key in filesPath) {
      getColor(filesPath[key]);
    }
  }
}

function getColor(path: string) {
  invoke("get_image_color", { path: path });
}
</script>

<template>
  <div class="header">
    <div class="upload" @click="handleUpload">
      <label class="cursor-pointer">
        <img src="./assets/image/upload.svg" />
        <div>上传图片</div>
      </label>
    </div>
  </div>
  <div class="body">
    <ul>
      <li v-for="(item, index) in palettes" :key="index">
        <div class="palette">
          <img :src="item.imgSrc" />
          <div class="colors"></div>
        </div>
      </li>
    </ul>
  </div>
</template>

<style lang="scss">
.header {
  @apply flex w-full justify-center mb-2 select-none;

  .upload {
    @apply flex w-28 justify-center border-2 rounded-xl;
    border-color: #9b4ee4;

    label {
      @apply cursor-pointer;

      img {
        @apply w-16 h-14;
      }
    }
  }
}

.body {
  @apply overflow-auto;
  height: calc(100% - 92px);

  .palette {
    @apply h-40 mt-4 flex;

    img {
      @apply h-40;
    }
  }
}

.body::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

.body::-webkit-scrollbar-track {
  background-color: #353535;
  -webkit-border-radius: 2em;
  -moz-border-radius: 2em;
  border-radius: 2em;
}
.body::-webkit-scrollbar-thumb {
  background-color: #9b4ee4;
  -webkit-border-radius: 2em;
  -moz-border-radius: 2em;
  border-radius: 2em;
}
</style>
