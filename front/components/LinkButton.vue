<script setup lang="ts">
import CopyLink from "./icons/CopyLink.vue";
import { POSITION, useToast } from "vue-toastification";

interface Props {
  link: string;
}

const props = defineProps<Props>();

const toast = useToast();

const toastOptions = {
  position: POSITION.BOTTOM_RIGHT,
  timeout: 3000,
};

async function copyURL() {
  try {
    await navigator.clipboard.writeText(props.link);
    toast.success("Link has been copied to clipboard!", toastOptions);
  } catch ($e) {
    toast.error(`Failed to copy the link to clipboard: ${$e}`, toastOptions);
  }
}
</script>

<template>
  <button
    class="h-min rounded-lg bg-zinc-700 p-1 px-3 py-2 text-center text-sm font-medium hover:bg-zinc-600 focus:outline-none focus-visible:ring-2 focus-visible:ring-zinc-300"
    title="Copy Draft Link"
    @click="copyURL()"
  >
    <CopyLink />
  </button>
</template>
