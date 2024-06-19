import { createRouter, createWebHistory } from "vue-router";
import Application from "@/views/Application.vue";

export default createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/:chapters*",
      component: Application,
    },
    {
      path: "/resource_manager",
      component: () => import("@/views/ResourceManager.vue"),
    },
  ],
});
