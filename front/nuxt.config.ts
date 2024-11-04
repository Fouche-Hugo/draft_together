// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: "2024-04-03",
  devtools: { enabled: true },

  typescript: {
    typeCheck: true,
  },

  modules: ["@nuxt/eslint", "@nuxtjs/tailwindcss"],

  runtimeConfig: {
    public: { wsBaseAddress: "ws://localhost:3636/ws/", httpBaseAddress: 'http://localhost:3636/' },
  },
});
