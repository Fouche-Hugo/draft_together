<script setup lang="ts">
import { POSITION, useToast } from "vue-toastification";
import { toBlob } from "html-to-image";
import CopyImage from "./icons/CopyImage.vue";

interface Props {
  htmlElementId: string;
}

const props = defineProps<Props>();

const toast = useToast();

const toastOptions = {
  position: POSITION.BOTTOM_RIGHT,
  timeout: 3000,
};

async function copyImg() {
  const draftNode = document.getElementById(props.htmlElementId)!;
  try {
    const blob = await toBlob(draftNode);
    if (blob) {
      await navigator.clipboard.write([
        new ClipboardItem({
          "image/png": blob,
        }),
      ]);
      toast.success("Draft image has been copied to clipboard!", toastOptions);
    }
  } catch ($e) {
    toast.error(
      `Failed to copy the draft image to clipboard: ${$e}`,
      toastOptions,
    );
  }
}
</script>

<template>
  <button
    aria-label="copy image button"
    class="h-min rounded-lg bg-zinc-700 px-2 py-1 text-center text-sm font-medium hover:bg-zinc-600 focus:outline-none focus-visible:ring-2 focus-visible:ring-zinc-300 sm:px-3 sm:py-2"
    title="Copy Draft as Image"
    @click="copyImg()"
  >
    <CopyImage />
  </button>
</template>
