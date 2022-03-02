<script setup lang="ts">
import { ref } from "@vue/reactivity";

interface Palette {
  imgSrc: string;
  colors: string[];
}

let palettes = ref<Palette[]>([]);

// 处理上传图片
function handleUpload(event: any) {
  const files = event.target.files;

  for (let i = 0; i < files.length; i++) {
    let reader = new FileReader();
    reader.readAsDataURL(files[i]);
    reader.onload = function (e) {
      let src = e.target?.result?.toString();
      if (src) {
        palettes.value.push({
          imgSrc: src,
          colors: ["color1", "color2"]
        });
      }
    };
  }
}
</script>

<template>
  <div class="header">
    <div class="upload">
      <label class="cursor-pointer">
        <img src="./assets/image/upload.svg" />
        <div>上传图片</div>
        <input class="hidden w-0 h-0" multiple="true" type="file" accept="image/*" @change="handleUpload" />
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
