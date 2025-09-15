import axios from "axios";

const api = axios.create({
  baseURL: "http://localhost:7878/api/v1",
  withCredentials: true, // <--- 这行是关键，允许跨域发送cookie
  headers: {
    "Content-Type": "application/json",
  },
});

// 我们不再需要手动设置 Authorization 头，可以移除请求拦截器

export default api;
