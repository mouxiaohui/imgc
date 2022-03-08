<script setup lang="ts">
import { ref } from "@vue/reactivity";
import { invoke } from "@tauri-apps/api/tauri";
import Color from "color";
import { writeText } from "@tauri-apps/api/clipboard";

interface RefPalette {
  // imageBase64
  imgSrc: string;
  extractedColors: Array<Colors>;
  // Array<hexColor>
  pickerColor: Array<string>;
}

interface Colors {
  rgb: string;
  hex: string;
}

const refPalette = ref<RefPalette[]>([]);
const colorType = ref<string>("HEX");
const colorHex = ref<string>("");
const colorValue = ref<string>("");
const showCopy = ref<boolean>(true);
const loading = ref(false);

// 处理上传图片
async function handleUpload(event: any) {
  const files = event.target.files;

  for (let i = 0; i < files.length; i++) {
    loading.value = true;
    let reader = new FileReader();
    reader.readAsDataURL(files[i]);
    reader.onload = async function (e) {
      let imageBase64 = e.target?.result?.toString();
      if (imageBase64) {
        await addPalette(imageBase64);
        if (i == files.length - 1) {
          loading.value = false;
        }
      }
    };
  }
}

// 调用Rust函数，处理图片并分析颜色
async function addPalette(imageBase64: string) {
  try {
    let extractedColors = (await invoke("extraction_color", {
      // 去除base64图片前缀: data:image/jpg;base64,
      imageBase64: imageBase64.substring(imageBase64.indexOf(",") + 1)
    })) as Colors[];

    refPalette.value.push({
      imgSrc: imageBase64,
      extractedColors,
      pickerColor: []
    });
  } catch (error) {
    console.log(error);
  }
}

function handleSelectColor(colors: Colors) {
  colorHex.value = colors.hex;
  changeColorType();
  // 写入剪切板
  writeText(colorValue.value);
}

function changeColorType() {
  if (colorHex.value === "") return false;
  const color = Color(colorHex.value);
  if (colorType.value === "HEX") {
    colorValue.value = colorHex.value;
  }
  if (colorType.value === "HSL") {
    colorValue.value = color.hsl().string();
  }
  if (colorType.value === "RGB") {
    colorValue.value = color.rgb().string();
  }
  if (colorType.value === "CMYK") {
    colorValue.value = color.cmyk().round().array().toString();
  }
  if (colorType.value === "HWB") {
    colorValue.value = color.hwb().string();
  }
}

function handleCopy() {
  if (colorValue.value !== "") {
    writeText(colorValue.value);
    showCopy.value = false;
    setTimeout(() => {
      showCopy.value = true;
    }, 450);
  }
}

async function handlePicker(pickerColor: string[]) {
  // @ts-ignore
  const eyeDropper = new EyeDropper();
  const result = await eyeDropper.open();
  pickerColor.push(result.sRGBHex);
}

function handleClear(pickerColor: string[]) {
  pickerColor.length = 0;
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
    <div class="select">
      <div class="circle-block" :style="`background-color: ${colorHex};`"></div>
      <div class="input"><input maxlength="50" type="text" v-model="colorValue" /></div>
      <div class="copy" @click="handleCopy">
        <img v-show="showCopy" src="./assets/image/copy.svg" />
        <img v-show="!showCopy" src="./assets/image/ok.svg" />
      </div>
      <select class="select-type" v-model="colorType" @change="changeColorType">
        <option value="RGB" label="RGB"></option>
        <option value="HEX" label="HEX"></option>
        <option value="HSL" label="HSL"></option>
        <option value="HWB" label="HWB"></option>
        <option value="CMYK" label="CMYK"></option>
      </select>
    </div>
  </div>
  <div
    class="body"
    v-loading.fullscreen.lock="loading"
    element-loading-text="解析中..."
    element-loading-background="rgba(0, 0, 0, 0.8)"
  >
    <ul>
      <li v-for="(item, index) in refPalette" :key="index">
        <div class="palette">
          <div class="img"><img :src="item.imgSrc" /></div>
          <div class="colors">
            <div class="extracted-color">
              <div
                class="color-block"
                title="单击显示并复制颜色值颜色值"
                v-for="(colors, j) in item.extractedColors"
                :key="j"
                :style="`background-color: ${colors.hex};`"
                @click="handleSelectColor(colors)"
              ></div>
            </div>
            <div class="picker-color">
              <div class="left-icon">
                <img @click="handlePicker(item.pickerColor)" src="./assets/image/color-picker.svg" alt="" />
                <img @click="handleClear(item.pickerColor)" src="./assets/image/clear.svg" alt="" />
              </div>
              <div class="picker-items">{{ item.pickerColor }}</div>
            </div>
          </div>
        </div>
      </li>
    </ul>
  </div>
</template>

<style lang="scss">
.header {
  @apply flex w-full mb-2 select-none px-4 justify-between;

  .upload {
    @apply flex w-24 justify-center border-2 rounded-xl;
    flex-shrink: 0;
    border-color: #6e6e6e;

    label {
      @apply cursor-pointer;

      img {
        @apply w-16 h-14;
      }
    }
  }

  .select {
    @apply flex items-center;

    .circle-block {
      @apply rounded-full w-8 h-8 mx-2;
    }

    .input {
      @apply border border-white;

      input {
        @apply border-none outline-none text-white bg-black;
        text-indent: 4px;
        width: 210px;
        height: 30px;
      }
    }

    .copy {
      @apply flex justify-center items-center cursor-pointer;
      height: 30px;
      width: 30px;

      img {
        width: 30px;
        height: 30px;
      }
    }

    .select-type {
      @apply outline-none border-none;
      font-size: 18px;
      text-indent: 4px;
      width: 80px;
      height: 30px;
      background: black;
    }
  }
}

.body {
  @apply overflow-auto px-4;
  height: calc(100% - 148px);

  .palette {
    @apply mt-4 flex;
    height: 164px;

    .img {
      @apply border-2 border-white select-none;
      flex-shrink: 0;

      img {
        width: 160px;
        height: 160px;
      }
    }

    .colors {
      height: 164px;
      width: calc(100% - 160px);

      .extracted-color {
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

      .picker-color {
        @apply pl-4 pt-4 flex;
        height: 124px;

        .left-icon {
          @apply flex flex-col justify-between py-4 border border-white;
          flex-shrink: 0;
          background-color: #202020;

          img {
            @apply cursor-pointer;
            width: 30px;
          }

          img:hover {
            background-color: #474747;
          }
        }

        .picker-items {
        }
      }
    }
  }
}

*::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

*::-webkit-scrollbar-track {
  background-color: #292929;
  -webkit-border-radius: 2em;
  -moz-border-radius: 2em;
  border-radius: 2em;
}
*::-webkit-scrollbar-thumb {
  background-color: #5f5f5f;
  -webkit-border-radius: 2em;
  -moz-border-radius: 2em;
  border-radius: 2em;
}
</style>
