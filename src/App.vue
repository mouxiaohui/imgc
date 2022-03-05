<script setup lang="ts">
import { ref } from "@vue/reactivity";
import { invoke } from "@tauri-apps/api/tauri";

interface RefPalette {
  imgSrc: string;
  palettes: Array<Palette>;
}

interface Palette {
  rgb: string;
  hex: string;
}

let refPalette = ref<RefPalette[]>([]);
let refRgb = ref<string>();
let refHex = ref<string>();

// 处理上传图片
async function handleUpload(event: any) {
  const files = event.target.files;

  for (let i = 0; i < files.length; i++) {
    let reader = new FileReader();
    reader.readAsDataURL(files[i]);
    reader.onload = function (e) {
      let imageBase64 = e.target?.result?.toString();
      if (imageBase64) {
        addPalette(imageBase64);
      }
    };
  }
}

async function addPalette(imageBase64: string) {
  let palettes = (await invoke("get_palettes", {
    imageBase64: imageBase64.substring(imageBase64.indexOf(",") + 1)
  })) as Palette[];
  refPalette.value.push({
    imgSrc: imageBase64,
    palettes
  });
  console.log(palettes);
}

function handleSelectColor(color: Palette) {
  refRgb.value = `Rgb(${color.rgb[0]}, ${color.rgb[1]}, ${color.rgb[2]})`;
  refHex.value = color.hex;
}
</script>

<template>
  <div class="header">
    <div class="upload">
      <label class="cursor-pointer">
        <img src="./assets/image/upload.svg" />
        <div>上传图片</div>
        <input
          class="hidden w-0 h-0"
          multiple="true"
          type="file"
          accept="image/jpg, image/png, image/jpeg"
          @change="handleUpload"
        />
      </label>
    </div>
  </div>
  <div class="body">
    <ul>
      <li v-for="(item, index) in refPalette" :key="index">
        <div class="palette">
          <div class="img"><img :src="item.imgSrc" /></div>
          <div class="colors">
            <div class="wraper">
              <div
                class="color-block"
                v-for="(color, j) in item.palettes"
                :key="j"
                :style="`background-color: ${color.hex};`"
                @click="handleSelectColor(color)"
              ></div>
            </div>
          </div>
        </div>
      </li>
    </ul>
  </div>
  <div class="footer">
    <div :style="`border-top: 3px solid ${refHex};`">
      <div>{{ refRgb }}</div>
      <div>{{ refHex }}</div>
    </div>
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
  height: calc(100% - 148px);

  .palette {
    @apply h-40 mt-4 flex;

    .img {
      img {
        @apply h-40 w-40;
      }
    }

    .colors {
      height: 160px;
      width: calc(100% - 160px);

      .wraper {
        @apply flex justify-center;

        .color-block {
          @apply cursor-pointer;
          width: 40px;
          height: 40px;
        }

        .color-block:hover {
          box-shadow: 0 0 8px rgb(255, 255, 255);
        }

        .color-block:last-child {
          border-radius: 0 6px 6px 0;
        }

        .color-block:first-child {
          border-radius: 6px 0 0 6px;
        }
      }
    }
  }
}

.footer {
  @apply h-14 text-center flex justify-center flex-col;
  background-color: rgb(15, 15, 15);
}

*::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

*::-webkit-scrollbar-track {
  background-color: #5f5f5f;
  -webkit-border-radius: 2em;
  -moz-border-radius: 2em;
  border-radius: 2em;
}
*::-webkit-scrollbar-thumb {
  background-color: #292929;
  -webkit-border-radius: 2em;
  -moz-border-radius: 2em;
  border-radius: 2em;
}
</style>
