// // import '@fullcalendar/core/vdom'

// // import '@fullcalendar/common/main.min.css';
// import "flatpickr/dist/flatpickr.min.css";

// import "dropzone/dist/dropzone.css";

// import "font-awesome/css/font-awesome.css";

// import "bootstrap-icons/font/bootstrap-icons.css";

// import "../public/scss/style.scss";

// // @ts-ignore
// import init from "website-admin";

// init().then((exports) => {
//   exports.run_app(0);
// });

import { createApp } from "vue";
import router from "./router"; // ++ 将上一步骤配置 router导入
import App from "./App.vue";

import init from "frontend-wasm";
await init();

const app = createApp(App);
app.config.globalProperties.$base_url = "http://localhost:8100/api/v1/admin";

app
    .use(router) // ++ 挂载路由
    .mount("#app");
