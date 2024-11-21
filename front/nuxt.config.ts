// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: "2024-04-03",
  devtools: { enabled: true },

  typescript: {
    typeCheck: true,
  },

  modules: ["@nuxt/eslint", "@nuxtjs/tailwindcss", "@nuxt/image"],

  runtimeConfig: {
    public: {
      wsBaseAddress: "ws://localhost:3636/ws/",
      httpBaseAddress: "http://app:3000/",
    },
  },

  postcss: {
    plugins: {
      autoprefixer: {},
      cssnano: {},
    },
  },

  app: {
    head: {
      title: "Draft Together",
      link: [{ rel: "icon", type: "image/png", href: "/logo.png" }],
    },
  },

  build: {
    transpile: ["vue-toastification"],
  },

  image: {
    domains: process.env.NUXT_IMAGE_DOMAIN
      ? process.env.NUXT_IMAGE_DOMAINS?.split(",")
      : [],
  },
});
